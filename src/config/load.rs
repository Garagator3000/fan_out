use std::{fs,error};

use super::Config;

pub fn load(path: &str) -> Result<Config, Box<dyn error::Error>> {
    let raw = fs::read_to_string(path)?;
    let cfg: Config = serde_yaml::from_str(&raw)?;

    cfg.validate()?;

    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile;

    use super::*;

    #[test]
    fn load_valid_file() {
        let content = r#"
            log_level: info
            app:
                port: 5000
                shards: 2
                file_pattern: shard_{}.log
                file_path: /tmp
        "#;

        let mut file = tempfile::NamedTempFile::new().unwrap();

        write!(file, "{}", content).unwrap();

        let cfg: Config = load(file.path().to_str().unwrap()).unwrap();

        assert_eq!(cfg.log_level, log::LevelFilter::Info);
        assert_eq!(cfg.app.port, 5000);
        assert_eq!(cfg.app.shards, 2);
        assert_eq!(cfg.app.file_pattern, "shard_{}.log".to_string());
        assert_eq!(cfg.app.file_path, "/tmp".to_string());
    }

    #[test]
    fn load_invalid_file() {
        let content = r#"
            port: 5000
            shards: 2
            file_pattern: shard.log
            file_path: /tmp
        "#;

        let mut file = tempfile::NamedTempFile::new().unwrap();

        write!(file, "{}", content).unwrap();

        let path = file.path().to_str().unwrap();

        assert!(load(path).is_err());
    }
}
