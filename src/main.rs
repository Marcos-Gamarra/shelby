use std::io::{stdin, stdout, Write};
use std::process::Command;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod keys;

fn main() {
    let mut history: Vec<String> = Vec::new();
    loop {
        let prompt = String::from(" > ");
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();

        //read input from user
        let input = take_input(prompt.len() as u16, &mut history);

        //splits the user input by whitespaces into args.
        //args.remove(0) removes the first element from args and returns it,
        //so command now will hold the name of the command and args only the arguments.
        let mut args: Vec<String> = input.trim().split(" ").map(|arg| arg.to_string()).collect();
        let command = args.remove(0);

        //executes the command as child process

        if command == "exit" {
            break;
        }

        let child = Command::new(&command).args(args).spawn();

        //if the child executes correctly wait until it completes, else
        //print error message
        if let Ok(mut child) = child {
            history.push(input[..input.len() - 1].to_string());
            child.wait().unwrap();
        } else {
            println!("\nError while trying to execute command: {}", command);
        }
    }
}

fn take_input(prompt_len: u16, history: &mut Vec<String>) -> String {
    let mut history_index = history.len();
    let mut is_history_traversal_mode = false;
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut input = String::new();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => {
                keys::enter(&mut input, is_history_traversal_mode, history);
                break;
            }
            Key::Ctrl('c') => {
                input.clear();
                break;
            }
            //Key::Esc => println!("ESC"),
            Key::Char(c) => keys::insertion(c, &mut input, &mut stdout, prompt_len),
            Key::Left => keys::left(&mut stdout),
            Key::Right => keys::right(&mut stdout),
            Key::Up => keys::up(
                &mut stdout,
                history,
                &mut history_index,
                prompt_len,
                &mut input,
                &mut is_history_traversal_mode,
            ),
            Key::Down => keys::down(
                &mut stdout,
                history,
                &mut history_index,
                prompt_len,
                &mut input,
            ),
            Key::Backspace => keys::backspace(&mut input, &mut stdout, prompt_len),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    input
}
