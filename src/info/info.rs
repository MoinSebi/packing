use std::collections::HashSet;
use log::info;
use crate::convert::convert_helper::Method;
use crate::core::reader::{get_meta, read_index, unpack_zstd_to_byte};

/// Information about the a index file
pub fn stats_index(filename: &str){
    let nodes = read_index(filename);
    let nodes_hs: HashSet<u32> = nodes.iter().cloned().collect();
    info!("Number of nodes: {}", nodes_hs.len());
    info!("Number of entries: {}", nodes.len());

}



/// Compute statistics about the index file.
///
/// TODO
/// Mean and median coverage of all and covered nodes if single file
/// Split files after cat
pub fn stats(filename: &str) {
    let g: Vec<u8> = unpack_zstd_to_byte(filename);
    let meta = get_meta(&g);


    info!("Entry type: {}", if meta.0  {"Node"} else {"Sequence"});
    info!("Data type: {}", if meta.1  {"Binary"} else {"Value (u16)"});
    info!("Method: {}", Method::from_u8(meta.2).to_string());
    info!("Relative threshold: {}", meta.3);
    info!("Real threshold: {}", meta.4);
    info!("Bytes: {}", meta.5 );
    info!("Entries: {}", meta.6);
    info!("Header bytes: {}", 77);
    info!("Name: {}", meta.7);


}