use std::io::{Stdout, Write};
use termion::cursor::DetectCursorPos;
use termion::raw::RawTerminal;

pub fn enter(
    input: &mut String,
    is_history_traversal_mode: bool,
    history: &mut Vec<String>,
) {
    if is_history_traversal_mode {
        history.pop();
    }
    input.push('\n');
}

pub fn backspace(input: &mut String, stdout: &mut RawTerminal<Stdout>, prompt_len: u16) {
    //input_bf_cursor holds the value of the input before the cursor
    //input_af_cursor holds the value of the input after the cursor
    let (mut x, _y) = stdout.cursor_pos().unwrap();
    x -= prompt_len + 1;

    if x == 0 {
        return;
    }

    let mut input_bf_cursor = input[..x as usize].to_string();
    let input_af_cursor = input[x as usize..].to_string();

    input_bf_cursor.pop().unwrap();

    //to show the backspace deletion on screen:
    //move cursor to the left on position, print whitespace, move cursor to the left
    //save the cursor position
    //clear screen from current position to end of line
    //print input_af_cursor
    //restore the cursor position

    write!(
        stdout,
        "{} {}{}{}{}{}",
        termion::cursor::Left(1),
        termion::cursor::Left(1),
        termion::cursor::Save,
        termion::clear::UntilNewline,
        input_af_cursor,
        termion::cursor::Restore,
    )
    .unwrap();

    //update input value
    *input = input_bf_cursor + &input_af_cursor;
}

pub fn insertion(
    character: char,
    input: &mut String,
    stdout: &mut RawTerminal<Stdout>,
    prompt_len: u16,
) {
    let (mut x, _y) = stdout.cursor_pos().unwrap();
    x -= prompt_len + 1;

    let mut input_bf_cursor = input[..x as usize].to_string();
    let input_af_cursor = input[x as usize..].to_string();

    input_bf_cursor.push(character);

    //handling insertion of character:
    //print character
    //save cursor position
    //clear screen from current position to end of line
    //print input_af_cursor
    //restore cursor position

    write!(
        stdout,
        "{}{}{}{}{}",
        character,
        termion::cursor::Save,
        termion::clear::UntilNewline,
        input_af_cursor,
        termion::cursor::Restore,
    )
    .unwrap();

    //update input value
    *input = input_bf_cursor + &input_af_cursor;
}

pub fn left(stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
}

pub fn right(stdout: &mut RawTerminal<Stdout>) {
    write!(stdout, "{}", termion::cursor::Right(1)).unwrap();
}

pub fn up(
    stdout: &mut RawTerminal<Stdout>,
    history: &mut Vec<String>,
    history_index: &mut usize,
    prompt_len: u16,
    input: &mut String,
    is_history_traversal_mode: &mut bool,
) {
    if history.len() == 0 || *history_index == 0 {
        return;
    }

    if !(*is_history_traversal_mode) {
        history.push((*input).clone());
        *is_history_traversal_mode = true;
    }

    *history_index -= 1;

    let (_, y) = stdout.cursor_pos().unwrap();
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(prompt_len + 1, y),
        termion::clear::AfterCursor,
        history[*history_index]
    )
    .unwrap();

    //update value of input
    *input = history[*history_index].clone();
}

pub fn down(
    stdout: &mut RawTerminal<Stdout>,
    history: &Vec<String>,
    history_index: &mut usize,
    prompt_len: u16,
    input: &mut String,
) {
    if history.len() == 0 || *history_index >= history.len() - 1 {
        return;
    }

    *history_index += 1;
    let (_, y) = stdout.cursor_pos().unwrap();
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Goto(prompt_len + 1, y),
        termion::clear::AfterCursor,
        history[*history_index]
    )
    .unwrap();

    *input = history[*history_index].clone();
}
