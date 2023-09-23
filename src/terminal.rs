use crate::println;
use crate::print;
use crate::remove;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Terminal
{
    column_pos: u8
}
impl Terminal
{
    
    fn write_terminal_line()
    {
        print!("/> ");
    }
    pub const fn new() -> Terminal {
        return Terminal { column_pos: 0 };
    }

    pub fn init(&self)
    {
        println!("Welcome to the Terminal...");
        Self::write_terminal_line();
    }

    pub fn write_terminal(& mut self, mut ch: char)
    {
        if ch == '\x08'
        {
            if self.column_pos > 0
            {
                remove!();

                self.column_pos-=1;
            }
            return;
        }

        if ch == '\t'
        {
            print!(" ");
            self.column_pos+=1;
            ch = ' ';
        }
        
        print!("{}", ch);
        self.column_pos += 1;


        if ch == '\n'
        {
            Self::write_terminal_line();
            self.column_pos = 0;
        }
    }
}