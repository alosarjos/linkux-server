use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "José M. Sarasola <alosarjos@gmail.com>")]
pub struct Config {
    /// GPU card ID from /sys/class/drm/
    #[clap(short, long, default_value = "0")]
    pub gpu_card_id: u32,
    /// Listening port for the API Rest server
    #[clap(short, long, default_value = "8080")]
    pub server_port: u32,
}

pub fn get_run_config() -> Config {
    Config::parse()
}
