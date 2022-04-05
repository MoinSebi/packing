use std::io::{Write, Read};
use bitvec::bitvec;
use bitvec::order::{Lsb0, Msb0};
use bitvec::vec::BitVec;
use byteorder::{BigEndian, ByteOrder};
use log::info;



/// u16 vector to u8 vector
pub fn vec_u16_u8(vecc: &Vec<u16>) -> Vec<u8>{
    let mut buff2: Vec<u8> = vec![0; vecc.len()*2];
    BigEndian::write_u16_into(vecc, & mut buff2);

    buff2
}




/// Mean of a U16 vector and returns a u16
pub fn mean_vec_u16_u16(val: &Vec<u16>) -> u16{
    let su: u16= val.iter().sum();
    let j:u16  = ((su as f64)/(val.len() as f64)).round() as u16;
    j
}


/// Mean of a vector
pub fn median(numbers: & Vec<u16>) -> u16 {
    let mut num = numbers.clone();
    num.sort();
    let mid = num.len() / 2;
    num[mid]
}





/// u32 -> 4xu8
pub fn transform_u32_to_array_of_u8(x:u32) -> [u8;4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    let array = [b1, b2, b3, b4];
    return array
}




/// Byte to string
///
/// Alternativ to  std::str::from_utf8
/// https://doc.rust-lang.org/std/str/fn.from_utf8.html
pub fn byte_to_string(input: &[u8]) -> String {
    let mut o = "".to_string();
    for x in input.iter(){

        o.push(x.clone() as char);

    }
    return o
}




// pub fn read_be_u32(input: & mut &[u8]) -> u32 {
//     let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
//     *input = rest;
//     u32::from_be_bytes(int_bytes.try_into().unwrap())
// }
//
// pub fn read_be_u16(input: &mut &[u8]) -> u16 {
//     let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
//     *input = rest;
//     u16::from_be_bytes(int_bytes.try_into().unwrap())
// }




pub fn normalizeing(vecc: Vec<u16>, absolute_thresh: &u16) -> Vec<u16>{

    let mut ww :  Vec<u16> = Vec::new();
    for x in vecc.iter(){
        ww.push(((*x as f64)/(*absolute_thresh as f64)).round() as u16)
    }
    ww
}


pub fn bitbit(vecc: Vec<u16>, absolute_thresh: &u16) -> Vec<u8>{

    let mut bv: BitVec<u8, Msb0> = BitVec::new();
    for x in vecc.iter(){
        if x >= absolute_thresh{
            bv.push(true)
        } else {
            bv.push(false)
        }
    }
    let o = bv.into_vec();
    o
}

pub fn make_header(node: &bool, thresh: &u16, bytes: &Vec<u8>, name: &str) -> Vec<u8>{
    let mut buff: Vec<u8> = vec![1,1];
    if !node{
        buff.push(1);
    } else {
        buff.push(0);
    }
    info!("Bytes {}", bytes.len());
    // Length of the vector
    let mut buff2 = vec![0; 4];
    BigEndian::write_u32(& mut buff2, bytes.len() as u32);
    buff.extend(buff2);
    // Add threshold
    let mut buff2 = vec![0; 2];
    BigEndian::write_u16(& mut buff2, thresh.clone());
    buff.extend(buff2);
    // Name
    let char_vec: Vec<char> = name.chars().collect();



    for c in char_vec.iter() {
        buff.push(c.clone() as u8);
    }

    for _x in 0..(64 - char_vec.len()){
        buff.push(0);
    }
    return buff;
}


//-------------------------------------------------------------------------------------------------------
// Compression

/// Helper function for zstd encoder
/// /// https://docs.rs/zstd/0.1.9/zstd/struct.Endcoder.html
pub fn zstd_encode(v: &Vec<u8>) -> Vec<u8>{
    let mut e = zstd::Encoder::new(Vec::new(), 0).unwrap();
    e.write_all(v).expect("Not working");
    let com = e.finish().unwrap();
    return com
}


/// Helper function for zstd decoder
/// https://docs.rs/zstd/0.1.9/zstd/struct.Decoder.html
pub fn zstd_decode(bytes: Vec<u8>) -> Vec<u8> {
    let mut gz = zstd::Decoder::new(&bytes[..]).unwrap();
    let mut k: Vec<u8> = Vec::new();
    gz.read_to_end(& mut k).expect("Decoding does not work");
    k
}









