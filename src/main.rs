use std::io::{stdout, stdin, Write};
use termion::{color, cursor, clear};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

fn print_list<W: Write>(todo_list: &Vec<String>, curr_idx: usize, stdout: &mut W, curr_table: bool) {
    let check = if curr_table { 'X' } else { ' ' };
    for (idx, todo) in todo_list.iter().enumerate() {
        print!("{}", termion::cursor::Goto(1, idx as u16 + 1));
        if idx == curr_idx {
            print!("[{}] {}{}{}", check, color::Fg(color::White), todo, color::Fg(color::Reset));
        } else {
            print!("[{}] {}", check, todo);
        }
    }
}

fn print_updated_items<W: Write>(todo_list: &Vec<String>, curr_idx: usize, stdout: &mut W, curr_table: bool) {
    let check = if curr_table { 'X' } else { ' ' };
    for (idx, todo) in todo_list.iter().enumerate() { 
        print!("{}", termion::cursor::Goto(1, idx as u16 + 1));
        if idx == curr_idx {
            print!("[{}] {}{}{}", check, color::Fg(color::White), todo, color::Fg(color::Reset));
        } else {
            print!("[{}] {}", check, todo);
        }
    }
}

fn main() {
    let stdin_handle = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut curr_idx = 0;
    let mut curr_table = false;

    let mut todo_list = vec![
        "Buy milk".to_string(),
        "Clean room".to_string(),
        "Do laundry".to_string(),
        "Cook dinner".to_string(),
        "Finish project".to_string(),
    ];

    let mut done_list = vec![
        "Buy milk".to_string(),
        "Do laundry".to_string(),
        "Finish project".to_string(),
    ];

    println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    print_list(&todo_list, curr_idx, &mut stdout, curr_table);
    stdout.flush().unwrap();
    
    for c in stdin_handle.keys() {
        let key = c.unwrap();

        match key {
            Key::Char('q') => break, 
            Key::Up => {
                if curr_idx > 0 {
                    curr_idx -= 1;
                }
            },
            Key::Down => {
                if (!curr_table && curr_idx < todo_list.len() - 1) || (curr_table && curr_idx < done_list.len() - 1) {
                    curr_idx += 1;
                }
            },
            Key::Char('\t') => {
                curr_table = !curr_table;
                curr_idx = 0; 
                println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
                if !curr_table {
                    print_list(&todo_list, curr_idx, &mut stdout, curr_table);
                } else {
                    print_list(&done_list, curr_idx, &mut stdout, curr_table);
                }
            }
            Key::Char('\n') => {
                println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
                if (!curr_table && todo_list.len() == 0) || (curr_table && done_list.len() == 0) {
                    // do nothing
                } else {
                    if !curr_table {
                        done_list.push(todo_list[curr_idx].clone());
                        todo_list.remove(curr_idx);
                        print_list(&todo_list, curr_idx, &mut stdout, curr_table);
                    } else {
                        todo_list.push(done_list[curr_idx].clone());
                        done_list.remove(curr_idx);
                        print_list(&done_list, curr_idx, &mut stdout, curr_table);
                    }
                }
            }
            Key::Char('i') => {
                let mut input = String::new();
                print!("{}{}Enter New Task: ", termion::clear::All, termion::cursor::Goto(1, 1));
                stdout.flush().unwrap();

                loop {
                    let c = stdin_handle.keys().next().unwrap().unwrap();
                    match c {
                        Key::Char('\n') => break, 
                        Key::Char(ch) => {
                            input.push(ch); 
                            print!("{}", ch); 
                            stdout.flush().unwrap();
                        }
                        Key::Backspace => {
                            if !input.is_empty() {
                                input.pop(); // Remove last character
                                print!("{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine);
                                print!("{}Enter New Task: {}", termion::cursor::Goto(1, 1), input); 
                                stdout.flush().unwrap();
                            }
                        }
                        Key::Delete => {
                            // Since we don't have cursor position handling, we'll just ignore this for now.
                        }
                        _ => {}
                    }
                }

                todo_list.push(input.trim().to_string());
                curr_table = false;
                println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
                print_list(&todo_list, curr_idx, &mut stdout, false);
                stdout.flush().unwrap();
            }
            _ => {}
        }

        if !curr_table {
            print_updated_items(&todo_list, curr_idx, &mut stdout, curr_table);
        } else {
            print_updated_items(&done_list, curr_idx, &mut stdout, curr_table);
        }
        
        stdout.flush().unwrap();
    }
}
