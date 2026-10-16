use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    name: &'static str,
    version: &'static str,
    core_version: &'static str,
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        name: "Parallaxe",
        version: env!("CARGO_PKG_VERSION"),
        core_version: parallaxe_core::VERSION,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_info_uses_camel_case() {
        let json = serde_json::to_value(app_info()).unwrap();
        assert_eq!(json["name"], "Parallaxe");
        assert!(json.get("coreVersion").is_some());
    }
}
