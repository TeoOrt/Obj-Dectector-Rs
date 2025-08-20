use anyhow::{Context, Result};
use config::{Config, File};
use serde::Deserialize;
use std::path::Path;
use super::decode_dev_number;

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub device: Vec<String>,
}

// ------> Constructors
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
}


// -- Setters
impl CameraConfig{
    pub fn add_cameras(&mut self,list_of_cameras:Vec<String>) {
        for cameras in list_of_cameras{
            self.device.push(cameras);
        }
    }
    pub fn remove_cameras(&mut self,camera :String){
        self.device.retain(|dev| dev.as_str()!=camera.as_str());
    }
}

// -- Getters
impl CameraConfig{

    pub fn get_video_device_list(self) -> Result<Vec<i32>> {
        let res: Vec<i32> = self
            .device
            .iter()
            .map(|hey| {
                let dev_number = decode_dev_number(hey.as_str());
                dev_number.map_or(0, |f| f)
            })
            .collect();
        Ok(res)
    }
}
