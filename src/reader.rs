
use std::fs::File;
use std::io::Read;
use std::fs;
use std::convert::TryInto;
use crate::core::read_in2;


///
/// TODO
/// Read to data structure struct on main
/// in gfa2bin main to vector
///
/// TODO today
/// mapping 27 genomes to graph dont strore bam
/// store gaf
/// genome mapping
/// 200  read sets
/// cluster
/// check mapping yeast
/// basic GEMMA RUN
///
/// what to show
/// project layout
/// explain the extended reference princibale
/// cooler gwas aufhaenger
/// double smoothing
/// qq plot
/// flc lets go
/// show difference to snps
/// can i find nested variation, kmer comparsion
/// yes i can
/// now find cool stuff

/// get shit byte by byte
///
pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");


    buffer
}

pub fn wrapper2(buffer: &Vec<u8>){
    // total length 72 + len
    let length = read_be_u32(&mut &buffer[2..6]);
    let oo = buffer.chunks((length + 72) as usize );
    let mut jo: Vec<read_in2> = Vec::new();
    for x in oo.into_iter(){
        let u = get_meta(x);
        let c = get_bin(x);
        jo.push(read_in2{name: u.2, ty: "dunno".to_string(), cc: c});
    }

}



/// get the meta data
/// see definition
pub fn get_meta(buffer: & [u8]) -> (u32, u16, String){
    let length = read_be_u32(&mut &buffer[2..6]);
    let thresh = read_be_u16(&mut &buffer[6..8]);
    let name = byte_to_string(&mut &buffer[8..72]);
    (length, thresh, name)

}

pub fn get_bin(buffer: & [u8]) -> Vec<bool>{
    let mut j: Vec<bool> = Vec::new();
    for x in buffer[72..].iter(){
        j.extend(byte_to_bitvec(&x));
    }
    j
}





fn byteitin2(buffer: Vec<u8>, b: bool){
    let mut j = Vec::new();
    for x in buffer.iter(){
        j.extend(byte_to_bitvec(x));

    }
    // return j
}

fn byte_to_bitvec(buf: &u8) -> Vec<bool>{
    let mut h: Vec<bool> = Vec::new();
    let mut n = buf.clone();
    while (n > 0){
        h.push((n%2)  == 1);
        n = n/2
    }
    for x in 0..(8-h.len()){
        h.insert(0, false);
    }
    h
}


fn byte_to_u16vec(buf: &mut [u8]) -> Vec<u16>{
    let mut h: Vec<u16> = Vec::new();
    let mut j: Vec<&[u8]> = buf.chunks(2).collect();
    for x in j{
        h.push(test1(&mut &x));
    }
    h
}

fn read_be_u32(input: & mut &[u8]) -> u32 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    u32::from_be_bytes(int_bytes.try_into().unwrap())
}

fn read_be_u16(input: &mut &[u8]) -> u16 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
    *input = rest;
    u16::from_be_bytes(int_bytes.try_into().unwrap())
}


fn byte_to_string(input: &[u8]) -> String {
    let mut o = "".to_string();
    for x in input.iter(){
        o.push(x.clone() as char);
    }
    return o
}

fn test1(vector: &[u8]) -> u16{
    let number = ((vector[0] as u16) << 8) | vector[1] as u16;
    number
}
