use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

use catch_cli::git_info::{get_git_remote_url, parse_github_url};
#[test]
fn test_get_git_remote_url() {
    let dir = tempdir().unwrap();
    let git_dir = dir.path().join(".git");
    std::fs::create_dir(&git_dir).unwrap();

    let config_path = git_dir.join("config");
    let mut config_file = File::create(&config_path).unwrap();
    writeln!(config_file, "[core]").unwrap();
    writeln!(config_file, "\trepositoryformatversion = 0").unwrap();
    writeln!(config_file, "\tfilemode = false").unwrap();
    writeln!(config_file, "\tbare = false").unwrap();
    writeln!(config_file, "[remote \"origin\"]").unwrap();
    writeln!(config_file, "\turl = https://github.com/user/repo.git").unwrap();
    writeln!(config_file, "\tfetch = +refs/heads/*:refs/remotes/origin/*").unwrap();

    std::env::set_current_dir(&dir).unwrap();

    let result = get_git_remote_url();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "https://github.com/user/repo.git");
}

#[test]
fn test_get_git_remote_url_no_git_dir() {
    let dir = tempdir().unwrap();
    std::env::set_current_dir(&dir).unwrap();

    let result = get_git_remote_url();
    assert!(result.is_err());
}

#[test]
fn test_parse_github_url_ssh() {
    let url = "git@github.com:user/repo.git";
    let result = parse_github_url(url);
    assert!(result.is_ok());
    let (org, repo) = result.unwrap();
    assert_eq!(org, "user");
    assert_eq!(repo, "repo");
}

#[test]
fn test_parse_github_url_https() {
    let url = "https://github.com/user/repo.git";
    let result = parse_github_url(url);
    assert!(result.is_ok());
    let (org, repo) = result.unwrap();
    assert_eq!(org, "user");
    assert_eq!(repo, "repo");
}

#[test]
fn test_parse_github_url_git() {
    let url = "git://github.com/user/repo.git";
    let result = parse_github_url(url);
    assert!(result.is_ok());
    let (org, repo) = result.unwrap();
    assert_eq!(org, "user");
    assert_eq!(repo, "repo");
}

#[test]
fn test_parse_github_url_no_git_extension() {
    let url = "https://github.com/user/repo";
    let result = parse_github_url(url);
    assert!(result.is_ok());
    let (org, repo) = result.unwrap();
    assert_eq!(org, "user");
    assert_eq!(repo, "repo");
}
