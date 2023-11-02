use alloc::string::{String, ToString};
use alloc::format;
use alloc::vec::Vec;

use crate::println;

static mut KEYS: Vec<String> = Vec::new();
static mut VALUES: Vec<String> = Vec::new();

pub fn add(key:String, value:String)
{
    unsafe
    {
        if KEYS.contains(&key)
        {
            let index = KEYS.iter().position(|x| x == &key).unwrap();
            VALUES[index] = value;
            return;
        }

        KEYS.push(key);
        VALUES.push(value);
    }
}
pub fn get(key:String)->String
{
    unsafe
    {
        if KEYS.contains(&key)
        {
            let index = KEYS.iter().position(|x| x == &key).unwrap();
            return VALUES[index].clone();
        }
        
        return format!("[ERROR] Couldn't find {} in filesystem.", key);
    }
}
pub fn remove(key:String)
{
    unsafe
    {
        if KEYS.contains(&key)
        {
            let index = KEYS.iter().position(|x| x == &key).unwrap();
            KEYS.remove(index);
            VALUES.remove(index);
            return;
        }
        
        panic!("[ERROR] Couldn't find {} in filesystem.", key);
    }
}

pub fn init()
{
    add("/".to_string(), "".to_string());
    unsafe
    {
        println!("{:?}", KEYS);
    }
}

pub fn read(path:String) -> String
{
    return get(path);
}
pub fn write(path:String, value:String)
{
    add(path, value);
}
pub fn delete(path:String)
{
    remove(path);
}
pub fn exists(path:String) -> bool
{
    if get(path.clone()) == format!("[ERROR] Couldn't find {} in filesystem.",path)
    {
        return false;
    }
    return true;
}