use std::fmt::{self, Formatter, Display};
use std::vec::Vec;
use std::thread::sleep;
use std::time::Duration;

const GAME_SIZE : usize = 10;

struct Game {
  board : [
    [bool; GAME_SIZE]
    ; GAME_SIZE
  ]
}


enum Mutation {
  On(usize, usize),
  Off(usize, usize)
}

impl Display for Game {
    // `f` is a buffer, this method must write the formatted string into it
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      for i in 0..GAME_SIZE {
        for j in 0..GAME_SIZE {
          try!(write!(
            f, "{}", if self.board[i][j] { 
              'X' 
            } else {
              '.'
            }
          ));
        }
        try!(write!(f, "\n"));
      }
      write!(f, "")
    }
}

fn mutate_single(mut game : Game, mutation : &Mutation) -> Game {
  match *mutation {
    Mutation::On(x, y) => game.board[x][y]=true,
    Mutation::Off(x, y) => game.board[x][y]=false
  }
  game
}

fn mutate(game : Game, mutations : Vec<Mutation>) -> Game {
  mutations.iter().fold(game, mutate_single)
}

fn count_neighbours(game : &Game, x : usize, y : usize) -> usize {
  [
    (1i8, -1i8),
    (1, 0),
    (1, 1),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
  ].iter()
    .map(|&(mod_x, mod_y)| {
      let n_x = x as i8 + mod_x;
      let n_y = y as i8 + mod_y;

      (n_x, n_y)
    })
    .filter(|&(n_x, n_y)| {
      let game_size_u = GAME_SIZE as i8;

      let x_out_of_range = n_x < 0 || n_x >= game_size_u;
      let y_out_of_range = n_y < 0 || n_y >= game_size_u;

      !x_out_of_range && !y_out_of_range
    })
    .filter(|&(n_x, n_y)| {
      let is_alive = game.board[n_x as usize][n_y as usize];
      is_alive
    })
    .count()
}

fn game_of_life(game : &Game) -> Vec<Mutation> {
  (0..GAME_SIZE)
    .flat_map(|i| {
      (0..GAME_SIZE).map(move |j| (i, j))
    })
    .map(|(x, y)| {
      let current_is_alive = game.board[x][y];
      let no_of_neighbours = count_neighbours(&game, x, y);

      if current_is_alive && no_of_neighbours != 2 && no_of_neighbours != 3 {
        Some(Mutation::Off(x, y))
      } else if !current_is_alive && no_of_neighbours == 3 {
        Some(Mutation::On(x, y))
      } else {
        None
      }
    })
    .filter_map(|x| x)
    .collect()
}

fn main() {
  let g = Game {
    board: [[false; GAME_SIZE]; GAME_SIZE]
  };

  let initial_game = mutate(
    g,
    vec![
      Mutation::On(1, 1),
      Mutation::On(2, 1),
      Mutation::On(1, 2),
      Mutation::On(2, 2)
    ]
  );

  let mut game = initial_game;
  loop {
    println!("{}", game);
    let mutations = game_of_life(&game);
    game = mutate(game, mutations);
    sleep(Duration::from_secs(1));
  } 
}
