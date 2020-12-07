pub mod cli;
pub mod gpu;
pub mod server;

use actix_rt;
use std::error::Error;

use cli::Config;
use server::Server;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    actix_rt::System::new("Server thread").block_on(async move {
        let server = Server::new(config);
        server.run().await.unwrap();
    });

    Ok(())
}
