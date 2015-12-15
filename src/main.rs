use std::thread::sleep;
use std::time::Duration;

mod game;
use game::*;

fn main() {
  let mut game = Game::from_str([
    "..X.",
    "...X", 
    ".XXX", 
  ].iter()
    .map(|x| x.to_string())
    .collect()
  );

  loop {
    println!("{}", game);
    let mutations = game_of_life(&game);
    game = mutate(game, mutations);
    sleep(Duration::from_secs(1));
  } 
}
