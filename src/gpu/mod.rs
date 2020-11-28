mod status;
mod temps;

use status::Status;
use std::{error::Error, path::Path, process::Command};

#[derive(Debug)]
pub struct GPU {
    pub name: String,
    file_path: String,
}

impl GPU {
    /// Instantiates a new GPU with the GPU card ID
    pub fn new(card_id: u32) -> Result<Self, Box<dyn Error>> {
        let file_path = GPU::get_card_file_path(card_id);

        assert!(
            Path::new(&file_path).exists(),
            "Could not find the file path: {}",
            file_path
        );

        Ok(GPU {
            name: GPU::get_gpu_name(),
            file_path: file_path.clone(),
        })
    }

    fn get_card_file_path(card_id: u32) -> String {
        format!("/sys/class/drm/card{}/device/", card_id)
    }

    fn get_gpu_name() -> String {
        // TODO: Need a better way to get the GPU Name based on the device ID
        // For now, limit to the output of glxinfo as it seems to be the more precise name

        let command = Command::new("sh")
            .arg("-c")
            .arg("glxinfo -B | grep Device")
            .env("LANG", "en")
            .output()
            .expect("Could not launch glxinfo command");

        assert!(
            command.status.success(),
            "Could not execute glxinfo command. {}",
            String::from_utf8(command.stderr).unwrap()
        );

        let output =
            String::from_utf8(command.stdout).expect("Could not convert the command to String");

        // TODO: Find a better way to get the GPU name from the substring, but since source
        // may change, it has low priority
        output[output.find("Device: ").unwrap() + 8..output.find(" (").unwrap()].to_string()
    }

    pub fn get_status(&self) -> Result<Status, Box<dyn Error>> {
        Status::read_status(&format!("{}/hwmon/hwmon2/", self.file_path))
    }
}
