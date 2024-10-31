/// Synchronous blocking cache.
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Instant;

/// State for cached item.
#[derive(Debug, PartialEq)]
pub enum CachedItemState {
    /// Item is not in cache.
    Missing,
    /// Item is currently in cache and hasn't reached timeout.
    Active,
    /// Item is in cache, but is expired.
    Expired,
    /// Error, something like lock poisoning occured.
    Error,
}

/// Entry inside of cache.
struct CacheEntry<T> {
    entry: T,
    update_time: Instant,
    timeout_override: Option<f32>,
}

impl<T> CacheEntry<T> {
    /// Generate new entry for cache.
    fn new(item: T) -> Self {
        CacheEntry {
            entry: item,
            update_time: Instant::now(),
            timeout_override: None,
        }
    }

    /// Check if entry is expired.
    fn is_expired(&self, timeout: f32) -> bool {
        // Always reload in debug
        if cfg!(debug_assertions) {
            return true;
        }
        // Check timeout
        let timeout = self.timeout_override.unwrap_or(timeout);
        let time_since_update: f32 = self.update_time.elapsed().as_secs_f32();
        return time_since_update > timeout;
    }

    /// Update and replace entry.
    fn update(&mut self, new_entry: T) -> () {
        self.entry = new_entry;
        self.update_time = Instant::now();
    }
}

/// Cache.
pub struct Cache<T> {
    /// Statefull entries.
    entries: RwLock<HashMap<String, CacheEntry<T>>>,
    /// Time until an entry expires (in seconds).
    timeout: f32,
}

#[allow(dead_code)]
impl<T: Clone> Cache<T> {
    /// Create a new cache.
    pub fn new(timeout: f32) -> Self {
        Cache {
            entries: RwLock::new(HashMap::new()),
            timeout,
        }
    }

    /// Get state of cache item.
    pub fn get_state(&self, name: &str) -> CachedItemState {
        if let Ok(entries) = self.entries.read() {
            return match entries.get(name) {
                Some(entry) => match entry.is_expired(self.timeout) {
                    true => CachedItemState::Expired,
                    false => CachedItemState::Active,
                },
                None => CachedItemState::Missing,
            };
        }
        return CachedItemState::Error;
    }

    /// Check if item is in the cache and not expired.
    pub fn in_cache(&self, name: &str) -> bool {
        return self.get_state(name) == CachedItemState::Active;
    }

    /// Retrieve item from the cache.
    pub fn retrieve(&self, name: &str) -> Result<T, CachedItemState> {
        if let Ok(entries) = self.entries.read() {
            return match entries.get(name) {
                Some(entry) => match entry.is_expired(self.timeout) {
                    true => Err(CachedItemState::Expired),
                    false => Ok(entry.entry.clone()),
                },
                None => Err(CachedItemState::Missing),
            };
        }
        return Err(CachedItemState::Error);
    }

    /// Retrieve item from the cache, even if expired.
    pub fn retrieve_force(&self, name: &str) -> Option<T> {
        if let Ok(entries) = self.entries.read() {
            return match entries.get(name) {
                Some(entry) => Some(entry.entry.clone()),
                None => None,
            };
        }
        return None;
    }

    /// Update item in cache.
    pub fn update(&self, name: &str, item: T) -> T {
        return self.update_override(name, item, None);
    }

    //// Update item in cache, with custom timeout.
    pub fn update_override(&self, name: &str, item: T, custom_timeout: Option<f32>) -> T {
        if let Ok(mut entries) = self.entries.write() {
            match entries.get_mut(name) {
                Some(entry) => entry.update(item.clone()),
                None => {
                    let mut entry = CacheEntry::new(item.clone());
                    entry.timeout_override = custom_timeout;
                    entries.insert(name.to_string(), entry);
                }
            };
        }
        return item; // Return original for one-line update
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::*;

    #[test]
    fn general_cacheing() {
        let cache: Cache<i8> = Cache::new(f32::INFINITY);
        cache.update("foo", 8);
        cache.update("bar", -16);

        let foo_result = cache.retrieve("foo");
        assert!(foo_result.is_ok());
        assert_eq!(cache.get_state("foo"), CachedItemState::Active);

        assert!(cache.in_cache("bar"));
        assert_eq!(cache.retrieve_force("bar"), Some(-16));

        let baz_result = cache.retrieve("baz");
        assert_eq!(baz_result, Err(CachedItemState::Missing));
        assert_eq!(cache.retrieve_force("baz"), None);
    }
}
