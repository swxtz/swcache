// Copyright 2026 Athenas System
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::config::config_file::ConfigManager;
use crate::protobuf::server::ping_service;
use disclaimer::{beta, logo};
use runtime::runtime::RuntimeState;
use tonic::transport::Server;
use utils::clean_terminal;

mod config;
mod disclaimer;
pub mod protobuf;
mod runtime;
mod utils;

#[tokio::main]
async fn main() {
    let runtime = RuntimeState::new();
    let state = RuntimeState::state(&runtime);

    startup().await;
    while state {}
}

async fn startup() {
    clean_terminal::clean();

    logo::show_logo();
    beta::beta_warning();

    ConfigManager::init_global("swconfig.toml").expect("Error initializing config manager");

    let is_single_threaded: bool = ConfigManager::get_as("single_thread_mode").unwrap_or(false);
    println!("Is single thread mode enabled? {}", is_single_threaded);

    let server_port: i16 = ConfigManager::get_as("port").unwrap_or(8080);
    println!("In port {}", server_port);

    tokio::spawn(async move {
        if let Err(e) = init_grpc_server(server_port).await {
            eprintln!("Erro ao iniciar servidor gRPC: {}", e);
        }
    });

    println!("gRPC server iniciado em background na porta {}", server_port);
}

async fn init_grpc_server(port: i16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", port).parse()?;

    println!("gRPC server rodando em {}", addr);

    Server::builder()
        .add_service(ping_service())
        .serve(addr)
        .await?;

    Ok(())
}

/// Verify config file with the default path our custom path, if config file not exists, the program
/// will running in default configuration
fn verify_config(path: String) -> bool {
    let config_file_exists = true;

    return if config_file_exists {
        println!("Verify config file exists");
        true
    } else {
        println!("Verify config file not exists");
        false
    };
}