use std::collections::HashSet;
use crate::reader::{get_file_as_byte_vec, get_meta, read_simple, wrapper_bool, wrapper_u16};
use crate::helper::{u8_u16, u8_u322};

/// This is the same as read_exact, except if it reaches EOF it doesn't return
/// an error, and it returns the number of bytes read.
/// From: https://stackoverflow.com/questions/60951064/how-to-read-the-first-n-bytes-of-a-file-or-less-if-it-is-shorter
fn read_up_to(file: &mut impl std::io::Read, mut buf: &mut [u8]) -> Result<usize, std::io::Error> {
    let buf_len = buf.len();

    while !buf.is_empty() {
        match file.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(buf_len - buf.len())
}

/// Get the threshold from index (bytes)
pub fn get_thresh(filename: &str) -> u16{
    let size = u8_u16(&mut & get_file_as_byte_vec(filename)[7..9]);
    size
}


/// Information about the a index file
pub fn stats_index(filename: &str){
    let nodes = read_simple(filename);
    let nodes_hs: HashSet<u32> = nodes.iter().cloned().collect();
    println!("Number of nodes: {}", nodes_hs.len());
    println!("Number of entries: {}", nodes.len());

}



/// Compute statistics about the index file.
pub fn stats(filename: &str, exact: bool, check_all: bool) {
    let g: Vec<u8> = get_file_as_byte_vec(filename);
    let meta = get_meta(&g);
    let length = meta.1;
    let mut length_measured = 0;
    let mut all_length: Vec<usize> = Vec::new();
    let mut length: u32 = 0;
    let mut h = 0;
    if meta.2 != 0{

        let k = wrapper_bool(&g);
        all_length = k.iter().map(|x| x.data.len()).collect();
        length_measured = k[0].data.len();
        h = k.len();
    } else {
        let k = wrapper_u16(&g);
        all_length = k.iter().map(|x| x.data.len()).collect();
        length_measured = k[0].data.len();
        h = k.len();
    }

    let mut allgood = true;
    if check_all{
        for x in all_length.iter(){
             if x != &all_length[0]{
                 allgood = false;
             }
        }
    }

    if meta.2 != 0{
        length = meta.1*8
    } else {
        length = meta.1/2
    }
    println!("Number of elements: {}", h);
    if meta.0{
        println!("Threshold: Sequence");

    } else {
        println!("Threshold: Node");

    }
    println!("Threshold: {}", meta.2);
    println!("Bytes: {}", meta.1 );
    println!("Length: {}", length);
    if exact{
        println!("Length measured {}", length_measured);
    }
    if check_all{
        if allgood{
            println!("All length are the same.")
        } else {
            println!("ERROR! Not the same length. ")
        }
    }


}