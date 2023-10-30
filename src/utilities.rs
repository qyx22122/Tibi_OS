use alloc::string::String;
use core::panic;


pub struct HashNode{
    key: String,
    value: String,
}

pub fn get_value(hashmap: &[HashNode],key: String)-> String{
    for  node in hashmap.iter() {
        if node.key == key {
            return node.value.clone();
        }
    }
    panic!("Hashmap does not have this key: {}",key);
}
