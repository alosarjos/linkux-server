use std::str::FromStr;
use std::{error::Error, fs};

use super::temps::{GPUTempReader, GPUTemps};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct GPUStatus {
    pub voltage: Option<f32>,
    pub power_consumption: Option<f32>,
    pub fan_speed: Option<i32>,
    pub temps: Option<GPUTemps>,
}

#[derive(Clone)]
pub struct GPUStatusReader {
    hwmon_path: String,
    temps_reader: GPUTempReader,
}

impl GPUStatusReader {
    pub fn new(hwmon_path: &str) -> Self {
        GPUStatusReader {
            hwmon_path: hwmon_path.to_string(),
            temps_reader: GPUTempReader::new(hwmon_path),
        }
    }

    pub fn read_status(&self) -> GPUStatus {
        GPUStatus {
            voltage: self
                .read_gpu_status_file(&format!("{}in0_input", self.hwmon_path))
                .ok(),
            power_consumption: self
                .read_gpu_status_file(&format!("{}power1_average", self.hwmon_path))
                .ok(),
            fan_speed: self
                .read_gpu_status_file(&format!("{}fan1_input", self.hwmon_path))
                .ok(),
            temps: self.temps_reader.read_temps(),
        }
    }

    fn read_gpu_status_file<T>(&self, file_path: &str) -> Result<T, Box<dyn Error>>
    where
        T: FromStr,
        <T as FromStr>::Err: Error,
        <T as FromStr>::Err: 'static,
    {
        let file_content = fs::read_to_string(file_path)?;
        let value = file_content.trim().parse::<T>()?;
        Ok(value)
    }
}
