use termion::async_stdin;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::{clear, cursor};
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;
use snake::{Field, Velocity};

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
    let mut field = Field::new(borders.0, borders.1);
    field.set_snake_velocity(Velocity::new(1, 0));

    'outer: loop {
        if let Some(c) = stdin.next() {
            match c.unwrap() {
                Key::Char('q')  => break 'outer,
                Key::Left       => field.set_snake_velocity(Velocity::new(-1, 0)),
                Key::Right      => field.set_snake_velocity(Velocity::new(1, 0)),
                Key::Up         => field.set_snake_velocity(Velocity::new(0, -1)),
                Key::Down       => field.set_snake_velocity(Velocity::new(0, 1)),
                _ => {},
            }
        }

        // Clears screen
        write!(stdout, "{}", clear::All).unwrap();
        field.calc_new_frame();
        write!(stdout, "{}", field).unwrap();

        if field.is_game_over() {
            write!(stdout, "{}", field.get_game_over_message()).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_secs(2));
            break 'outer;
        }

        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(70));
    }

    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::Show).unwrap();
}
