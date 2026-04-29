use thiserror;

use super::config::*;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid port: {0}")]
    InvalidPort(u16),
    #[error("invalid shards: {0}")]
    InvalidShards(u8),
    #[error("invalid file pattern")]
    InvalidPattern,
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        self.app.validate()
    }
}

impl App {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.port == 0 {
            return Err(ConfigError::InvalidPort(self.port));
        }

        if self.shards <= 0 {
            return Err(ConfigError::InvalidShards(self.shards));
        }

        if !self.file_pattern.contains("{}") {
            return Err(ConfigError::InvalidPattern);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config_ok() {
        let cfg = Config {
            log_level: log::LevelFilter::Warn,
            app: App {
                port: 3000,
                shards: 3,
                file_pattern: "file_{}.log".to_string(),
                file_path: "/tmp/dir".to_string(),
            },
        };

        assert!(cfg.validate().is_ok());
    }

    #[test]
    fn cfg_zero_port() {
        let cfg = Config {
            log_level: log::LevelFilter::Warn,
            app: App {    
                port: 0,
                shards: 3,
                file_pattern: "file_{}.log".to_string(),
                file_path: "/tmp/dir".to_string(),
            },
        };

        assert!(matches!(cfg.validate(), Err(ConfigError::InvalidPort(0))));
    }

    #[test]
    fn reject_zero_shards() {
        let cfg = Config {
            log_level: log::LevelFilter::Warn,
            app: App {
                port: 3000,
                shards: 0,
                file_pattern: "file_{}.log".to_string(),
                file_path: "/tmp/dir".to_string(),
            },
        };

        assert!(matches!(cfg.validate(), Err(ConfigError::InvalidShards(0))));
    }


    #[test]
    fn reject_bad_pattern() {
        let cfg = Config {
            log_level: log::LevelFilter::Warn,
            app: App {
                port: 3000,
                shards: 3,
                file_pattern: "bad_pattern.log".to_string(),
                file_path: "/tmp/dir".to_string(),
            },
        };

        assert!(matches!(cfg.validate(), Err(ConfigError::InvalidPattern)));
    }
}
