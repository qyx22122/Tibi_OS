use crate::print;
use crate::remove;
use crate::commands;
use alloc::string::{String, ToString};
use alloc::format;
use alloc::vec::Vec;
use crate::filesystem;

static mut COMMAND: String = String::new();

static mut DIRECTORY: String = String::new();

static mut PREV_COMMANDS: Vec<String> = Vec::new();
static mut CURRENT_INDEX: i32 = -1;

pub fn init()
{
    unsafe
    {
        DIRECTORY = "/".to_string();
    }
    print_terminal_header();
}

pub fn print_terminal_header()
{
    unsafe
    {
        print!("{}> ", DIRECTORY);
    }
}

unsafe fn enter_command()
{
    if COMMAND.len() < 1
    {
        return;
    }

    print!("\n");

    
    save_command();
    
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
        "ls" => {
            unsafe
            {
                commands::ls(DIRECTORY.clone());
            }
        }
        "cf" => {
            unsafe
            {
                if split.len() < 1
                {
                    DIRECTORY = "/".to_string();
                }
                else if filesystem::exists(format!("/{}", split[0]))
                {
                    DIRECTORY = format!("/{}", split[0]);
                }
                else
                {
                    print!("Couldn't find /{} in filesystem.", split[0]);
                }
            }
        }
        "mk" => {
            commands::mk(split);
        }
        "rm" => {
            if split.len() < 1
            {
                print!("You must provide an argument.");
            }
            else if filesystem::exists(format!("/{}", split[0]))
            {
                commands::rm(split[0].to_string());
            }
            else
            {
                print!("Couldn't find /{} in filesystem.", split[0]);
            }
        }

        _ => {
            print!("No command called '{}'", COMMAND);
        }
    }
}
unsafe fn save_command()
{
    PREV_COMMANDS.push(COMMAND.clone());
}

pub fn get_raw_key(key: String)
{
    if key == "ArrowUp"
    {
        unsafe{

            // Make sure the index doesn't increse past the lenght of commands-1
            if CURRENT_INDEX+1 < PREV_COMMANDS.len() as i32
            {
                CURRENT_INDEX+=1;

                // Reverse the commands
                let reversed: &mut Vec<String> = &mut PREV_COMMANDS;
                reversed.reverse();
                
                // Get the command
                let command = reversed[(CURRENT_INDEX) as usize].clone();

                if command != COMMAND
                {
                        
                    // Remove the current line
                    for _ in 0..COMMAND.len()
                    {
                        remove!();
                    }
                    
                    // Update the command
                    COMMAND = command;
                    print!("{}", COMMAND);
                }
            }
        }
    }
    else if key == "ArrowDown"
    {
        unsafe
        {
            if CURRENT_INDEX != 0 && CURRENT_INDEX != -1
            {
                CURRENT_INDEX -= 1;
                
                // Reverse the commands
                let reversed: &mut Vec<String> = &mut PREV_COMMANDS;
                reversed.reverse();
                
                // Get the command
                let command = reversed[(CURRENT_INDEX) as usize].clone();

                // Remove the current line
                for _ in 0..COMMAND.len()
                {
                    remove!();
                }

                // Update the command
                COMMAND = command;
                print!("{}", COMMAND);
            }
        }
    }
}

pub fn write_terminal(ch: char)
{
    match ch{
        // delete, tab
        '\u{007F}' | '\t' => {

        }
        // escape
        '\x1B' => {
            // remove the current line
            unsafe{
                for _ in 0..COMMAND.len()
                {
                    remove!();
                }
                CURRENT_INDEX = -1;
                COMMAND = String::new();
            }
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
                CURRENT_INDEX = -1;
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