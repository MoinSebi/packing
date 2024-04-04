use clap::ArgMatches;
use log::{debug, info, warn};
use packing_lib::convert::convert_helper::Method;
use packing_lib::convert::helper::{normalize_u16_f32, vec2binary, vec_u16_to_u8,
};
use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::read_input;
use packing_lib::core::writer::{writer_compress, writer_compress_zlib};
use std::process;

pub fn bit_main(matches: &ArgMatches) {
    let (mut pc, index_present) = read_input(matches);
    if pc.is_binary == DataType::TypeBit {
        warn!("You loaded a presence/absence file. You are not able to further convert it.");
        process::exit(0x0100);
    }

    // you need the index to convert
    if !index_present {
        warn!("There is no index file.");
        process::exit(0x0100);
    }

    // If it is binary, you are not able to convert anything
    if !pc.coverage.is_empty() || !pc.normalized_coverage.is_empty() {
        warn!("The data is empty");
        process::exit(0x0100);
    }

    // If name is set as argument, replace filename
    if matches.is_present("name") {
        pc.name = matches.value_of("name").unwrap().to_string();
    }

    let mut absolute_thresh: u16 = matches
        .value_of("absolute threshold")
        .unwrap_or("0")
        .parse()
        .unwrap();
    let mut relative_thresh: f32 = matches
        .value_of("relative threshold")
        .unwrap_or("0")
        .parse()
        .unwrap();
    let std: f32 = matches.value_of("std").unwrap_or("0").parse().unwrap();

    let method_string = matches.value_of("method").unwrap_or("nothing");
    let mut method = Method::from_str(method_string);
    let include_all = matches.is_present("non-covered");
    let node = matches.is_present("node");

    if !matches.is_present("absolute threshold")
        && method == Method::Nothing
        && matches.is_present("relative threshold")
    {
        method = Method::Percentile;
    }

    let real_thresh: f32;
    let mut sequence_out = matches.is_present("node");

    // Checking the output base (sequence, nodes) or pack file
    info!(
        "Feature: {}",
        if sequence_out { "node" } else { "sequence" }
    );
    if matches.is_present("relative threshold") && relative_thresh == 0.0 {
        warn!("Relative threshold is 0");
        process::exit(0x0100);
    }
    if !matches.is_present("absolute threshold")
        && method == Method::Nothing
        && relative_thresh == 0.0
    {
        warn!("Nothing here");
        process::exit(0x0100);
    }

    if matches.is_present("node") {
        pc.calc_node_cov();
    }

    // Absolute threshold is adjusted is made with thresh
    if matches.is_present("absolute threshold") {
        real_thresh =
            PackCompact::get_threshold(&mut pc, include_all, relative_thresh, std, method);
    } else {
        real_thresh = absolute_thresh as f32;
    }
    info!("Method: {}", method.to_string());
    info!("Absolute threshold: {}", absolute_thresh);
    info!("Relative threshold: {}", relative_thresh);
    info!("Include all: {}", include_all);
    info!("Standard deviation {}", std);
    info!("Real threshold: {}", real_thresh);

    // The vector we work with


    let number_entries = pc.coverage.len();
    let buffer: Vec<u8> = vec2binary(pc.coverage, &real_thresh);
    let mut bb = PackCompact::file_header(
        sequence_out,
        DataType::TypeBit,
        method,
        relative_thresh,
        std,
        real_thresh as f32,
        number_entries as u32,
        &pc.name,
    );
    bb.extend(buffer);
    println!("{}", bb.len());

    if matches.is_present("non-compressed") {
        writer_compress(&bb, matches.value_of("out").unwrap());
    } else {
        writer_compress_zlib(&bb, matches.value_of("out").unwrap());
    }
}
