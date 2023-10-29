use crate::print;
use crate::remove;
use crate::commands;
use alloc::string::{String};

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

    match COMMAND.as_str()
    {
        "help" => {
            commands::help();
        }

        _ => {
            print!("No command called '{}'", COMMAND);
        }
    }
}

pub fn write_terminal(ch: char)
{
    match ch{
        '\x08'=> {
            unsafe{
                if COMMAND.len() > 0
                {
                    COMMAND.pop();
                    remove!();
                }
            }
        }
        '\n' => {
            unsafe {
                enter_command();
                COMMAND = String::new();
            };
            print!("\n");
            print_terminal_header();
        }
        _ => {
            unsafe{COMMAND.push(ch)};
            print!("{}", ch);
        }
    }
}