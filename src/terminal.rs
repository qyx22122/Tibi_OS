use crate::print;
use crate::remove;
use crate::commands;
use alloc::string::{String};
use alloc::vec::Vec;

static mut COMMAND: String = String::new();

pub fn print_terminal_header()
{
    print!("> ");
}

unsafe fn enter_command()
{
    if COMMAND.len() < 1
    {
        return;
    }

    print!("\n");

    match COMMAND.split(" ").collect::<Vec<&str>>()[0]
    {
        "help" => {
            commands::help();
        }
        "clear" => {
            commands::clear();
        }
        "exit" => {
            commands::exit();
        }

        _ => {
            print!("No command called '{}'", COMMAND);
        }
    }
}

pub fn write_terminal(ch: char)
{
    match ch{
        // delete, escape, tab
        '\u{007F}' | '\x1B' | '\t' => {

        }
        // backspace
        '\x08' => {
            unsafe{
                if COMMAND.len() > 0
                {
                    COMMAND.pop();
                    remove!();
                }
            }
        }
        // newline
        '\n' => {
            unsafe {
                enter_command();
                COMMAND = String::new();
            };
            print!("\n");
            print_terminal_header();
        }
        _ => {
            unsafe{
                if COMMAND.len() > 77
                {
                    write_terminal('\n');
                    return;
                }
                COMMAND.push(ch);
            }
            print!("{}", ch);
        }
    }
}