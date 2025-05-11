use gfa_reader::Gfa;
use log::info;
use packing_lib::normalize::helper::transform_u32_to_array_of_u8;

/// Read GFA and get nodes + sequences
/// Same order than VG --> sort(node, sequence)
pub fn make_index(filename: &str) -> Vec<u8> {
    let mut graph: Gfa<u32, (), ()> = Gfa::parse_gfa_file_multi(filename, 1);

    let mut nodes: Vec<_> = graph.segments.iter().map(|u| (u.id, u.sequence.get_len())).collect();
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
