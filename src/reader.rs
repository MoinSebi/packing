
use std::fs::File;
use std::io::{Read, Write};
use std::{fs, io};
use crate::helper::{byte_to_bitvec, byte_to_string, byte2u16, u8_u322, u8_u16, zstd_decode};
use crate::core::PackCompact;


pub struct ReaderBit {
    pub ty: bool,
    pub name: String,
    pub cc: Vec<bool>,
}


pub struct ReaderU16 {
    pub ty: bool,
    pub name: String,
    pub cc: Vec<u16>,
}

/// Get files byte by byte - Now exact
pub fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    // THIS IS A FUCKING JOKE
    f.read_exact(&mut buffer).expect("buffer overflow");

    let buf2 = zstd_decode(buffer);


    buf2
}


#[allow(dead_code)]
/// Reads buf to Vec<bool>
pub fn wrapper_bool(buffer: &Vec<u8>) -> Vec<ReaderBit>{
    // total length 73 + len
    let length = u8_u322(&buffer[3..7]);
    let oo = buffer.chunks((length + 73) as usize );
    eprintln!("Number of samples: {}", oo.len());
    let mut jo: Vec<ReaderBit> = Vec::new();
    for x in oo.into_iter(){

        let u = get_meta(x);
        eprintln!("");
        eprintln!("{}", u.3);
        io::stdout().flush().unwrap();
        let c = get_bin(x);
        jo.push(ReaderBit {name: u.3, ty: u.0, cc: c});
    }
    return jo

}


#[allow(dead_code)]
/// Reads buf to Vec<u16>
pub fn wrapper_u16(buffer: &Vec<u8>) -> Vec<ReaderU16>{
    // total length 73 + len
    let length = u8_u322(&mut &buffer[3..7]);
    let oo = buffer.chunks((length + 73) as usize );

    eprintln!("Number of samples: {}", oo.len());
    let mut jo: Vec<ReaderU16> = Vec::new();
    for x in oo.into_iter(){
        let u = get_meta(x);
        eprintln!("");
        eprintln!("{}", u.3);
        io::stdout().flush().unwrap();
        let c = get_u16(x);
        jo.push(ReaderU16 {name: u.3, ty: u.0, cc: c});
    }
    return jo

}

#[allow(dead_code)]
/// Get the meta data from the binary pack file (73 bytes)
/// Sequence/Node, length, thresh, name
pub fn get_meta(buffer: & [u8]) -> (bool, u32, u16, String){
    let cov = buffer[3];
    let length = u8_u322(&mut &buffer[3..7]);
    let thresh = u8_u16(&mut &buffer[7..9]);
    let name = byte_to_string(&mut &buffer[9..73]);


    (cov == 1, length, thresh, name)

}


/// Reads a binary file
/// Buff -> Vec<u32>
pub fn read_simple(filename: &str) -> Vec<u32>{
    let buf = get_file_as_byte_vec(filename);
    let chunks = buf.chunks(4);
    let mut vec_nodes: Vec<u32> = Vec::new();
    for x in chunks.into_iter(){
        vec_nodes.push(u8_u322(x));
    }
    return vec_nodes
}

/// Wrapper for meta + coverage combination
/// https://stackoverflow.com/questions/29445026/converting-number-primitives-i32-f64-etc-to-byte-representations
pub fn wrapper_meta(filename1: &str, filename2: &str) -> PackCompact{
    let nodes = read_simple(filename1);
    let cov = read_simple(filename2);
    let pc: PackCompact = PackCompact{node: nodes, coverage: cov, coverage_normalized: Vec::new()};
    pc
}




//___________________________________________________________
// Helper functions
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

#[cfg(test)]
mod reader {
    use crate::vg_parser::{parse_smart};
    use crate::writer::{writer_compress_zlib};

    #[test]
    fn pack_pack() {
        let k = parse_smart("testing/9986.100k.txt");
        let buf = k.compress_only_coverage();
        writer_compress_zlib(&buf, "testing/cov.test.zst");
    }
}










