use std::fmt;

#[derive(Debug)]
pub struct Browser {
    pub name: BrowserName,
    pub family: BrowserFamily,
    pub config_path: &'static str,
    pub profiles_file: &'static str,
    pub history_file: &'static str,
    pub cache_path: &'static str,
    pub cache_entries_path: &'static str,
}

#[derive(Debug, Clone)]
pub enum BrowserName {
    Firefox,
    LibreWolf,
    Chrome,
    Chromium,
}

impl fmt::Display for BrowserName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BrowserName::Firefox => write!(f, "Firefox"),
            BrowserName::LibreWolf => write!(f, "Librewolf"),
            BrowserName::Chrome => write!(f, "Chrome"),
            BrowserName::Chromium => write!(f, "Chromium"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BrowserFamily {
    Gecko,
    Chromium,
}

#[allow(dead_code)]
pub static SUPPORTED_BROWSERS: &[Browser] = &[
    Browser {
        name: BrowserName::Firefox,
        family: BrowserFamily::Gecko,
        config_path: ".mozilla/firefox",
        profiles_file: "profiles.ini",
        history_file: "places.sqlite",
        cache_path: ".cache/firefox",
        cache_entries_path: "cache2/entries",
    },
    Browser {
        name: BrowserName::LibreWolf,
        family: BrowserFamily::Gecko,
        config_path: ".config/librewolf/librewolf",
        profiles_file: "profiles.ini",
        history_file: "places.sqlite",
        cache_path: ".cache/librewolf",
        cache_entries_path: "cache2/entries",
    },
    Browser {
        name: BrowserName::Chrome,
        family: BrowserFamily::Chromium,
        config_path: ".config/google-chrome",
        profiles_file: "Local State",
        history_file: "History",
        cache_path: ".cache/google-chrome",
        cache_entries_path: "cache2/entries", //in older ver Cache/
    },
    Browser {
        name: BrowserName::Chromium,
        family: BrowserFamily::Chromium,
        config_path: ".config/chromium",
        profiles_file: "Local State",
        history_file: "History",
        cache_path: ".cache/chromium",
        cache_entries_path: "cache2/entries", //see chrome's one for info
    },
];
