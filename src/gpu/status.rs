use std::{error::Error, fs};
use std::{path::PathBuf, str::FromStr};

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
    hwmon_path: PathBuf,
    temps_reader: GPUTempReader,
}

impl GPUStatusReader {
    pub fn new(hwmon_path: &PathBuf) -> Self {
        GPUStatusReader {
            hwmon_path: hwmon_path.clone(),
            temps_reader: GPUTempReader::new(&hwmon_path),
        }
    }

    pub fn read_status(&self) -> GPUStatus {
        let voltage = match self.read_gpu_status_file(&self.hwmon_path.join("in0_input")) {
            Ok(value) => Some(value),
            Err(error) => {
                eprintln!("Could not read voltage value!\n{}", error);
                None
            }
        };

        let power_consumption =
            match self.read_gpu_status_file(&&self.hwmon_path.join("power1_average")) {
                Ok(value) => Some(value),
                Err(error) => {
                    eprintln!("Could not read power consumption value!\n{}", error);
                    None
                }
            };

        let fan_speed = match self.read_gpu_status_file(&&self.hwmon_path.join("fan1_input")) {
            Ok(value) => Some(value),
            Err(error) => {
                eprintln!("Could not read fan speed value!\n{}", error);
                None
            }
        };

        GPUStatus {
            voltage,
            power_consumption,
            fan_speed,
            temps: self.temps_reader.read_temps(),
        }
    }

    fn read_gpu_status_file<T>(&self, file_path: &PathBuf) -> Result<T, Box<dyn Error>>
    where
        T: FromStr,
        <T as FromStr>::Err: Error + 'static,
    {
        let file_content = fs::read_to_string(file_path)?;
        let value = file_content.trim().parse::<T>()?;
        Ok(value)
    }
}
