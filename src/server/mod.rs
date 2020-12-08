use std::path::PathBuf;

use actix_web::{get, web, App, HttpResponse, HttpServer};

use crate::gpu::{status::GPUStatus, GPUPath};
use crate::{cli::Config, gpu::GPU};
use serde::Serialize;

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new(config: Config) -> Self {
        Server { config }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let gpu = GPU::new(&GPUPath::new(&PathBuf::from(
            &self.config.gpu_card_sys_path,
        )));
        HttpServer::new(move || App::new().data(gpu.clone()).service(index))
            .bind(("127.0.0.1", self.config.server_port))?
            .run()
            .await
    }
}

#[get("/")]
pub fn index(gpu: web::Data<GPU>) -> HttpResponse {
    #[derive(Serialize)]
    struct GPUInfo {
        name: String,
        status: GPUStatus,
    }

    HttpResponse::Ok().json(GPUInfo {
        name: gpu.name.clone(),
        status: gpu.get_status(),
    })
}
