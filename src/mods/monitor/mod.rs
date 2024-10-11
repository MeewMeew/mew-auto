use anyhow::{anyhow, Result};
use std::mem;
use windows::{
  core::PCWSTR,
  Win32::{
    Foundation::{BOOL, HWND, LPARAM, POINT, RECT, TRUE, WPARAM},
    Graphics::Gdi::{
      CreateDCW, EnumDisplayMonitors, EnumDisplaySettingsExW, GetDeviceCaps, GetMonitorInfoW,
      MonitorFromPoint, ReleaseDC, DEVMODEW, DEVMODE_DISPLAY_ORIENTATION, EDS_RAWMODE,
      ENUM_CURRENT_SETTINGS, HDC, HMONITOR, LOGPIXELSX, MONITORINFOEXW, MONITOR_DEFAULTTONULL,
    },
    UI::WindowsAndMessaging::{GetForegroundWindow, SendMessageW, SC_MONITORPOWER, WM_SYSCOMMAND},
  },
};

use super::utils::{hash, string};

#[derive(Debug, Clone)]
pub struct DisplayInfo {
  pub name: String,
  pub id: u32,
  pub raw_handle: HMONITOR,
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub rotation: f32,
  pub scale_factor: f32,
  pub frequency: f32,
  pub is_primary: bool,
}

impl DisplayInfo {
  fn new(h_monitor: HMONITOR, monitor_info: &MONITORINFOEXW) -> Self {
    let device = monitor_info.szDevice.as_ptr();
    let device_string = string::wide_to_string(device).unwrap();
    let rc = monitor_info.monitorInfo.rcMonitor;
    let (rotation, frequency) = get_rotation_frequency(device).unwrap_or((0.0, 0.0));

    DisplayInfo {
      id: hash::hash32(device_string.as_bytes()).unwrap(),
      name: device_string,
      raw_handle: h_monitor,
      x: rc.left,
      y: rc.top,
      width: (rc.right - rc.left) as u32,
      height: (rc.bottom - rc.top) as u32,
      rotation,
      frequency,
      scale_factor: dpi_to_scale_factor(get_dpi_for_monitor(PCWSTR(device)).unwrap_or(BASE_DPI))
        .unwrap() as f32,
      is_primary: monitor_info.monitorInfo.dwFlags == 1,
    }
  }
}

pub const BASE_DPI: u32 = 96;

fn dpi_to_scale_factor(dpi: u32) -> Result<f64> {
  Ok(dpi as f64 / BASE_DPI as f64)
}

fn get_rotation_frequency(device: *const u16) -> Result<(f32, f32)> {
  let mut dev_mode: DEVMODEW = DEVMODEW {
    dmSize: mem::size_of::<DEVMODEW>() as u16,
    ..DEVMODEW::default()
  };

  let dev_modew_ptr = <*mut _>::cast(&mut dev_mode);

  unsafe {
    EnumDisplaySettingsExW(
      PCWSTR(device),
      ENUM_CURRENT_SETTINGS,
      dev_modew_ptr,
      EDS_RAWMODE,
    )
    .ok()?;
  };

  let dm_display_orientation = unsafe { dev_mode.Anonymous1.Anonymous2.dmDisplayOrientation };

  let rotation = match dm_display_orientation {
    DEVMODE_DISPLAY_ORIENTATION(0) => 0.0,
    DEVMODE_DISPLAY_ORIENTATION(1) => 90.0,
    DEVMODE_DISPLAY_ORIENTATION(2) => 180.0,
    DEVMODE_DISPLAY_ORIENTATION(3) => 270.0,
    _ => dm_display_orientation.0 as f32,
  };

  Ok((rotation, dev_mode.dmDisplayFrequency as f32))
}

fn get_dpi_for_monitor(name: PCWSTR) -> Result<u32> {
  let hdc = unsafe { CreateDCW(name, None, None, None) };
  let dpi = unsafe { GetDeviceCaps(hdc, LOGPIXELSX) as u32 };
  if hdc != HDC::default() {
    unsafe { ReleaseDC(HWND::default(), hdc) };
  }
  Ok(dpi)
}

fn get_monitor_info_exw(h_monitor: HMONITOR) -> Result<MONITORINFOEXW> {
  let mut monitor_info_exw: MONITORINFOEXW = unsafe { mem::zeroed() };
  monitor_info_exw.monitorInfo.cbSize = mem::size_of::<MONITORINFOEXW>() as u32;
  let monitor_info_exw_ptr = <*mut _>::cast(&mut monitor_info_exw);
  unsafe { GetMonitorInfoW(h_monitor, monitor_info_exw_ptr).ok()? };
  Ok(monitor_info_exw)
}

pub fn get_all() -> Result<Vec<DisplayInfo>> {
  let mut monitors = Vec::new();
  unsafe {
    EnumDisplayMonitors(
      HDC::default(),
      None,
      Some(monitor_enum_proc),
      LPARAM(&mut monitors as *mut _ as isize),
    )
    .ok()?;
  }

  monitors
    .iter()
    .map(|&h_monitor| {
      let info = get_monitor_info_exw(h_monitor)?;
      Ok(DisplayInfo::new(h_monitor, &info))
    })
    .collect()
}

pub fn get_from_point(x: i32, y: i32) -> Result<DisplayInfo> {
  let point = POINT { x, y };
  let h_monitor = unsafe { MonitorFromPoint(point, MONITOR_DEFAULTTONULL) };
  if h_monitor.is_invalid() {
    return Err(anyhow!("Monitor is invalid"));
  }
  let monitor_info = get_monitor_info_exw(h_monitor)?;
  Ok(DisplayInfo::new(h_monitor, &monitor_info))
}

pub fn turn_off_monitor() {
  unsafe {
    SendMessageW(
      GetForegroundWindow(),
      WM_SYSCOMMAND,
      WPARAM(SC_MONITORPOWER as usize),
      LPARAM(2isize),
    )
  };
}

extern "system" fn monitor_enum_proc(
  h_monitor: HMONITOR,
  _: HDC,
  _: *mut RECT,
  state: LPARAM,
) -> BOOL {
  unsafe {
    let state = Box::leak(Box::from_raw(state.0 as *mut Vec<HMONITOR>));
    state.push(h_monitor);
    TRUE
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_all() {
    let displays = get_all().unwrap();
    displays.iter().for_each(|d| {
      println!("{:?}", d);
    });
  }

  #[test]
  fn test_get_from_point() {
    let display = get_from_point(0, 0).unwrap();
    println!("{:?}", display);
  }
}
