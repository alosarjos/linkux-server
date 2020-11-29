pub mod cli;
pub mod gpu;
pub mod server;

use actix_rt;
use std::error::Error;

use cli::Config;
use gpu::GPU;
use server::Server;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let gpu = GPU::new(config.gpu_card_id)?;
    println!("GPU: {}", gpu.name);
    let current_status = gpu.get_status()?;
    println!("{}", current_status);

    actix_rt::System::new("Server thread").block_on(async move {
        let server = Server::new(config.server_port);
        server.run().await.unwrap();
    });

    Ok(())
}
