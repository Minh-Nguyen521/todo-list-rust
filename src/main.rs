use std::io::{stdout, stdin, Write};
use termion::{color, cursor, clear};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use std::io::Read;


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
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut curr_idx = 0;
    let mut curr_table = false;
    let mut add_mode = false;

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
    
    for c in stdin.keys() {
        let key = c.unwrap();
        // if (!add_mode){
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
                    print!("{}", termion::cursor::Goto(1, 1));
                    print!("{}{}", clear::CurrentLine, cursor::Goto(1, 1));
                    stdout.flush().unwrap();
                    print!("Enter new item: ");
                    stdout.flush().unwrap();
                    add_mode = true;
                }
                _ => {}
            }
            if !curr_table {
                print_updated_items(&todo_list, curr_idx, &mut stdout, curr_table);
            } else {
                print_updated_items(&done_list, curr_idx, &mut stdout, curr_table);
            }

        // } else {
        //     match key{
        //         Key::Char('\n') => {
        //             println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        //             todo_list.push(input.clone());
        //             print_list(&todo_list, curr_idx, &mut stdout, curr_table);
        //             add_mode = false;
        //         }
        //         Key::Char(c) => {
        //             input.push(c);
        //             print!("{}", c);
        //             stdout.flush().unwrap();
        //         }
        //         Key::Backspace => {
        //             input.pop();
        //             print!("{}", termion::cursor::Left);
        //             print!("{}", termion::clear::AfterCursor);
        //             stdout.flush().unwrap();
        //         }
        //         _ => {}
        //     }
        // }

        
        stdout.flush().unwrap();
    }
}