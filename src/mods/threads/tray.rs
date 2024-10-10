use trayicon::TrayIcon;

use crate::mods::{
  bun, game,
  trayicon::{setup_tray_icon, Events},
  utils::msgbox::info_msg_box,
};

pub fn thread(receiver: std::sync::mpsc::Receiver<Events>, mut tray_icon: TrayIcon<Events>) {
  println!("  + Running Tray Thread");
  receiver.iter().for_each(|m| match m {
    Events::LeftClickTrayIcon => {
      let _ = tray_icon.show_menu().unwrap();
    }
    Events::DisableLabelBun => {
      let _ = setup_tray_icon(&mut tray_icon);
    }
    Events::CurrentVersion => {
      let res = bun::fetch_latest_api().unwrap();
      let _ = info_msg_box("Release description", &res.body);
      let _ = setup_tray_icon(&mut tray_icon);
    }
    Events::AutoUpdate => {
      let _ = bun::toggle_auto_update().unwrap();
      let _ = setup_tray_icon(&mut tray_icon);
    }
    Events::OpenReleaseNotes => {
      let _ = bun::open_release_notes();
      let _ = setup_tray_icon(&mut tray_icon);
    }
    Events::AutoDetectGameMode => {
      let _ = game::toggle_auto_game_mode().unwrap();
      let _ = setup_tray_icon(&mut tray_icon);
    }
    Events::Exit => {
      std::process::exit(0);
    }
  });
}
