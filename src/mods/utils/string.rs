use anyhow::Result;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub fn wide_to_string(wide: *const u16) -> Result<String> {
  let wide_slice = unsafe {
    let mut len = 0;
    while *wide.offset(len) != 0 {
      len += 1;
    }
    std::slice::from_raw_parts(wide, len as usize)
  };
  let os_string = OsString::from_wide(wide_slice);
  Ok(os_string.to_string_lossy().into_owned())
}
