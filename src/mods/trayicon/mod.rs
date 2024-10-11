use std::sync::mpsc::Sender;

use anyhow::Result;
use trayicon::{MenuBuilder, MenuItem, TrayIcon, TrayIconBuilder};

use super::{bun, game, utils::constants::APP_VERSION};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Events {
  LeftClickTrayIcon,

  DisableLabelBun,
  AboutVersion,
  AutoUpdate,

  Monitor,
  TurnOffMonitor,

  AutoDetectGameMode,

  Exit,
}

pub fn init_tray_icon(sender: Sender<Events>) -> Result<TrayIcon<Events>> {
  let mut tray_icon = TrayIconBuilder::new()
    .sender(move |e| sender.send(*e).unwrap())
    .icon_from_buffer(include_bytes!("../../../res/icon.ico"))
    .tooltip(format!("MewAuto v{}", APP_VERSION).as_str())
    .on_click(Events::LeftClickTrayIcon)
    .build()
    .unwrap();

  setup_tray_icon(&mut tray_icon).unwrap();
  Ok(tray_icon)
}

pub fn setup_tray_icon(tray_icon: &mut trayicon::TrayIcon<Events>) -> Result<()> {
  tray_icon
    .set_menu(
      &MenuBuilder::new()
        .with(MenuItem::Item {
          id: Events::DisableLabelBun,
          name: "Bun runtime".into(),
          disabled: true,
          icon: None,
        })
        .item(
          format!("About bun v{}", bun::get_current_version().unwrap()).as_str(),
          Events::AboutVersion,
        )
        .checkable(
          "Always up to date",
          bun::get_auto_update().unwrap(),
          Events::AutoUpdate,
        )
        .separator()
        .with(MenuItem::Item {
          id: Events::DisableLabelBun,
          name: "Monitor".into(),
          disabled: true,
          icon: None,
        })
        .item("Turn off monitor", Events::TurnOffMonitor)
        .separator()
        .checkable(
          "Auto-detect game mode",
          game::get_auto_game_mode().unwrap(),
          Events::AutoDetectGameMode,
        )
        .separator()
        .item("Exit", Events::Exit),
    )
    .unwrap();
  Ok(())
}
