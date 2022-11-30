use std::collections::HashSet;
use log::info;
use crate::reader::{get_file_as_byte_vec, get_meta, read_simple_u32, wrapper_bool, wrapper_u16};

/// Information about the a index file
pub fn stats_index(filename: &str){
    let nodes = read_simple_u32(filename);
    let nodes_hs: HashSet<u32> = nodes.iter().cloned().collect();
    info!("Number of nodes: {}", nodes_hs.len());
    info!("Number of entries: {}", nodes.len());

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
    info!("Number of elements: {}", h);
    if meta.0{
        info!("Threshold: Sequence");

    } else {
        info!("Threshold: Node");

    }
    info!("Name: {}", meta.3);
    info!("Threshold: {}", meta.2);
    info!("Bytes: {}", meta.1 );
    info!("Length: {}", length);
    if exact{
        info!("Length measured {}", length_measured);
    }
    if check_all{
        if allgood{
            info!("All length are the same.")
        } else {
            info!("ERROR! Not the same length. ")
        }
    }


}