#[derive(Debug, serde::Deserialize)]
pub struct Config {
    #[serde(default = "default_log_level")]
    pub log_level: log::LevelFilter,
    
    pub app: App,
}

fn default_log_level() -> log::LevelFilter { log::LevelFilter::Warn }


#[derive(Debug, serde::Deserialize)]
pub struct App {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_shards")]
    pub shards: u8,
    #[serde(default = "default_pattern")]
    pub file_pattern: String,
    #[serde(default = "default_path")]
    pub file_path: String,
}

fn default_port() -> u16 { 3000 }
fn default_shards() -> u8 { 3 }
fn default_pattern() -> String { "shard_{}.log".to_string() }
fn default_path() -> String { "/tmp".to_string() }


#[cfg(test)]
mod serde_tests {
    use log::LevelFilter;

    use super::*;

    #[test]
    fn deserialize_full_yaml() {
        let yaml = r#"
            log_level: warn
            app:
                port: 3000
                shards: 4
                file_pattern: "file_{}.log"
                file_path: "/tmp"
        "#;

        let cfg:Config = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(cfg.log_level, LevelFilter::Warn);
        assert_eq!(cfg.app.port, 3000);
        assert_eq!(cfg.app.shards, 4);
        assert_eq!(cfg.app.file_pattern, "file_{}.log".to_string());
        assert_eq!(cfg.app.file_path, "/tmp".to_string());
    }

    #[test]
    fn default_are_applied() {
        let yaml = r#"
        app:
            port: 3000
            shards: 4
        "#;

        let cfg:Config = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(cfg.log_level, LevelFilter::Warn);

        assert_eq!(cfg.app.port, 3000);
        assert_eq!(cfg.app.shards, 4);
        assert_eq!(cfg.app.file_pattern, "shard_{}.log".to_string());
        assert_eq!(cfg.app.file_path, "/tmp".to_string());
    }
}
