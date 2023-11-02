use crate::println;
use crate::print;
use alloc::string::{String,ToString};
use alloc::format;
use alloc::vec::Vec;
use crate::filesystem;

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
pub fn ls(path: String)
{
    let file = filesystem::read(path.clone());

    print!("{}", file);
}
pub fn mk(mut args: Vec<&str>)
{
    let path: String = args[0].to_string();
    let mut string = String::new();

    args.remove(0);

    for word in args.iter(){
        string.push_str(word);
        string.push(' ');
    }

    filesystem::write(format!("/{}", path), string);

    echo(args);
}
pub fn rm(path: String)
{
    filesystem::delete(format!("/{}", path));
    print!("Successfully deleted /{}.", path);
}
pub fn exit()
{
    for _ in 1..2{
        exit();
    }
}