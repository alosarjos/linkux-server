pub mod cli;
pub mod gpu;

use std::error::Error;

use cli::Config;
use gpu::GPU;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let gpu = GPU::new(config.gpu_card_id)?;
    println!("GPU: {:?}", gpu);

    Ok(())
}
