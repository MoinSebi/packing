
use std::fs::File;
use std::io::Read;
use std::fs;
use crate::helper::{byte_to_bitvec, byte_to_string, byte2u16, transform_u32_to_array_of_u8, u8_u32, u8_u322, u8_u16};
use std::error::Error;
use crate::core::PackCompact;

pub struct R2 {
    pub ty: String,
    pub name: String,
    pub cc: Vec<bool>,
}

pub struct R3 {
    pub ty: String,
    pub name: String,
    pub cc: Vec<u16>,
}

/// Get files byte by byte
pub fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    println!("meta len {:?}", metadata.len());
    let mut buffer = vec![0; metadata.len() as usize];
    // THIS IS A FUCKING JOKE
    f.read_exact(&mut buffer).expect("buffer overflow");



    buffer
}


pub fn wrapper_bool(buffer: &Vec<u8>) -> Vec<R2>{
    // total length 73 + len
    let length = u8_u322(&buffer[3..7]);
    println!("{}", length);
    let oo = buffer.chunks((length + 73) as usize );
    println!("How many samples: {}", oo.len());
    let mut jo: Vec<R2> = Vec::new();
    for x in oo.into_iter(){
        println!("len is {}", x.len());
        let u = get_meta(x);
        let c = get_bin(x);
        jo.push(R2 {name: u.3, ty: "dunno".to_string(), cc: c});
    }
    return jo

}

pub fn wrapper_u16(buffer: &Vec<u8>) -> Vec<R3>{
    // total length 73 + len
    let length = u8_u322(&mut &buffer[3..7]);
    println!("{}", length);
    let oo = buffer.chunks((length + 73) as usize );

    println!("How many samples: {}", oo.len());
    let mut jo: Vec<R3> = Vec::new();
    for x in oo.into_iter(){
        println!("len is {}", length + 73);
        let u = get_meta(x);
        let c = get_u16(x);
        jo.push(R3 {name: u.3, ty: "dunno".to_string(), cc: c});
    }
    return jo

}

/// Get the meta data from the binary pack file (73 bytes)
pub fn get_meta(buffer: & [u8]) -> (bool, u32, u16, String){
    let cov = buffer[3];
    let length = u8_u322(&mut &buffer[3..7]);
    let thresh = u8_u16(&mut &buffer[7..9]);
    let name = byte_to_string(&mut &buffer[9..73]);


    (cov == 1, length, thresh, name)

}

/// Get binary information from file
pub fn get_bin(buffer: & [u8]) -> Vec<bool>{
    let mut j: Vec<bool> = Vec::new();
    for x in buffer[73..].iter(){
        j.extend(byte_to_bitvec(&x));
    }
    j
}

/// Get coverage from file
pub fn get_u16(buffer: & [u8]) -> Vec<u16>{
    let mut j: Vec<u16> = Vec::new();
    let g = buffer[73..].chunks(2);

    for x in g.into_iter(){

        j.push(byte2u16(& x));
    }
    j
}






