use std::collections::HashSet;
use crate::reader::{get_file_as_byte_vec, get_meta, read_simple_u32, wrapper_bool, wrapper_u16};

/// Information about the a index file
pub fn stats_index(filename: &str){
    let nodes = read_simple_u32(filename);
    let nodes_hs: HashSet<u32> = nodes.iter().cloned().collect();
    println!("Number of nodes: {}", nodes_hs.len());
    println!("Number of entries: {}", nodes.len());

}



/// Compute statistics about the index file.
///
/// TODO
/// Mean and median coverage of all and covered nodes if single file
/// Split files after cat
pub fn stats(filename: &str, exact: bool, check_all: bool) {
    let g: Vec<u8> = get_file_as_byte_vec(filename);
    let meta = get_meta(&g);
    let length_measured;
    let all_length: Vec<usize>;
    let length: u32;
    let h ;
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