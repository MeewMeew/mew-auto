use std::{
  sync::{Arc, Mutex},
  thread::JoinHandle,
};

use crate::mods::voice::{self, get_listen_hey_pc};
use anyhow::Result;

pub fn thread() -> Result<()> {
  println!("  + Running Voice Thread");
  let handle: Arc<Mutex<Option<JoinHandle<Result<()>>>>> = Arc::new(Mutex::new(None));

  loop {
    match get_listen_hey_pc() {
      Ok(true) => {
        if handle.lock().unwrap().is_none() {
          let h = std::thread::spawn(|| voice::process_input());
          *handle.lock().unwrap() = Some(h);
        }
      }
      Ok(false) => {
        if let Some(h) = handle.lock().unwrap().take() {
          let _ = h.join();
        }
      }
      Err(e) => {
        eprintln!("Error checking voice input: {}", e);
        // Consider adding a short sleep here to avoid busy-waiting
      }
    }
  }
}
