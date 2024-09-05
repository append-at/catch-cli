use catch_cli::git_info;
use catch_cli::ongoing_session::{handle_sessions, CatchSessionError};
use flume::{Receiver, Sender};
use log::{error, info};
use once_cell::sync::Lazy;
use std::process::exit;
use std::{io, panic};

pub static SIGNALING_STOP: Lazy<(Sender<()>, Receiver<()>)> = Lazy::new(flume::unbounded);

fn shutdown() {
    for _ in 0..1000 {
        SIGNALING_STOP.0.send(()).unwrap();
    }
    exit(0);
}

#[tokio::main]
async fn main() -> io::Result<()> {
    handsome_logger::init().unwrap();

    panic::set_hook(Box::new(|e| {
        println!("{e}");
        error!("{e}");
        shutdown();
    }));

    // find ongoing session
    let temp_path = std::env::temp_dir();

    println!("temp_path: {:?}", temp_path);

    let active_session_id = match handle_sessions(&temp_path) {
        Ok(session_id) => {
            info!("Found catch session: {}", session_id);
            session_id
        }
        Err(CatchSessionError::NoSessionFound) => {
            exit(-1);
        }
        Err(CatchSessionError::MultipleSessionsFound) => {
            exit(-2);
        }
        Err(CatchSessionError::IoError(e)) => {
            error!("An IO error occurred: {}", e);
            exit(-3)
        }
    };

    let (org_name, repo_name) = git_info::get_repo_info()?;
    info!(
        "Setting repo key to session {}: {}/{}",
        active_session_id, org_name, repo_name
    );

    Ok(())
}
