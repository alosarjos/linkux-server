use serde::Serialize;
use std::{error::Error, fs};

#[derive(Serialize, Clone)]
pub struct GPUTemps {
    pub edge: Option<f32>,
    pub junction: Option<f32>,
    pub memory: Option<f32>,
}

#[derive(Clone)]
pub struct GPUTempReader {
    hwmon_path: String,
}

impl GPUTempReader {
    pub fn new(hwmon_path: &str) -> Self {
        GPUTempReader {
            hwmon_path: hwmon_path.to_string(),
        }
    }

    pub fn read_temps(&self) -> Option<GPUTemps> {
        let edge =
            GPUTempReader::read_temp_from_file(&format!("{}temp1_input", self.hwmon_path)).ok();
        let junction =
            GPUTempReader::read_temp_from_file(&format!("{}temp2_input", self.hwmon_path)).ok();
        let memory =
            GPUTempReader::read_temp_from_file(&format!("{}temp3_input", self.hwmon_path)).ok();

        if edge.is_none() && junction.is_none() && memory.is_none() {
            return None;
        }

        Some(GPUTemps {
            edge,
            junction,
            memory,
        })
    }

    fn read_temp_from_file(file_path: &str) -> Result<f32, Box<dyn Error>> {
        let file_content = fs::read_to_string(file_path)?;
        let value = file_content.trim().parse::<f32>()? / 1000.0;
        Ok(value)
    }
}
