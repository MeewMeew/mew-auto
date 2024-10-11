use trayicon::TrayIcon;

use crate::mods::{
  bun, game, monitor,
  trayicon::{setup_tray_icon, Events},
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
    Events::AboutVersion => {
      let _ = bun::open_release_notes();
      let _ = setup_tray_icon(&mut tray_icon);
    }
    Events::AutoUpdate => {
      let _ = bun::toggle_auto_update().unwrap();
      let _ = setup_tray_icon(&mut tray_icon);
    }
    Events::TurnOffMonitor => {
      let _ = monitor::turn_off_monitor();
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
