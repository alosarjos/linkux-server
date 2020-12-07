use actix_web::{get, web, App, HttpResponse, HttpServer};

use crate::gpu::status::GPUStatus;
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
        let binding = format!("127.0.0.1:{}", self.config.server_port);
        let gpu = GPU::new(&self.config.gpu_card_sys_path, None);
        HttpServer::new(move || App::new().data(gpu.clone()).service(index))
            .bind(binding)?
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
