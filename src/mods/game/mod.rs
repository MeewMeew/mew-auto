use std::os::windows::process::CommandExt;

use super::utils::{
  constants::{AUTO_GAME_MODE, AUTO_PAUSE_PROCESS, CREATE_NO_WINDOW},
  store::{read_from_registry, write_to_registry},
};
use anyhow::{Ok, Result};

pub fn get_auto_game_mode() -> Result<bool> {
  let auto = read_from_registry(AUTO_GAME_MODE, "0").unwrap();
  Ok(if auto == "1" { true } else { false })
}

pub fn toggle_auto_game_mode() -> Result<()> {
  let auto = get_auto_game_mode().unwrap();
  write_to_registry(AUTO_GAME_MODE, if auto { "0" } else { "1" }).unwrap();
  Ok(())
}

pub fn get_auto_pause_process() -> Result<bool> {
  let auto = read_from_registry(AUTO_PAUSE_PROCESS, "0").unwrap();
  Ok(if auto == "1" { true } else { false })
}

pub fn set_auto_pause_process(value: bool) -> Result<()> {
  write_to_registry(AUTO_PAUSE_PROCESS, if value { "1" } else { "0" }).unwrap();
  Ok(())
}

pub fn get_pid_by_name(name: &str) -> Result<u32> {
  let output = std::process::Command::new("cmd")
    .creation_flags(CREATE_NO_WINDOW)
    .args(&[
      "/C",
      "tasklist",
      "/NH",
      "/FI",
      format!("WINDOWTITLE eq {}*", name).as_str(),
    ])
    .output()
    .unwrap();
  let result = String::from_utf8_lossy(&output.stdout);
  if result.contains("No tasks are running") {
    return Ok(0);
  }
  return Ok(
    result.split_whitespace().collect::<Vec<&str>>()[1]
      .parse::<u32>()
      .unwrap(),
  );
}
