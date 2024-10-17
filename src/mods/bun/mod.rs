#![allow(dead_code)]
use std::os::windows::process::CommandExt;

use super::utils::{
  constants::BUN_AUTO_UPDATE,
  store::{read_from_registry, write_to_registry},
};
use crate::mods::utils::constants::CREATE_NO_WINDOW;
use anyhow::Result;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApiResponse {
  pub tag_name: String,
  pub body: String,
}

const POWERSHELL: &str = "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe";

pub fn get_current_version() -> Result<String> {
  let username = std::env::var("USERNAME").unwrap();
  let bun = format!("C:\\Users\\{}\\.bun\\bin\\bun.exe", username);
  let current_version = String::from_utf8(
    std::process::Command::new(bun)
      .creation_flags(CREATE_NO_WINDOW)
      .arg("--version")
      .output()
      .unwrap()
      .stdout,
  )
  .unwrap();
  Ok(current_version)
}

pub fn fetch_latest_api() -> Result<ApiResponse> {
  let url = "https://api.github.com/repos/oven-sh/bun/releases/latest";

  let client = reqwest::blocking::Client::new();
  let response = client
    .get(url)
    .header(USER_AGENT, "bun")
    .header("X-GitHub-Api-Version", "2022-11-28")
    .send()
    .unwrap();

  if !response.status().is_success() {
    Ok(ApiResponse {
      tag_name: "v0.0.0".to_string(),
      body: "Failed to fetch latest API".to_string(),
    })
  } else {
    let response: ApiResponse = response.json().unwrap();
    Ok(response)
  }
}

pub fn check_for_updates() -> Result<bool> {
  let response = fetch_latest_api().unwrap();

  if response.tag_name == "v0.0.0" {
    return Ok(false);
  }
  let latest_version = response.tag_name.split('v').last().unwrap();
  let current_version = get_current_version().unwrap();
  let latest = latest_version == current_version.trim();
  return Ok(if latest { false } else { true });
}

pub fn install_update() -> Result<()> {
  std::process::Command::new(POWERSHELL)
    .creation_flags(CREATE_NO_WINDOW)
    .args(&["-Command", "irm bun.sh/install.ps1 | iex"]);
  Ok(())
}

pub fn open_release_notes() -> Result<()> {
  let version = get_current_version().unwrap();
  open::that(format!("https://bun.sh/blog/bun-v{}", version).as_str()).unwrap();
  Ok(())
}

pub fn get_auto_update() -> Result<bool> {
  let auto = read_from_registry(BUN_AUTO_UPDATE, "0").unwrap();
  Ok(if auto == "1" { true } else { false })
}

pub fn toggle_auto_update() -> Result<()> {
  let auto = get_auto_update().unwrap();
  write_to_registry(BUN_AUTO_UPDATE, if auto { "0" } else { "1" }).unwrap();
  Ok(())
}
