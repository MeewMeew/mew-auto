use anyhow::Result;
use winreg::enums::*;
use winreg::RegKey;

use super::constants::APP_NAME;

pub fn write_to_registry(key: &str, value: &str) -> Result<()> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let (subkey, _): (RegKey, _) = hklm.create_subkey(format!("Software\\{}", APP_NAME))?;
  subkey.set_value(key, &value).unwrap();
  Ok(())
}

pub fn read_from_registry(key: &str, default: &str) -> Result<String> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let subkey = hklm.open_subkey(format!("Software\\{}", APP_NAME)).unwrap();

  match subkey.get_value::<String, &str>(key) {
    Ok(value) => Ok(value),
    Err(_) => {
      write_to_registry(key, default).unwrap();
      Ok(default.to_string())
    }
  }
}

pub fn delete_from_registry(key: &str) -> Result<()> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let subkey = hklm.open_subkey(format!("Software\\{}", APP_NAME))?;
  subkey.delete_value(key).unwrap();
  Ok(())
}
