#![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mods;

use mods::{
  process::get_processes_by_name,
  threads::{bun, game, tray},
  trayicon::{init_tray_icon, Events},
  utils::{constants::APP_VERSION, msgbox::error_msg_box},
};

use anyhow::Result;
use std::mem::MaybeUninit;
use windows::Win32::{
  Foundation::TRUE,
  UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, TranslateMessage},
};

fn main() -> Result<()> {
  pre_init().unwrap();
  init_events().unwrap();

  loop {
    unsafe {
      let mut msg = MaybeUninit::uninit();
      let bret = GetMessageW(msg.as_mut_ptr(), None, 0, 0);
      if bret == TRUE {
        let _ = TranslateMessage(msg.as_ptr());
        DispatchMessageW(msg.as_ptr());
      } else {
        break;
      }
    }
  }

  Ok(())
}

fn pre_init() -> Result<()> {
  println!("{}", format!("  + Running MewAuto v{}", APP_VERSION));

  let processes = get_processes_by_name("MewAuto").unwrap();
  processes.iter().for_each(|p| {
    if processes.len() > 1 {
      println!("  + Found process: {}", p);
    }
  });
  if processes.len() > 1 {
    error_msg_box(
      "Error",
      "Another instance is already running\nPlease close it first",
    );
    std::process::exit(1);
  }

  Ok(())
}

fn init_events() -> Result<()> {
  let (sender, receiver) = std::sync::mpsc::channel::<Events>();
  let tray_icon = init_tray_icon(sender.clone());

  let _ = std::thread::Builder::new()
    .name("Bun_Thread".to_string())
    .spawn(bun::thread);

  let _ = std::thread::Builder::new()
    .name("Game_Thread".to_string())
    .spawn(game::thread);

  let _ = std::thread::Builder::new()
    .name("Tray_Thread".to_string())
    .spawn(move || tray::thread(receiver, tray_icon.unwrap()));

  Ok(())
}
