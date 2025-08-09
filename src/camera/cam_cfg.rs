use anyhow::{Context, Result};
use config::{Config, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub device: Vec<String>,
}

fn decode_dev_number(path: &str) -> Option<i32> {
    let pos = match path.rfind(|c: char| !c.is_ascii_digit()) {
        Some(t) => t,
        None => {
            return None;
        }
    };
    path[pos + 1..].parse().ok()
}

impl CameraConfig {
    /// Reads the configuration file and returns a new `CameraConfig`.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path_ref = path.as_ref();
        if !path_ref.exists() {
            return Err(anyhow::anyhow!(
                "Configuration file not found: {}",
                path_ref.display()
            ));
        }

        let settings = Config::builder()
            .add_source(File::with_name(path_ref.to_str().unwrap()))
            .build()
            .with_context(|| format!("Failed to load configuration from {}", path_ref.display()))?;

        let config: Self = settings
            .try_deserialize()
            .context("Failed to deserialize CameraConfig")?;
        Ok(config)
    }

    pub fn get_video_device_list(self) -> Result<Vec<i32>> {
        let res: Vec<i32> = self
            .device
            .iter()
            .map(|hey| {
                let no_way = decode_dev_number(hey.as_str());
                no_way.map_or(0, |f| f)
            })
            .collect();
        Ok(res)
    }
}
