use std::fs::File;
use std::io::{Write, BufWriter};
use log::info;
use crate::helper::{transform_u16_to_array_of_u8, transform_u32_to_array_of_u8, zstd_encode};
use crate::core::PackCompact;


/// Writer for u8
/// Output: 2 bytes (identifier), 4 byte (länge), 1(vec<u16>)
/// To add, threshold yes no -> bit
/// if not -> number threshold number (0 if nothing)
pub fn write_file(name: &str, vecc: &Vec<u8>, tresh: u16, out: &str, b: bool){
    let s2:Vec<&str> = name.split("/").collect();

    let s = s2.last().unwrap().clone();
    // this is the identifier
    let mut buff: Vec<u8> = vec![1,1];
    if b{
        buff.push(1);
    } else {
        buff.push(0);
    }
    info!("Bytes {}", vecc.len());
    // Length of the vector
    buff.extend(transform_u32_to_array_of_u8(vecc.len() as u32));
    // Add threshold
    buff.extend(transform_u16_to_array_of_u8(tresh));


    // Name
    let char_vec: Vec<char> = s.chars().collect();



    for c in char_vec.iter() {
        buff.push(c.clone() as u8);
    }

    for _x in 0..(64 - char_vec.len()){
        buff.push(0);
    }


    for x in vecc.iter(){
        buff.push(x.clone());
    }
    let buf2 = zstd_encode(&buff);
    let mut file = File::create([out, "bin", "zst"].join(".")).expect("Not able to write ");
    file.write_all(&buf2).expect("Not able to write ");

}

#[allow(dead_code)]
/// Just writing bytes to a file
pub fn writer_compress(buf: &Vec<u8>, filename: &str){
    let mut file = File::create(filename).expect("Not able to write ");
    file.write_all(&buf).expect("Not able to write ");
}



/// Just writing bytes to a file
pub fn writer_compress_zlib(buf: &Vec<u8>, filename: &str){
    let u = zstd_encode(buf);
    let mut file = File::create([filename, "zst"].join(".")).expect("Not able to write ");
    file.write_all(&u).expect("Not able to write ");
}


/// Writing normal pack file using the PackCompact structure
pub fn write_pack(pc: &PackCompact, filename: &str){
    let f = File::create([filename, "bin", "zst"].join(".")).expect("Unable to create file");
    let mut f = BufWriter::new(f);
    write!(f, "{}\t{}\t{}\t{}\n", "seq.pos", "node.id", "node.offset", "coverage").expect("Can not write file");

    let mut node = 0;
    for x in 0..pc.coverage.len(){
        if x == 0{
            write!(f, "{}\t{}\t{}\t{}\n", x, pc.node[x], node, pc.coverage[x]).expect("Can not write file");
        }else {
            if pc.node[x] == pc.node[x - 1] {
                node += 1;
                write!(f, "{}\t{}\t{}\t{}\n", x, pc.node[x], node, pc.coverage[x]).expect("Can not write file");
            } else {
                node = 0;
                write!(f, "{}\t{}\t{}\t{}\n", x, pc.node[x], node, pc.coverage[x]).expect("Can not write file");
            }
        }



    }
}





