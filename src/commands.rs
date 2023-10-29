use crate::println;

pub fn help()
{
    let s = stringify!(terminal::write_terminal);

    println!("{}", s);
}