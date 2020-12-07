pub mod utils;

pub mod status;
mod temps;

use serde::Serialize;
use status::{GPUStatus, GPUStatusReader};
use std::process::Command;

pub struct GPUPath {
    pub file_path: String,
    pub hwmon_path: String,
}

impl GPUPath {
    pub fn new(file_path: &str, hwmon_path: Option<&str>) -> Self {
        let hwmon_path = match hwmon_path {
            Some(hwmon_path) => hwmon_path.to_string(),
            None => format!("{}hwmon/hwmon2/", file_path),
        };

        GPUPath {
            file_path: file_path.to_string(),
            hwmon_path,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct GPU {
    pub name: String,
    pub file_path: String,

    #[serde(skip_serializing)]
    status_reader: GPUStatusReader,
}

impl GPU {
    pub fn new(name: Option<&str>, gpu_path: GPUPath) -> Self {
        let name = match name {
            Some(name) => name.to_string(),
            None => GPU::get_card_name(&gpu_path.file_path),
        };

        GPU {
            name: name.to_string(),
            file_path: gpu_path.file_path,
            status_reader: GPUStatusReader::new(&gpu_path.hwmon_path),
        }
    }

    fn get_card_name(file_path: &str) -> String {
        let command = format!(
            "udevadm info -q property -p {} | grep ID_MODEL_FROM_DATABASE | cut -d '=' -f2 ",
            file_path
        );

        let command = Command::new("sh")
            .arg("-c")
            .arg(command)
            .env("LANG", "en")
            .output()
            .expect("Could not launch udevadm command");

        String::from_utf8(command.stdout)
            .unwrap()
            .trim()
            .to_string()
    }

    pub fn get_status(&self) -> GPUStatus {
        self.status_reader.read_status()
    }
}
