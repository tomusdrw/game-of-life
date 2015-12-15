use std::thread::sleep;
use std::time::Duration;

mod game;
use game::*;

extern crate ncurses;
use ncurses::*;

#[derive(PartialEq)]
enum Direction {
  Up, Down, Left, Right
}

#[derive(PartialEq)]
enum Action {
  SpeedUp,
  SpeedDown,
  ToggleRunning,
  ToggleFieldActive,
  Cursor(Direction),
  Quit,
  None
}

struct GameState {
  game : Game,
  speed: f64,
  is_running: bool,
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
    'q' => Action::Quit,
    _ => Action::None
  }
}

fn bind_to_range(val : i8, min : usize, max : usize) -> usize {
  if val < min as i8 {
    min
  } else if val >= max as i8 {
    max - 1 
  } else {
    val as usize
  }
}

fn handle_action(a : Action, game_state : &mut GameState) {
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
      game_state.cursor_x = bind_to_range(game_state.cursor_x as i8 + mod_x, 0, GAME_SIZE);
      game_state.cursor_y = bind_to_range(game_state.cursor_y as i8 + mod_y, 0, GAME_SIZE);
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
  }
}

fn main() {
  let game = Game::from_str([
    "..X.",
    "...X", 
    ".XXX", 
  ].iter()
    .map(|x| x.to_string())
    .collect()
  );

  let mut game_state = GameState {
    game: game,
    speed: 1.0,
    is_running: false,
    cursor_x: 0,
    cursor_y: 0
  };

  initscr();
  noecho();
  timeout(0);

  loop {
    // print board
    clear();
    mv(0, 0);
    printw(format!("{}", game_state.game).as_ref());
    mv(game_state.cursor_x as i32, game_state.cursor_y as i32);
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
