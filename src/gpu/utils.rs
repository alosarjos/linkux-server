use crate::gpu::GPU;
use std::{error::Error, fs, path::PathBuf, process::Command};

use super::GPUPath;

pub fn get_system_gpus() -> Option<Vec<GPU>> {
    let lspci_gpu_output = get_lspci_gpus().ok()?;

    let lspci_gpus: Vec<(&str, &str)> = lspci_gpu_output
        .iter()
        .map(|value| lspci_gpu_to_tuple(value))
        .collect();

    let mut system_gpus = Vec::new();

    for (lspci_path, gpu_name) in lspci_gpus {
        if let Some(gpu_file_path) = get_gpu_path_from_lspci_path(lspci_path) {
            system_gpus.push(GPU::new(Some(gpu_name), GPUPath::new(&gpu_file_path, None)));
        }
    }

    if !system_gpus.is_empty() {
        Some(system_gpus)
    } else {
        None
    }
}

fn get_lspci_gpus() -> Result<Vec<String>, Box<dyn Error>> {
    let command = Command::new("sh")
        .arg("-c")
        .arg("lspci -P | grep VGA")
        .env("LANG", "en")
        .output()?;

    let command_ouput = String::from_utf8(command.stdout)?;

    Ok(command_ouput.trim().split('\n').map(String::from).collect())
}

fn lspci_gpu_to_tuple(lspci_gpu: &str) -> (&str, &str) {
    let values_separator = "VGA compatible controller:";
    let split_strings = lspci_gpu.split(values_separator).collect::<Vec<&str>>();

    (split_strings[0].trim(), split_strings[1].trim())
}

fn get_gpu_path_from_lspci_path(lspci_path: &str) -> Option<PathBuf> {
    let mut gpu_path = PathBuf::from("/sys/bus/pci/devices");

    for suffix in lspci_path.split('/').collect::<Vec<&str>>() {
        if let Some(next_dir) = get_dir_by_suffix(&gpu_path, suffix) {
            gpu_path = gpu_path.join(next_dir.path());
        } else {
            return None;
        }
    }
    Some(gpu_path)
}

fn get_dir_by_suffix(path: &PathBuf, suffix: &str) -> Option<fs::DirEntry> {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.file_name().to_string_lossy().ends_with(suffix) {
                    return Some(entry);
                }
            }
        }
    }
    None
}
