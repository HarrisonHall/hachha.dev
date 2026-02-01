//! Synchronous blocking cache

use super::*;

/// State for cached item.
#[derive(Debug, PartialEq)]
pub enum CachedItemState {
    /// Item is not in cache
    Missing,
    /// Item is currently in cache and hasn't reached timeout
    Active,
    /// Item is in cache, but is expired
    Expired,
}

/// Entry inside of cache.
struct CacheEntry<T> {
    entry: T,
    update_time: Instant,
    timeout_override: Option<f32>,
}

impl<T> CacheEntry<T> {
    /// Generate new cache entry.
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

    /// Update entry.
    fn update(&mut self, new_entry: T) -> () {
        self.entry = new_entry;
        self.update_time = Instant::now();
    }
}

/// Cache.
pub struct Cache<T> {
    /// Statefull entries
    entries: RwLock<HashMap<String, CacheEntry<T>>>,
    /// Time until an entry expires (in seconds)
    timeout: f32,
}

#[allow(unused)]
impl<T: Clone> Cache<T> {
    /// Generate new cache.
    pub fn new(timeout: f32) -> Self {
        Cache {
            entries: RwLock::new(HashMap::new()),
            timeout,
        }
    }

    /// Get state of item in the cache.
    pub async fn get_state(&self, name: &str) -> CachedItemState {
        let entries = self.entries.read().await;
        match entries.get(name) {
            Some(entry) => match entry.is_expired(self.timeout) {
                true => CachedItemState::Expired,
                false => CachedItemState::Active,
            },
            None => CachedItemState::Missing,
        }
    }

    /// Check if item is in the cache.
    pub async fn in_cache(&self, name: &str) -> bool {
        return self.get_state(name).await == CachedItemState::Active;
    }

    /// Get item from the cache.
    pub async fn retrieve(&self, name: &str) -> Result<T, CachedItemState> {
        let entries = self.entries.read().await;
        match entries.get(name) {
            Some(entry) => match entry.is_expired(self.timeout) {
                true => Err(CachedItemState::Expired),
                false => Ok(entry.entry.clone()),
            },
            None => Err(CachedItemState::Missing),
        }
    }

    /// Get item from the cache, ignoring the expiry.
    pub async fn retrieve_force(&self, name: &str) -> Option<T> {
        let entries = self.entries.read().await;
        match entries.get(name) {
            Some(entry) => Some(entry.entry.clone()),
            None => None,
        }
    }

    /// Insert/update item into the cache.
    pub async fn update(&self, name: &str, item: T) -> T {
        self.update_override(name, item, None).await
    }

    /// Update item in the cache, overriding the previous item according to a custom
    /// timeout.
    pub async fn update_override(&self, name: &str, item: T, custom_timeout: Option<f32>) -> T {
        tracing::debug!("Updating cached value for {name}");
        let mut entries = self.entries.write().await;
        match entries.get_mut(name) {
            Some(entry) => entry.update(item.clone()),
            None => {
                let mut entry = CacheEntry::new(item.clone());
                entry.timeout_override = custom_timeout;
                entries.insert(name.to_string(), entry);
            }
        };
        item
    }
}

#[cfg(test)]
mod tests {
    use crate::cache::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn general_cacheing() {
        let cache: Cache<i8> = Cache::new(f32::INFINITY);
        cache.update("foo", 8).await;
        cache.update("bar", -16).await;

        let foo_result = cache.retrieve("foo").await;
        assert!(foo_result.is_ok());
        assert_eq!(cache.get_state("foo").await, CachedItemState::Active);

        assert!(cache.in_cache("bar").await);
        assert_eq!(cache.retrieve_force("bar").await, Some(-16));

        let baz_result = cache.retrieve("baz").await;
        assert_eq!(baz_result, Err(CachedItemState::Missing));
        assert_eq!(cache.retrieve_force("baz").await, None);
    }
}
