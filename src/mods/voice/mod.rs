use super::{
  process::get_processes_by_name,
  utils::{
    constants::LISTEN_HEY_PC,
    path::get_project_root,
    store::{read_from_registry, write_to_registry},
  },
};
use crate::mods::utils::constants::CREATE_NO_WINDOW;
use anyhow::Result;
use std::{os::windows::process::CommandExt, process::Child};

pub fn get_listen_hey_pc() -> Result<bool> {
  let auto = read_from_registry(LISTEN_HEY_PC, "0").unwrap();
  Ok(if auto == "1" { true } else { false })
}

pub fn toggle_auto_listen_hey_pc() -> Result<()> {
  let auto = get_listen_hey_pc().unwrap();
  write_to_registry(LISTEN_HEY_PC, if auto { "0" } else { "1" }).unwrap();
  Ok(())
}

pub fn process_input() -> Result<()> {
  let root_path = get_project_root().unwrap();
  let voice_path = if cfg!(debug_assertions) {
    root_path.join("bin").join("voice.exe")
  } else {
    root_path.join("voice.exe")
  };
  let mut chill: Child = std::process::Command::new(voice_path)
    .creation_flags(CREATE_NO_WINDOW)
    .arg("-d")
    .spawn()
    .unwrap();

  let _ = chill.wait().unwrap();

  Ok(())
}

pub fn kill_voice() -> Result<()> {
  let processes = get_processes_by_name("voice").unwrap();
  if processes.len() > 1 {
    let _ = std::process::Command::new("taskkill")
      .args(&["/F", "/IM", "voice.exe"])
      .output()
      .unwrap();
  }
  Ok(())
}
