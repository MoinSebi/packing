use crate::convert::helper::transform_u32_to_array_of_u8;
use gfa_reader::NCGfa;
use log::info;

/// Read GFA and get nodes + sequences
/// Same order than VG --> sort(node, sequence)
pub fn make_index(filename: &str) -> Vec<u8> {
    let mut graph: NCGfa<()> = NCGfa::new();
    graph.parse_gfa_file(filename, false);

    let mut nodes: Vec<_> = graph.nodes.iter().map(|u| (u.id, u.seq.len())).collect();
    println!("Node length {}", nodes.len());
    nodes.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut buf = Vec::new();
    let mut count = 0;
    for x in nodes.iter() {
        for _y in 0..x.1 {
            count += 1;
            buf.extend(transform_u32_to_array_of_u8(x.0));
        }
    }
    info!("Total length: {}", count);
    buf
}
