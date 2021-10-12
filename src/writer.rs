use std::fs::File;
use std::io::Write;
use crate::helper::{transform_u16_to_array_of_u8, transform_u32_to_array_of_u8};





/// Write node to binary
/// genereal writer for u8
/// Output: 2 bytes (identifier), 4 byte (länge), 1(vec<u16>)
/// To add, threshold yes no -> bit
/// if not -> number threshold number (0 if nothing)
pub fn write_file(name: &String, vecc: &Vec<u8>, tresh: u16, out: &str){
    let s2:Vec<&str> = name.split("/").collect();

    let s = s2.last().unwrap().clone();
    // this is the identifier
    let mut buff: Vec<u8> = vec![1,1];

    // Length of the vector
    buff.extend(transform_u32_to_array_of_u8(vecc.len() as u32));
    // Add threshold
    buff.extend(transform_u16_to_array_of_u8(tresh));


    // Name
    let char_vec: Vec<char> = s.chars().collect();



    for c in char_vec.iter() {
        buff.push(c.clone() as u8);
    }

    for x in 0..(64 - char_vec.len()){
        buff.push(0);
    }


    for x in vecc.iter(){
        buff.push(x.clone());
    }
    let mut file = File::create([out, "bin"].join(".")).expect("Not able to write ");
    file.write_all(&buff).expect("Not able to write ");

}