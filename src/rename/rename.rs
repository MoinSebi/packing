use crate::reader::get_file_as_byte_vec;
use crate::writer::writer_compress_zlib;

pub fn test(filename: &str, name: String, filename2: &str){
    let char_vec: Vec<char> = name.chars().collect();

    let mut namev = Vec::new();
    for c in char_vec.iter() {
        namev.push(c.clone() as u8);
    }
    for _x in 0..(64 - char_vec.len()){
        namev.push(20);
    }


    let buf: Vec<u8> = get_file_as_byte_vec(filename);
    let mut bb: Vec<u8> = Vec::new();
    bb.extend(&buf[..9]);
    bb.extend(&namev);
    bb.extend(&buf[73..]);
    writer_compress_zlib(&bb, filename2);


}

