use std::fmt;
use std::fs::File;
use std::io::prelude::*;

use rand::{thread_rng, Rng};
use colored::*;
use term_size;

const BASE_DIR: &str = "./static/words.txt";

#[derive(Debug)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


fn get_word(difficulty: Difficulty) -> String {
    let mut rand_gen = thread_rng();
    let word_list = match difficulty {
        Difficulty::Easy => read_words(4, 6),
        Difficulty::Medium => read_words(6, 8),
        Difficulty::Hard => read_words(8, usize::max_value()),
    };
    let selection: usize = rand_gen.gen_range(0, word_list.len()) as usize;
    word_list[selection].clone()
}

fn read_words(min_len: usize, max_len: usize) -> Vec<String> {
    match read_file() {
        Ok(contents) => {
            return contents.iter()
                    .filter(|s| s.len() >= min_len && s.len() <= max_len )
                    .map(|s| s.to_lowercase().clone())
                    .collect::<Vec<String>>();
        },
        _ => panic!("Can't read source file!"),
    };
}

fn read_file() -> std::io::Result<Vec<String>> {
    let path = String::from(BASE_DIR);
    let mut file  = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut lines: Vec<String> = Vec::new();
    for line in contents.split("\n") {
        lines.push(String::from(line));
    }

    Ok(lines)
}

fn select_difficulty() -> Difficulty {
    let difficulty: Difficulty = match input("Please select a difficulty [E/m/h]") {
        Ok(s) => match s {
            'e' => Difficulty::Easy,
            'm' => Difficulty::Medium,
            'h' => Difficulty::Hard,
            _ => Difficulty::Easy,
        },
        _ => {
            println!("Can't understand the selection selecting Easy");
            Difficulty::Easy
        }
    };
    println!("Starting {} game...", difficulty);
    difficulty
}

fn input(msg: &str) -> Result<char, &'static str> {
    print!("{}: ", msg);
    std::io::stdout().flush().expect("Can't write msg to stdout");
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Failed to read input");
    match buf.trim().to_lowercase().parse::<char>() {
        Ok(s) => Ok(s),
        _ => Err("Parsing Error"),
    }
}

#[derive(PartialEq)]
enum GameState {
    OutOfGuess,
    NotFinished,
    Complete,
}

struct Game <'a> {
    word: String,
    num_guess: usize,
    _drawer: &'a Drawer,
    _successful: Vec<char>,
    _unsuccessful: Vec<char>,
}

impl<'a> Game <'a> {
    fn new(word: String, drawer: &'a Drawer) -> Game {
        Game {
            word,
            num_guess: 10,
            _drawer: drawer,
            _successful: Vec::new(),
            _unsuccessful: Vec::new(),
        }
    }

    fn start(&mut self) {
        loop {
            match self.game_state() {
                GameState::Complete => {
                    self._display_word();
                    println!("Congratulations! You finished the game!");
                    self._drawer.horizontal_line();
                    break;
                },
                GameState::OutOfGuess => {
                    println!("You are out of tries :( It was {}", self.word.bold());
                    self._drawer.horizontal_line();
                    break;
                },
                _ => {
                    self._display_word();
                    println!(
                        "{} {} Unsuccessful tries: {:?}",
                        (self.num_guess - self._unsuccessful.len()).to_string().red().bold(),
                        "guesses remain.".red().bold(),
                        self._unsuccessful
                    );
                    let x = self._get_guess();
                    match self._is_hit(x) {
                        true => {
                            println!("{}", "Correct!".bold());
                            self._successful.push(x);
                        },
                        false => {
                            println!("{}", "False!".bold());
                            self._unsuccessful.push(x);
                        }
                    };
                }
            }
        }
    }

    fn _display_word(&self) {
        let a = self.word.chars().map(|x| {
            if self._successful.contains(&x) {
                x
            } else {
                '_'
            }
        }).map(|c| c.to_string()).collect::<Vec<_>>().join("");
        println!("\n        {}\n", a.white().on_blue().bold());
    }

    fn _get_guess(&self) -> char {
        loop {
            match input("Guess a letter from a to z") {
                Ok(s) => if s.is_alphabetic() {
                    match self._is_guessed(s) {
                        true => println!("You've already used that letter, please guess another one."),
                        false => return s,
                    }
                } else {
                    println!("Please guess a letter.");
                },
                _ => println!("Can't understand the guess, try again!")
            }
        }
    }

    fn _is_guessed(&self, c: char) -> bool {
        let a = self._successful.iter().chain(self._unsuccessful.iter())
            .filter(|x| **x == c).collect::<Vec<_>>();

        a.len() > 0
    }

    fn _is_hit(&self, c: char) -> bool {
        self.word.contains(c)
    }

    fn game_state(&self) -> GameState {
        if self.num_guess <= self._unsuccessful.len() {
            return GameState::OutOfGuess;
        }
        match self.word.chars().map(|x| self._successful.contains(&x)).fold(true, |acc, x| acc && x) {
            true => GameState::Complete,
            false => GameState::NotFinished,
        }
    }
}

struct Drawer {
    width: usize,
}

impl Drawer {
    fn _space(&self, w: Option<usize>) -> String {
        (vec![" "; w.unwrap_or(1)]).concat()
    }
    fn horizontal_line(&self) {
        let a = (vec!["━"; self.width]).concat();
        println!("{}", a);
    }

    fn centered_msg(&self, msg: &str) {
        let mut centered_msg:String;
        let available_width = self.width - 2;

        if msg.len() > available_width {
            centered_msg = String::from(&msg[..(available_width-2)]);
            centered_msg.push_str("..");
        } else {
            let for_padding = available_width - msg.len();
            let (left_padding, right_padding) = if for_padding % 2 == 0 {
                (for_padding / 2, for_padding / 2)
            } else {
                (for_padding / 2, for_padding / 2 + 1)
            };
            let left_pad = self._space(Some(left_padding));
            let right_pad = self._space(Some(right_padding));
            centered_msg = [left_pad, String::from(msg), right_pad].concat();
        }

        let border ="║".bold();
        println!("{}{}{}", border, centered_msg, border);
    }

    fn new(width: usize) -> Drawer {
        Drawer {
            width
        }
    }

    fn welcome(&self) {
        self.horizontal_line();
        self.centered_msg("HANGMAN");
        self.horizontal_line();
    }
}

fn main() {
    let drawer = match term_size::dimensions() {
        Some((w, _h)) => Drawer::new(w),
        _ => panic!("Can't get terminal size!")
    };
    drawer.welcome();
    let mut game = Game::new(get_word(select_difficulty()), &drawer);
    game.start();
}
