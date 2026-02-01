//! Site theme.

use super::*;

/// Embedded theme data.
#[derive(RustEmbed)]
#[folder = "content/styles/theme"]
struct EmbeddedThemes;

pub struct ThemeProvider {
    default_theme: EmbeddedData,
    theme_rules: ThemeRules,
}

impl ThemeProvider {
    pub fn new() -> Result<Self> {
        let theme_rules = util::read_embedded_toml::<ThemeRules, EmbeddedThemes>("rules.toml")?;
        let default_theme =
            match util::read_embedded_data::<EmbeddedThemes>(theme_rules.default.as_str()) {
                Ok(d) => d.clone(),
                Err(e) => {
                    bail!(
                        "Unable to parse default theme '{}': {}",
                        theme_rules.default,
                        e
                    );
                }
            };
        Ok(Self {
            theme_rules,
            default_theme,
        })
    }

    /// Fetch the correct theme, depending on the rules.
    fn get_theme(&self) -> EmbeddedData {
        let mut theme_name = self.theme_rules.default.clone();
        let date = chrono::Utc::now().date_naive();

        for rule in self.theme_rules.rules.iter() {
            if rule.matches(&date) {
                theme_name = rule.theme.clone();
                break;
            }
        }

        match util::read_embedded_data::<EmbeddedThemes>(theme_name.as_str()) {
            Ok(d) => d,
            Err(e) => {
                tracing::error!("Unable to fetch theme '{}': {e}.", theme_name);
                self.default_theme.clone()
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ThemeRules {
    default: Arc<String>,
    rules: Vec<ThemeRule>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ThemeRule {
    theme: Arc<String>,
    dates: Vec<ThemeDate>,
}

impl ThemeRule {
    fn matches(&self, date: &chrono::NaiveDate) -> bool {
        let date = ThemeDate::from(date);
        for td in self.dates.iter() {
            if td.matches(&date) {
                return true;
            }
        }
        false
    }
}

#[derive(Clone, Copy, Debug)]
enum ThemeDate {
    Date {
        month: u8,
        day: u8,
    },
    Range {
        month_a: u8,
        day_a: u8,
        month_b: u8,
        day_b: u8,
    },
}

impl ThemeDate {
    fn matches(&self, other: &Self) -> bool {
        match other {
            ThemeDate::Date { month, day } => match self {
                ThemeDate::Date {
                    month: month_o,
                    day: day_o,
                } => {
                    return month == month_o && day == day_o;
                }
                ThemeDate::Range {
                    month_a,
                    day_a,
                    month_b,
                    day_b,
                } => {
                    if month > month_a && month < month_b {
                        return true;
                    }
                    if month == month_a && month_a == month_b {
                        if day >= day_a && day <= day_b {
                            return true;
                        }
                    } else {
                        if month == month_a && day >= day_a {
                            return true;
                        }
                        if month == month_b && day <= day_b {
                            return true;
                        }
                    }
                    return false;
                }
            },
            ThemeDate::Range { .. } => {
                tracing::warn!("Cannot compare ThemeDate ranges.");
                false
            }
        }
    }
}

impl Serialize for ThemeDate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ThemeDate::Date { month, day } => {
                serializer.serialize_str(&format!("{month:02}/{day:02}"))
            }
            ThemeDate::Range {
                month_a,
                day_a,
                month_b,
                day_b,
            } => serializer
                .serialize_str(&format!("{month_a:02}/{day_a:02}-{month_b:02}/{day_b:02}")),
        }
    }
}

impl<'de> Deserialize<'de> for ThemeDate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let binding = String::deserialize(deserializer)?;
        let text = binding.clone();

        let mut day_a = None;
        let mut day_b = None;
        for (i, date) in text.split("-").enumerate() {
            if i >= 2 {
                return Err(D::Error::unknown_variant(
                    text.as_str(),
                    &["date in format mm/dd-mm/dd"],
                ));
            }

            let mut month = 0u8;
            let mut day = 0u8;
            for part in date.split("/") {
                let val: u8 = match part.parse() {
                    Ok(val) => val,
                    Err(_) => return Err(D::Error::unknown_variant(part, &["valid numeral"])),
                };
                if month == 0 {
                    month = val;
                } else {
                    day = val;
                }
            }

            if month == 0 || month > 12 || day == 0 || day > 31 {
                return Err(D::Error::unknown_variant(date, &["valid month and day"]));
            }

            if day_a.is_none() {
                day_a = Some((month, day));
            } else {
                day_b = Some((month, day));
            }
        }

        match (day_a, day_b) {
            (None, None) => Err(D::Error::unknown_variant(
                text.as_str(),
                &["date in format mm/dd-mm/dd"],
            )),
            (Some(d), None) => Ok(ThemeDate::Date {
                month: d.0,
                day: d.1,
            }),
            (None, Some(d)) => Ok(ThemeDate::Date {
                month: d.0,
                day: d.1,
            }),
            (Some(d1), Some(d2)) => Ok(ThemeDate::Range {
                month_a: d1.0,
                day_a: d1.1,
                month_b: d2.0,
                day_b: d2.1,
            }),
        }
    }
}

impl From<&chrono::NaiveDate> for ThemeDate {
    fn from(value: &chrono::NaiveDate) -> Self {
        Self::Date {
            month: value.month() as u8,
            day: value.day() as u8,
        }
    }
}

/// Get theme, matching rules.
pub async fn get_theme(State(site): State<Site>) -> impl axum::response::IntoResponse {
    adjust_content_header("theme.css", site.theme_provider().get_theme())
}
