use crossterm::{event::{KeyCode, Event, read, KeyModifiers}, terminal::{enable_raw_mode, disable_raw_mode}};
use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::io::{stdin, stdout, Write};

pub fn start_repl() {
    enable_raw_mode();

    let mut interpreter = Interpreter::new();

    let mut stdin = stdin();
    let mut stdout = stdout();

    let mut input = String::new();
    let mut cursor = 0;
    let mut scrollback = 0;
    let mut commands: Vec<String> = Vec::new();
    print!("> ");
    let _ = stdout.flush();

    loop {
        let event = match read().unwrap() {
            Event::Key(key) => key,
            _ => continue,
        };

        match event.code {
            KeyCode::Char('c') if event.modifiers == KeyModifiers::CONTROL => {
                break;
            }
            KeyCode::Char(c) => { 
                if cursor == input.len() {
                    input.push(c);
                } else {
                    input.insert(cursor, c);
                }
                cursor += 1;
            }
            KeyCode::Backspace => { 
                if cursor == 0 { continue; }
                input.remove(cursor - 1);
                cursor -= 1;
            },
            KeyCode::Left => if cursor != 0 { cursor -= 1; }
            KeyCode::Right => if cursor != input.len() { cursor += 1; }
            KeyCode::Up => {
                if scrollback == commands.len() { continue; }
                scrollback += 1;
                input = commands[commands.len() - scrollback].clone();
            }
            KeyCode::Down => {
                scrollback -= 1;
                input = commands[commands.len() - scrollback].clone();
            }
            KeyCode::Enter => {
                print!("\n\r");
                let tokens = Lexer::new(input.clone()).parse();
                let ast = Parser::new(tokens).parse().unwrap();
                let result = interpreter.parse(ast);
                if let Err(msg) = result { println!("{msg}"); }
                commands.push(input.clone());
                input.clear();
                cursor = 0;
                print!("\n\r> ");
            }
            _ => {} 
        }

        print!("\x1b[2K\r> {input}\r\x1b[{}C", cursor + 2);
        let _ = stdout.flush();
    }

    disable_raw_mode();
}
