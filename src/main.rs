use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

mod game;
use game::*;

extern crate ncurses;
use ncurses::*;

const FILE_NAME : &'static str = "game_of_life.save";

#[derive(PartialEq)]
enum Direction {
  Up, Down, Left, Right
}

#[derive(PartialEq)]
enum Action {
  SpeedUp,
  SpeedDown,
  ToggleHelp,
  ToggleRunning,
  ToggleFieldActive,
  Cursor(Direction),
  SaveToFile,
  LoadFromFile,
  Quit,
  None
}

struct IoError;

impl From<std::io::Error> for IoError {
  fn from(err : std::io::Error) -> IoError {
    IoError
  }
}

struct GameState {
  game : Game,
  speed: f64,
  is_running: bool,
  is_displaying_help: bool,
  cursor_x: usize,
  cursor_y: usize
}

fn key_to_action(ch : i32) -> Action {
  match ch as u8 as char {
    '>' => Action::SpeedUp,
    '<' => Action::SpeedDown,
    ' ' => Action::ToggleRunning,
    'x' => Action::ToggleFieldActive,
    'j' => Action::Cursor(Direction::Up),
    'k' => Action::Cursor(Direction::Down),
    'h' => Action::Cursor(Direction::Left),
    'l' => Action::Cursor(Direction::Right),
    's' => Action::SaveToFile,
    'f' => Action::LoadFromFile,
    '?' => Action::ToggleHelp,
    'q' => Action::Quit,
    _ => match ch {
      KEY_LEFT => Action::Cursor(Direction::Left),
      KEY_RIGHT => Action::Cursor(Direction::Right),
      KEY_UP => Action::Cursor(Direction::Down),
      KEY_DOWN => Action::Cursor(Direction::Up),
      KEY_F1 => Action::ToggleHelp,
      _ => Action::None
    }
  }
}

fn handle_action(a : Action, game_state : &mut GameState) -> Result<(), IoError> {
  match a {
    Action::SpeedUp => game_state.speed *= 2.0,
    Action::SpeedDown => game_state.speed /= 2.0,
    Action::ToggleRunning => game_state.is_running = !game_state.is_running,
    Action::Cursor(x) => {
      let (mod_x, mod_y) = match x {
        Direction::Up => (1i8, 0i8),
        Direction::Down => (-1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
      };
      game_state.cursor_x = add_mod_game_size(game_state.cursor_x, mod_x);
      game_state.cursor_y = add_mod_game_size(game_state.cursor_y, mod_y);
    },
    Action::ToggleHelp => {
      game_state.is_displaying_help = !game_state.is_displaying_help;
      if game_state.is_displaying_help {
        game_state.is_running = false
      }
    },
    Action::SaveToFile => {
      let mut f = try!(File::create(FILE_NAME));
      try!(f.write_fmt(format_args!("{}", game_state.game)));
      try!(f.sync_data());
    },
    Action::LoadFromFile => {
      game_state.is_running = false;
      let mut s = String::new();
      let mut f = try!(File::open(FILE_NAME));
      try!(f.read_to_string(&mut s));
      game_state.game = Game::new(&s);
    },
    Action::ToggleFieldActive => {
      let (cur_x, cur_y) = (game_state.cursor_x, game_state.cursor_y);
      game_state.game.mutate(
        vec![
          Mutation::Toggle(cur_x, cur_y) 
        ]
      );
    }
    _ => ()
  };
  Ok(())
}

fn get_help() -> String {
  format!(r#"
  Help!
  F1, ?   - Show/Hide This Help
  q       - Quit
  x       - Toggle Cell
  [space] - Start / Pause
  <, >    - Adjust simulation speed
  c       - Clear
  s       - Dump state to file
  f       - Load state from file
  "#)
}

fn main() {
  let game = Game::new(&r#"
    ..X.
    ...X 
    .XXX 
  "#.to_string());

  let mut game_state = GameState {
    game: game,
    speed: 1.0,
    is_running: false,
    is_displaying_help: true,
    cursor_x: 0,
    cursor_y: 0
  };

  initscr();
  noecho();
  keypad(stdscr, true);
  timeout(0);

  loop {
    // print board
    clear();
    mv(0, 0);

    if !game_state.is_displaying_help {
      printw(format!("{}", game_state.game).as_ref());
      mv(game_state.cursor_x as i32, game_state.cursor_y as i32);
    } else {
      printw(get_help().as_ref());
    }
    refresh();


    // handle actions
    let ch = getch();
    let action = key_to_action(ch);
    if action == Action::Quit {
      break;
    }
    handle_action(action, &mut game_state);

    // run simulation step
    if game_state.is_running {
      curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
      let mutations = game_of_life(&game_state.game);
      game_state.game.mutate(mutations);

      // slow it down
      let sleep_millis = (1.0 / game_state.speed * 500.0) as u64;
      sleep(Duration::from_millis(sleep_millis));
    } else {
      curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
    }

    sleep(Duration::from_millis(100));
  }
  endwin();
}
