use windows::{
  core::HSTRING,
  Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
      MessageBoxW, MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MB_SYSTEMMODAL,
    },
  },
};

pub fn error_msg_box(title: &str, msg: &str) {
  unsafe {
    MessageBoxW(
      HWND::default(),
      &HSTRING::from(msg),
      &HSTRING::from(title),
      MB_SYSTEMMODAL | MB_ICONERROR | MB_OK,
    );
  }
}

pub fn info_msg_box(title: &str, msg: &str) {
  unsafe {
    MessageBoxW(
      HWND::default(),
      &HSTRING::from(msg),
      &HSTRING::from(title),
      MB_SYSTEMMODAL | MB_ICONINFORMATION | MB_OK,
    );
  }
}
