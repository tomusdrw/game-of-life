use std::fmt::{self, Formatter, Display};
use std::vec::Vec;

const GAME_SIZE : usize = 10;

pub struct Game {
  board : [
    [bool; GAME_SIZE]
    ; GAME_SIZE
  ]
}

pub enum Mutation {
  On(usize, usize),
  Off(usize, usize)
}

impl Game {
  pub fn from_str(rows : Vec<String>) -> Self {
    let mut game = Game {
      board: [[false; GAME_SIZE]; GAME_SIZE]
    };

    let cells_to_activate = rows
      .iter()
      .zip(0..GAME_SIZE)
      .map(|(row, x)| {
        row
          .chars()
          .zip(0..GAME_SIZE)
          .filter(|&(c, _)| c == 'X')
          .map(move |(_, y)| {
            (x, y)
          })
      })
      .flat_map(|x| x);

    for (x, y) in cells_to_activate {
      game.board[x][y] = true;
    }
    
    game
  } 
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

pub fn mutate(game : Game, mutations : Vec<Mutation>) -> Game {
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

pub fn game_of_life(game : &Game) -> Vec<Mutation> {
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
