use std::time::Duration;

use anyhow::Result;

use crate::mods::{bun, game::get_auto_pause_process};

pub fn thread() -> Result<()> {
  println!("  + Running Bun Thread");
  loop {
    if get_auto_pause_process().unwrap() {
      println!("  + Auto-pause process is enabled");
      std::thread::sleep(Duration::from_secs(10));
      continue;
    }
    let auto_update = bun::get_auto_update().unwrap();
    if auto_update {
      let is_latest = bun::check_for_updates().unwrap();
      if is_latest {
        let _ = bun::install_update().unwrap();
      }
    }
    std::thread::sleep(Duration::from_secs(300));
  }
}
