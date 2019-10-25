use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::{clear, cursor};
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;
use snake::{Game, Velocity};

fn main() {
    let mut stdin = async_stdin().keys();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Use arrow keys to control the O. Press space to start...{}",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Hide
    ).unwrap();

    stdout.flush().unwrap();

    'starter: loop {
        if let Some(c) = stdin.next() {
            match c.unwrap() {
                Key::Char(' ') => break 'starter,
                _ => {},
            }
        }

        thread::sleep(Duration::from_millis(50));
    }

    let borders = termion::terminal_size().unwrap();
    let mut game = Game::new(borders.0, borders.1);
    game.set_snake_velocity(Velocity::new(1, 0));

    'main: loop {
        if let Some(c) = stdin.next() {
            match c.unwrap() {
                Key::Char('q')  => break 'main,
                Key::Left       => game.set_snake_velocity(Velocity::new(-1, 0)),
                Key::Right      => game.set_snake_velocity(Velocity::new(1, 0)),
                Key::Up         => game.set_snake_velocity(Velocity::new(0, -1)),
                Key::Down       => game.set_snake_velocity(Velocity::new(0, 1)),
                _ => {},
            }
        }

        // Clears screen
        write!(stdout, "{}", clear::All).unwrap();
        game.calc_new_frame();
        write!(stdout, "{}", game).unwrap();

        if game.is_game_over() {
            write!(stdout, "{}", game.get_game_over_message()).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_secs(2));
            break 'main;
        }

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(70));
    }

    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();
}
