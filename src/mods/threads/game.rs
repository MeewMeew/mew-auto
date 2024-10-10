use anyhow::Result;
use std::{collections::HashMap, time::Duration};

use crate::mods::game::{get_auto_game_mode, get_pid_by_name, set_auto_pause_process};

pub fn thread() -> Result<()> {
  println!("  + Running Game Thread");

  let list_games = vec!["VALORANT", "League of Legends", "Minecraft"];
  let mut games: HashMap<String, bool> = HashMap::new();

  for game in &list_games {
    games.insert(game.to_string(), false);
  }

  loop {
    if get_auto_game_mode().unwrap() {
      let game_keys: Vec<String> = games.keys().cloned().collect();

      for game in &game_keys {
        let game_running = get_pid_by_name(game).unwrap() > 0;
        let game_playing = *games.get(game).unwrap();

        if game_running && !game_playing {
          println!("   - {} is running, MewAuto features paused", game);
        } else if !game_running && game_playing {
          println!("   - {} is not running, MewAuto features resumed", game);
        }

        games.insert(game.clone(), game_running);
        set_auto_pause_process(game_running).unwrap();
      }
    }

    std::thread::sleep(Duration::from_secs(10));
  }
}
