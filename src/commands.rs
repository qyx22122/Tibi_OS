use crate::println;
use crate::print;
use alloc::string::String;
use alloc::vec::Vec;

pub fn help(_args: Vec<&str>)
{
    print!("Simple OS\n");
    print!("Terminal commands:\n");
    print!("help,\n");
    print!("clear,\n");
    print!("echo,\n");
    print!("exit");
}
pub fn clear(_args: Vec<&str>)
{
    for _ in 0..25
    {
        println!();
    }
}
pub fn echo(args: Vec<&str>)
{
    let mut string = String::new();

    for word in args.iter(){
        string.push_str(word);
        string.push(' ');
    }
    string.pop();
    print!("{}", string);
}
pub fn exit()
{
    for _ in 1..2{
        exit();
    }
}