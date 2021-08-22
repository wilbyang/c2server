mod pb;

use anyhow::Result;
use pb::{admin_server::*, command_control_server::*, *};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
const CHANNEL_SIZE: usize = 8;
#[tokio::main]
async fn main() -> Result<()> {
    let addr = "0.0.0.0:8888";
    let soket_addr = addr.parse().unwrap();
    println!("Listening on {:?}", soket_addr);
    // grpc -> PoW
    let (tx1, rx1) = mpsc::channel(CHANNEL_SIZE);

    // PoW -> grpc
    let (tx2, rx2) = mpsc::channel(CHANNEL_SIZE);

    Server::builder()
        .add_service(CommandControlServer::new(CommandControlService::new(
            tx1, rx2,
        )))
        .add_service(AdminServer::new(AdminService::new(tx2, rx1)))
        .serve(soket_addr)
        .await?;

    Ok(())
}
#[derive(Debug)]
struct CommandControlService {
    tx: mpsc::Sender<Command>,
    rx: mpsc::Receiver<Command>,
}
struct AdminService {
    tx: mpsc::Sender<Command>,
    rx: mpsc::Receiver<Command>,
}
impl AdminService {
    fn new(tx: mpsc::Sender<Command>, mut rx: mpsc::Receiver<Command>) -> Self {
        Self { tx, rx }
    }
}

impl CommandControlService {
    fn new(tx: mpsc::Sender<Command>, mut rx: mpsc::Receiver<Command>) -> Self {
        Self { tx, rx }
    }
}
#[tonic::async_trait]
impl Admin for AdminService {
    async fn run_command(&self, request: Request<Command>) -> Result<Response<Command>, Status> {
        let cmd = request.into_inner();
        match self.tx.send(cmd.clone()).await {
            Ok(()) => match self.rx.recv().await {
                Some(cmd) => Ok(Response::new(cmd.clone())),
                None => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}
#[tonic::async_trait]
impl CommandControl for CommandControlService {
    async fn fetch_command(&self, request: Request<Empty>) -> Result<Response<Command>, Status> {
        match self.rx.recv().await {
            Some(cmd) => Ok(Response::new(cmd.clone())),
            None => todo!(),
        }
    }

    async fn send_output(&self, request: Request<Command>) -> Result<Response<Empty>, Status> {
        let cmd = request.into_inner();
        match self.tx.send(cmd.clone()).await {
            Ok(_) => Ok(Response::new(Empty {})),
            Err(_) => todo!(),
        }
        
    }
}
