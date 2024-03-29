use log::info;
use packing_lib::convert::convert_helper::Method;
use packing_lib::core::reader::{get_meta, read_index, unpack_zstd_to_byte};
use std::collections::HashSet;

/// Information about the a index file
pub fn info_index(filename: &str) {
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
pub fn info_compressed(filename: &str) {
    let g: Vec<u8> = unpack_zstd_to_byte(filename);
    let meta = get_meta(&g);

    info!("Entry type: {}", if meta.0 { "Sequence" } else { "Node" });
    info!(
        "Data type: {}",
        if meta.1 { "Binary" } else { "Value (u16)" }
    );
    info!("Method: {}", Method::from_u8(meta.2).to_string());
    info!("Relative threshold: {}", meta.3);
    info!("Real threshold: {}", meta.4);
    info!("Bytes: {}", meta.5);
    info!("Entries: {}", meta.6);
    info!("Header bytes: {}", 77);
    info!("Name: {}", meta.7);
}
