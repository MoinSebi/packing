
use std::fs::File;
use std::io::{Read};
use std::{fs};
use bitvec::order::{Msb0};
use bitvec::vec::BitVec;
use byteorder::{BigEndian, ByteOrder};
use log::info;
use crate::helper::{byte_to_string, zstd_decode};
use crate::core::PackCompact;

// Use this as a library for reading the binary files

/// Reading th
pub struct ReaderBit {
    pub kind: bool,             // 0 = node, 1 == cov
    pub name: String,
    pub data: BitVec<u8, Msb0>,
}


pub struct ReaderU16 {
    pub kind: bool,              // 0 = node, 1 == cov
    pub name: String,
    pub data: Vec<u16>,
}

/// Read a file byte by byte (whole file)
///
/// Check if file is available and readable. Returns buffer
pub fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut file = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    // THIS IS A FUCKING JOKE
    file.read_exact(&mut buffer).expect("buffer overflow");

    let buffer2 = zstd_decode(buffer);


    buffer2
}


#[allow(dead_code)]
/// Reads buffer to Vector of ReaderBit
///
///
pub fn wrapper_bool(buffer: &Vec<u8>) -> Vec<ReaderBit>{
    // total length 73 + len
    let length = BigEndian::read_u32(& buffer[3..7]);
    let chunks = buffer.chunks((length + 73) as usize );
    eprintln!("Number of samples: {}", chunks.len());
    let mut result: Vec<ReaderBit> = Vec::new();

    for chunk in chunks.into_iter(){

        let (kind, _length, _thresh, name) = get_meta(chunk);
        info!("Name {}", name);
        let bv: BitVec<u8, Msb0> = BitVec::from_slice(&chunk[73..]);
        result.push(ReaderBit {name, kind, data: bv});
    }
    return result

}


#[allow(dead_code)]
/// Convert bytes to a Vector of ReaderU16
///
/// Iterate over each sample (
pub fn wrapper_u16(buffer: &Vec<u8>) -> Vec<ReaderU16>{
    // total length 73 + len
    let length = BigEndian::read_u32(& buffer[3..7]);
    let chunks = buffer.chunks((length + 73) as usize );

    eprintln!("Number of samples: {}", chunks.len());
    let mut result: Vec<ReaderU16> = Vec::new();
    for chunk in chunks.into_iter(){
        let (kind, _length, _thresh, name) = get_meta(chunk);
        info!("Name {}", name);
        let mut data = vec![0; chunk[73..].len()/2];
        BigEndian::read_u16_into(&chunk[73..], & mut data);
        result.push(ReaderU16 {name, kind, data});
    }
    return result

}

#[allow(dead_code)]
/// Get the meta data from the binary pack file (73 bytes)
/// Outputs sequence/Node, length, thresh, name
pub fn get_meta(buffer: & [u8]) -> (bool, u32, u16, String){
    let cov = buffer[3];
    let length = BigEndian::read_u32(& buffer[3..7]);
    let thresh = BigEndian::read_u16(& buffer[7..9]);
    let name = byte_to_string(&mut &buffer[9..73]);
    let name = name.trim_matches(char::from(0)).to_string();


    (cov == 1, length, thresh, name)

}


//--------------------------------------------------------------------------------------------------
//

/// Reads a binary file
/// Buff -> Vec<u32>
pub fn read_simple_u32(filename: &str) -> Vec<u32>{
    let buf = get_file_as_byte_vec(filename);
    let mut vec_nodes: Vec<u32> = vec![0; buf.len()/4];
    BigEndian::read_u32_into(& buf, & mut vec_nodes);

    return vec_nodes
}

pub fn read_simple_u16(filename: &str) -> Vec<u16> {
    let buf = get_file_as_byte_vec(filename);
    let mut vec_nodes: Vec<u16> = vec![0; buf.len() / 2];
    BigEndian::read_u16_into(&buf, &mut vec_nodes);

    return vec_nodes
}

/// Wrapper for meta + coverage combination
/// https://stackoverflow.com/questions/29445026/converting-number-primitives-i32-f64-etc-to-byte-representations
pub fn wrapper_meta(filename1: &str, filename2: &str) -> PackCompact{
    let nodes = read_simple_u32(filename1);
    let cov = read_simple_u16(filename2);
    let pc: PackCompact = PackCompact{node: nodes, coverage: cov, coverage_normalized: Vec::new(), node_coverage: Vec::new()};
    pc
}











