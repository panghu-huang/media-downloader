use config::{Config, ConfigError, Environment as ConfigEnvironment, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Environment {
  Local,
  Test,
  Production,
}

#[derive(Deserialize, Default, Clone)]
pub struct AppConfiguration {
  #[serde(rename = "services-addr")]
  pub services_addr: String,
  #[serde(rename = "gateway-addr")]
  pub gateway_addr: String,
}

#[derive(Deserialize, Clone)]
pub struct UnifiedItemConfig {
  pub base_url: String,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
  pub url: String,
}

// UserConfiguration is the configuration structure defined by the user.
#[derive(Deserialize)]
pub struct UserConfiguration {
  pub app: AppConfiguration,
  pub database: DatabaseConfig,
  #[serde(rename = "unified-channels")]
  pub unified_channels: HashMap<String, UnifiedItemConfig>,
  pub default_channel: String,
}

// Configuration is a structure composed of user configuration and environment configuration.
#[derive(Clone)]
pub struct Configuration {
  pub environment: Environment,
  pub app: AppConfiguration,
  pub database: DatabaseConfig,
  pub unified_channels: HashMap<String, UnifiedItemConfig>,
  pub default_channel: String,
}

impl Configuration {
  pub fn new() -> anyhow::Result<Self> {
    let env: Environment = env::var("APP_ENV")
      .unwrap_or_else(|_| "local".to_string())
      .try_into()?;

    Self::new_with_env(env)
  }

  pub fn new_with_env(env: impl Into<Environment>) -> anyhow::Result<Self> {
    let env = env.into();

    let config_dir = if cfg!(debug_assertions) {
      let workspace_root = workspace_root_dir()?;
      // If the application is running in debug mode, use the configuration directory in the workspace root.
      workspace_root.join("config")
    } else {
      env::current_dir()?.join("config")
    };

    let config = Config::builder()
      // Load the base configuration file
      .add_source(File::from(config_dir.join("base.toml")))
      // Load the environment-specific configuration file
      .add_source(File::from(config_dir.join(format!("{}.toml", env))))
      .add_source(ConfigEnvironment::default());

    let config = config.build()?;

    let user_config: UserConfiguration = config.try_deserialize()?;

    Ok(Self {
      // Return the environment
      environment: env,
      app: user_config.app,
      database: user_config.database,
      unified_channels: user_config.unified_channels,
      default_channel: user_config.default_channel,
    })
  }
}

fn workspace_root_dir() -> anyhow::Result<PathBuf> {
  let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
  let cargo_manifest_dir = PathBuf::from(cargo_manifest_dir);

  let mut root_dir = cargo_manifest_dir;

  while !root_dir.join("crates").exists() {
    root_dir = root_dir
      .parent()
      .ok_or_else(|| anyhow::anyhow!("Failed to find workspace root dir"))?
      .to_path_buf();
  }

  Ok(root_dir)
}

impl TryFrom<String> for Environment {
  type Error = ConfigError;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "local" => Ok(Self::Local),
      "production" => Ok(Self::Production),
      "test" => Ok(Self::Test),
      _ => Err(ConfigError::Message("Invalid environment".to_string())),
    }
  }
}

impl std::fmt::Display for Environment {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Local => write!(f, "local"),
      Self::Test => write!(f, "test"),
      Self::Production => write!(f, "production"),
    }
  }
}
