use std::net::SocketAddr;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Unexpected config error [{0}]: {1}")]
    UnexpectedConfigError(String, String),
    #[error("Invalid address: {0}")]
    InvalidAddress(String),
    #[error("Invalid port: {0}")]
    InvalidPort(String),
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub address: SocketAddr,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            address: "127.0.0.1:8000".parse::<SocketAddr>().unwrap(),
            username: None,
            password: None,
        }
    }
}

impl TryFrom<&str> for Config {
    type Error = ConfigError;

    fn try_from(data: &str) -> Result<Config, ConfigError> {
        data.split('\n')
            .filter_map(|f| match f.trim() {
                f if f.is_empty() || f.starts_with('[') || f.starts_with('#') => None,
                f => Some(f),
            })
            .try_fold(Config::default(), {
                |mut config, line| {
                    let (key, value) = line.split_once('=').ok_or_else(|| {
                        ConfigError::UnexpectedConfigError(line.into(), String::new())
                    })?;

                    match key.trim() {
                        "address" => {
                            config.address = value
                                .replace("\"", "")
                                .trim()
                                .parse::<SocketAddr>()
                                .map_err(|e| ConfigError::InvalidAddress(format!("{value}: {e}")))?
                        }
                        "username" => todo!("stop-point"),
                        "password" => todo!("stop-point"),
                        _ => eprintln!("unknown key: {key}"),
                    }

                    Ok(config)
                }
            })
    }
}

#[cfg(test)]
mod test {
    use crate::{mock_file, parser::Config};

    use rstest::rstest;

    #[rstest]
    #[case(
        mock_file!("[server]", r#"address = "127.0.0.1:9000""#, "", "[credentials]", r#"username="test""#, r#"password="test""#),
        Config { address: "127.0.0.1:9000".parse().unwrap(), username: Some("test".to_string()), password: Some("test".to_string()) }
    )]
    #[case(
        mock_file!("[server]", "address = \"127.0.0.1:9090\""),
        Config { address: "127.0.0.1:9090".parse().unwrap(), username: None, password: None }
    )]
    fn should_parse_toml_file(#[case] config: &str, #[case] expected: Config) {
        let config = Config::try_from(config).expect("should return valid config");

        debug_assert_eq!(config, expected);
    }
}
