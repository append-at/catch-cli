use flume::{Receiver, Sender};
use log::error;
use once_cell::sync::Lazy;
use std::process::exit;
use std::{io, panic};

mod api_client;
mod git_info;

pub static SIGNALING_STOP: Lazy<(Sender<()>, Receiver<()>)> = Lazy::new(flume::unbounded);

fn shutdown() {
    for _ in 0..1000 {
        SIGNALING_STOP.0.send(()).unwrap();
    }
    exit(0);
}

#[tokio::main]
async fn main() -> io::Result<()> {
    panic::set_hook(Box::new(|e| {
        println!("{e}");
        error!("{e}");
        shutdown();
    }));

    let (org_name, repo_name) = git_info::get_repo_info()?;
    println!("Organization Name: {}", org_name);
    println!("Repository Name: {}", repo_name);

    Ok(())
}
