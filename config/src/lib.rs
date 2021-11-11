use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    ambient_declarations: bool,
    out_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            ambient_declarations: false,
            out_dir: "typescript".to_owned(),
        }
    }
}

static CONFIG_INSTANCE: OnceCell<Arc<Config>> = OnceCell::new();

impl Config {
    const FILE_NAME: &'static str = "ts.toml";

    pub fn get() -> Result<Arc<Self>> {
        match CONFIG_INSTANCE.get() {
            None => {
                let cfg = Arc::new(Self::load()?);
                CONFIG_INSTANCE.set(cfg.clone()).ok();
                Ok(cfg)
            }
            Some(cfg) => Ok(cfg.clone()),
        }
    }

    fn load() -> Result<Self> {
        let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);
        let config = Self::try_load_from_dir(&manifest_dir)?.unwrap_or_default();
        Ok(config)
    }

    fn try_load_from_dir(dir: &Path) -> Result<Option<Self>> {
        let path = {
            let mut path = PathBuf::from(dir);
            path.push(Self::FILE_NAME);
            path
        };
        match path.is_file() {
            true => {
                let content = std::fs::read_to_string(path)?;
                let parsed = toml::from_str::<Config>(&content)?;
                Ok(Some(parsed))
            }
            false => Ok(None),
        }
    }
}
