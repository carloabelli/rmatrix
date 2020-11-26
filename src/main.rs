use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    collections::VecDeque,
    io::{stdout, Write},
    thread::sleep,
    time,
};
use termion::{clear, color, cursor, screen, style, terminal_size};

const CHARS: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!', '@', '#', '$', '%',
    '^', '&', '*', '(', ')', '-', '_', '+', '=', '{', '[', '}', ']', '|', '\\', '<', ',', '>', '.',
    '?', '/', '"', '\'',
];
const DEFAULT_MIN_GAP_LENGTH: u16 = 20;
const DEFAULT_MAX_GAP_LENGTH: u16 = 50;
const DEFAULT_MIN_TRAIL_LENGTH: u16 = 10;
const DEFAULT_MAX_TRAIL_LENGTH: u16 = 20;

struct Trail {
    front: u16,
    front_char: &'static char,
    length: u16,
}

impl Trail {
    fn new(length: u16) -> Trail {
        Trail {
            front: 0,
            front_char: &'\0',
            length,
        }
    }

    fn back(&self) -> Option<u16> {
        if self.front + 1 >= self.length {
            Some(self.front + 1 - self.length)
        } else {
            None
        }
    }
}

struct Column {
    trails: VecDeque<Trail>,
    wait: u16,
}

impl Column {
    fn with_wait(wait: u16) -> Column {
        Column {
            trails: VecDeque::new(),
            wait,
        }
    }
}

fn main() {
    ctrlc::set_handler(|| {
        print!("{}{}", cursor::Show, screen::ToMainScreen);
        std::process::exit(0);
    })
    .unwrap();

    print!(
        "{}{}{}",
        screen::ToAlternateScreen,
        clear::All,
        cursor::Hide
    );

    let mut rng = thread_rng();
    let mut columns = Vec::new();
    loop {
        let (width, height) = terminal_size().unwrap();
        let width = width as usize;

        if width < columns.len() {
            columns.truncate(width);
        } else if width as usize > columns.len() {
            let num_new_columns = width - columns.len();
            columns.extend(
                (0..num_new_columns)
                    .map(|_| Column::with_wait(rng.gen_range(0, DEFAULT_MAX_GAP_LENGTH + 1))),
            );
        }

        for (i, column) in columns.iter_mut().enumerate() {
            if column.wait == 0 {
                let length: u16 =
                    rng.gen_range(DEFAULT_MIN_TRAIL_LENGTH, DEFAULT_MAX_TRAIL_LENGTH + 1);
                let trail = Trail::new(length);
                column.trails.push_back(trail);
                column.wait =
                    length + rng.gen_range(DEFAULT_MIN_GAP_LENGTH, DEFAULT_MAX_GAP_LENGTH + 1);
            } else {
                column.wait -= 1;
            }

            if let Some(front_trail) = column.trails.front() {
                if let Some(back) = front_trail.back() {
                    if back > height {
                        column.trails.pop_front();
                    }
                }
            }

            for trail in column.trails.iter_mut() {
                if trail.length > 1 && trail.front < height {
                    print!(
                        "{}{}{}{}",
                        cursor::Goto(i as u16 + 1, trail.front + 1),
                        style::Reset,
                        color::Fg(color::Green),
                        trail.front_char,
                    );
                }

                if let Some(back) = trail.back() {
                    if back < height {
                        print!("{} ", cursor::Goto(i as u16 + 1, back + 1));
                    }
                }

                trail.front += 1;

                if trail.front < height {
                    let c = CHARS.choose(&mut rng).unwrap();
                    print!(
                        "{}{}{}{}",
                        cursor::Goto(i as u16 + 1, trail.front + 1),
                        style::Bold,
                        color::Fg(color::White),
                        c,
                    );
                    trail.front_char = c;
                }
            }
        }

        stdout().flush().unwrap();
        sleep(time::Duration::from_millis(50));
    }
}
