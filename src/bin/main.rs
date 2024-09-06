use catch_cli::code_reader::find_and_read_files;
use catch_cli::git_info;
use catch_cli::ongoing_session::active_session_checker::{handle_sessions, CatchSessionError};
use catch_cli::ongoing_session::session_connector::connect_cli_to_session;
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
    let cli_connect_result =
        match connect_cli_to_session(active_session_id, org_name, repo_name).await {
            Ok(response) => response,
            Err(e) => {
                error!("Failed to connect CLI to session: {}", e);
                exit(-4);
            }
        };

    info!(":✅ Connected CLI to session: {:?}", cli_connect_result);

    let current_dir = std::env::current_dir()?;
    let pre_target_files = find_and_read_files(&current_dir, &cli_connect_result.public_key).await?;

    for file in pre_target_files {
        info!(
            ":📄 Found supported file: {:?}",
            file.encrypted_file_content
        );
    }

    Ok(())
}
