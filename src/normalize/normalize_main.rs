use crate::core::reader::read_input;
use crate::core::writer::{writer_compress, writer_compress_zlib};
use crate::normalize::convert_helper::Method;
use crate::normalize::helper::{
    normalize_f32_f32, normalize_u16_f32, vec2binary, vec2binary_f32, vec_f32_to_u8, vec_u16_to_u8,
};
use clap::ArgMatches;
use log::{debug, info, warn};

use crate::core::core::{DataType, PackCompact};

use std::process;

pub fn normalize_main(matches: &ArgMatches) {
    let (mut pc, index_present) = read_input(matches);

    // Check if the right data
    if pc.data_type == DataType::TypeBit {
        warn!("You loaded a presence/absence file. You are not able to further normalize it.");
        process::exit(0x0100);
    }

    // you need the index to normalize
    if !index_present {
        warn!("There is no index file.");
        process::exit(0x0100);
    }

    // If it is binary, you are not able to normalize anything
    if pc.coverage.is_empty() && pc.normalized_coverage.is_empty() {
        warn!("The data is empty");
        process::exit(0x0100);
    }

    // If name is set as argument, replace filename
    if matches.is_present("name") {
        pc.name = matches.value_of("name").unwrap().to_string();
    }

    let absolute_thresh: u16 = matches
        .value_of("absolute threshold")
        .unwrap_or("0")
        .parse()
        .unwrap();
    let relative_thresh: f32 = matches
        .value_of("fraction")
        .unwrap_or("1.0")
        .parse()
        .unwrap();
    let std: f32 = matches.value_of("standard-deviation").unwrap_or("0").parse().unwrap();

    let method_string = matches.value_of("method").unwrap_or("nothing");
    let mut method = Method::from_str(method_string);
    let include_all = matches.is_present("non-covered");
    let want_sequence = !matches.is_present("node");

    if !matches.is_present("absolute threshold")
        && method == Method::Nothing
        && matches.is_present("fraction")
    {
        warn!("The data is empty");
        process::exit(0x0100);
    }

    let real_thresh: f32;

    // Checking the output base (sequence, nodes) or pack file

    if matches.is_present("fraction") && relative_thresh == 0.0 && std == 0.0{
        warn!("Relative threshold is 0");
        process::exit(0x0100);
    }

    if matches.is_present("node") && pc.is_sequence {
        pc.calc_node_cov();
    }

    // Absolute threshold is adjusted is made with thresh
    if !matches.is_present("absolute-threshold") {
        real_thresh =
            PackCompact::get_threshold(&mut pc, include_all, relative_thresh, std, method);
    } else {
        real_thresh = absolute_thresh as f32;
    }
    info!("New parameters");
    info!("Feature: {}", if want_sequence { "sequence" } else { "node" });
    info!("Method: {}", method.to_string());
    info!("Absolute threshold: {}", absolute_thresh);
    info!("Relative threshold: {}", relative_thresh);
    info!("Include all: {}", include_all);
    info!("Standard deviation {}", std);
    info!("Real threshold: {}", real_thresh);

    // The vector we work with

    let mut number_entries = 0;
    let mut buffer = Vec::new();
    if pc.normalized_coverage.is_empty() {
        buffer.extend(normalize_u16_f32(&pc.coverage, &real_thresh));
        number_entries = pc.coverage.len();
    } else {
        buffer.extend(normalize_f32_f32(&pc.normalized_coverage, &real_thresh));
        number_entries = pc.normalized_coverage.len();
    }
    info!("Number of entries: {}", number_entries);

    let mut bb = PackCompact::file_header(
        want_sequence,
        DataType::TypeF32,
        method,
        relative_thresh,
        std,
        real_thresh,
        number_entries as u32,
        &pc.name,
    );
    bb.extend(buffer);

    writer_compress_zlib(&bb, matches.value_of("out").unwrap());
}
