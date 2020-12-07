pub mod utils;

mod status;
mod temps;

use serde::Serialize;
use status::{GPUStatus, GPUStatusReader};
use std::process::Command;

#[derive(Serialize, Clone)]
pub struct GPU {
    pub name: String,
    pub file_path: String,

    #[serde(skip_serializing)]
    status_reader: GPUStatusReader,
}

impl GPU {
    pub fn new(file_path: &str, name: Option<&str>) -> Self {
        GPU {
            name: match name {
                Some(name) => name.to_string(),
                None => GPU::get_card_name(file_path),
            },
            file_path: file_path.to_string(),
            status_reader: GPUStatusReader::new(&format!("{}hwmon/hwmon2/", file_path)),
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

        String::from_utf8(command.stdout).unwrap()
    }

    pub fn get_status(&self) -> GPUStatus {
        self.status_reader.read_status()
    }
}
