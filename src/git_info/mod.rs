mod ui;

use crate::git_info::ui::prompt_git_info_form;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_repo_info() -> io::Result<(String, String)> {
    if Path::new(".git").exists() {
        match get_git_remote_url() {
            Ok(url) => {
                let (org, repo) =
                    parse_github_url(&url).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                prompt_git_info_form(&org, &repo)
            }
            Err(_) => prompt_git_info_form("", ""),
        }
    } else {
        prompt_git_info_form("", "")
    }
}

fn get_git_remote_url() -> io::Result<String> {
    let file = File::open(".git/config")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let origin_start = lines
        .iter()
        .position(|line| line.trim() == "[remote \"origin\"]");

    if let Some(start) = origin_start {
        for line in &lines[start + 1..] {
            if line.trim().starts_with('[') {
                break;
            }
            if line.trim().starts_with("url = ") {
                return Ok(line.trim_start_matches("url = ").to_string());
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Remote URL not found in .git/config",
    ))
}

fn parse_github_url(url: &str) -> Result<(String, String), String> {
    let url = url.trim().strip_prefix("url = ").unwrap_or(url).trim();
    let url = url
        .strip_prefix("git@github.com:")
        .or_else(|| url.strip_prefix("https://github.com/"))
        .or_else(|| url.strip_prefix("git://github.com/"))
        .unwrap_or(url);

    let url = url.strip_suffix(".git").unwrap_or(url);
    let parts: Vec<&str> = url.split('/').collect();

    if parts.len() >= 2 {
        Ok((parts[0].to_string(), parts[1].to_string()))
    } else {
        Err(format!("Unable to parse GitHub URL: {}", url))
    }
}
