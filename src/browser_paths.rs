#[derive(Debug, Clone)]
pub struct Browser {
    pub name: &'static str,
    pub config_path: &'static str,
    pub profiles_file: &'static str,
    pub history_file: &'static str,
    pub cache_path: &'static str,
    pub cache_entries_path: &'static str,
}
#[allow(dead_code)]
pub static SUPPORTED_BROWSERS: &[Browser] = &[
    Browser {
        name: "firefox",
        config_path: ".mozilla/firefox",
        profiles_file: "profiles.ini",
        history_file: "places.sqlite",
        cache_path: ".cache/firefox",
        cache_entries_path: "cache2/entries",
    },
    Browser {
        name: "librewolf",
        config_path: ".config/librewolf/librewolf",
        profiles_file: "profiles.ini",
        history_file: "places.sqlite",
        cache_path: ".cache/librewolf",
        cache_entries_path: "cache2/entries",
    },
    Browser {
        name: "chrome",
        config_path: ".config/google-chrome",
        profiles_file: "Local State",
        history_file: "History",
        cache_path: ".cache/google-chrome",
        cache_entries_path: "cache2/entries", //in older ver Cache/
    },
    Browser {
        name: "chromium",
        config_path: ".config/chromium",
        profiles_file: "Local State",
        history_file: "History",
        cache_path: ".cache/chromium",
        cache_entries_path: "cache2/entries", //see chrome's one for info
    },
];
