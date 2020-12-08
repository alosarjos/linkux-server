use serde::Serialize;
use std::{error::Error, fs, path::PathBuf};

#[derive(Serialize, Clone)]
pub struct GPUTemps {
    pub edge: Option<f32>,
    pub junction: Option<f32>,
    pub memory: Option<f32>,
}

#[derive(Clone)]
pub struct GPUTempReader {
    hwmon_path: PathBuf,
}

impl GPUTempReader {
    pub fn new(hwmon_path: &PathBuf) -> Self {
        GPUTempReader {
            hwmon_path: hwmon_path.clone(),
        }
    }

    pub fn read_temps(&self) -> Option<GPUTemps> {
        let edge = GPUTempReader::read_temp_from_file(&self.hwmon_path.join("temp1_input")).ok();
        let junction =
            GPUTempReader::read_temp_from_file(&self.hwmon_path.join("temp2_input")).ok();
        let memory = GPUTempReader::read_temp_from_file(&self.hwmon_path.join("temp3_input")).ok();

        if edge.is_none() && junction.is_none() && memory.is_none() {
            return None;
        }

        Some(GPUTemps {
            edge,
            junction,
            memory,
        })
    }

    fn read_temp_from_file(file_path: &PathBuf) -> Result<f32, Box<dyn Error>> {
        let file_content = fs::read_to_string(file_path)?;
        let value = file_content.trim().parse::<f32>()? / 1000.0;
        Ok(value)
    }
}
