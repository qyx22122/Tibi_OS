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

    let mut split = COMMAND.split(" ").collect::<Vec<&str>>();

    let command = split[0];

    split.remove(0);

    match command
    {
        "help" => {
            commands::help(split);
        }
        "clear" => {
            commands::clear(split);
        }
        "exit" => {
            commands::exit();
        }
        "echo" => {
            commands::echo(split);
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