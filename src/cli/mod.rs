use clap::Clap;

use crate::gpu::utils::get_system_gpus;

#[derive(Clap)]
#[clap(version = "1.0", author = "José M. Sarasola <alosarjos@gmail.com>")]
pub struct Config {
    /// GPU card ID from /sys/class/drm/
    #[clap(short, long, default_value = " ")]
    pub gpu_card_sys_path: String,
    /// Listening port for the API Rest server
    #[clap(short, long, default_value = "8080")]
    pub server_port: u16,
}

pub fn get_run_config() -> Config {
    let mut config: Config = Config::parse();

    if config.gpu_card_sys_path.trim().is_empty() {
        println!("No GPU path was provided, attempting to find GPU...");
        if let Some(system_gpus) = get_system_gpus() {
            let first_detected_gpu = system_gpus.first().unwrap();
            println!(
                "Found GPU: {} at {}",
                first_detected_gpu.name,
                first_detected_gpu.file_path.to_str().unwrap()
            );
            config.gpu_card_sys_path = first_detected_gpu.file_path.to_str().unwrap().to_string();
        } else {
            panic!("No GPU was found on the system");
        }
    }
    config
}
