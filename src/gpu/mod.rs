pub mod utils;

pub mod status;
mod temps;

use serde::Serialize;
use status::{GPUStatus, GPUStatusReader};
use std::{fs, path::PathBuf, process::Command};

pub struct GPUPath {
    pub file_path: PathBuf,
    pub hwmon_path: PathBuf,
}

impl GPUPath {
    pub fn new(file_path: &PathBuf) -> Self {
        let hwmon_path = file_path
            .join("hwmon")
            .join(GPUPath::find_hwmon_path(file_path).unwrap());

        GPUPath {
            file_path: file_path.clone(),
            hwmon_path,
        }
    }

    pub fn new_with_hwmon_path(file_path: &PathBuf, hwmon_path: &PathBuf) -> Self {
        GPUPath {
            file_path: file_path.clone(),
            hwmon_path: hwmon_path.clone(),
        }
    }

    pub fn find_hwmon_path(file_path: &PathBuf) -> Option<String> {
        let file_path = file_path.join("hwmon");
        if let Ok(entries) = fs::read_dir(file_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    return Some(entry.file_name().to_str().unwrap().to_string());
                }
            }
        }
        None
    }
}

#[derive(Serialize, Clone)]
pub struct GPU {
    pub name: String,
    pub file_path: PathBuf,

    #[serde(skip_serializing)]
    status_reader: GPUStatusReader,
}

impl GPU {
    pub fn new(gpu_path: &GPUPath) -> Self {
        GPU {
            name: GPU::get_card_name(&gpu_path.file_path),
            file_path: gpu_path.file_path.clone(),
            status_reader: GPUStatusReader::new(&gpu_path.hwmon_path),
        }
    }

    pub fn new_with_name(gpu_path: &GPUPath, name: &str) -> Self {
        GPU {
            name: name.to_string(),
            file_path: gpu_path.file_path.clone(),
            status_reader: GPUStatusReader::new(&gpu_path.hwmon_path),
        }
    }

    fn get_card_name(file_path: &PathBuf) -> String {
        let command = format!(
            "udevadm info -q property -p {} | grep ID_MODEL_FROM_DATABASE | cut -d '=' -f2 ",
            file_path.to_str().unwrap()
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
