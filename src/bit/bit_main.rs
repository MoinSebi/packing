use clap::ArgMatches;
use log::{info, warn};
use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::read_input;
use packing_lib::core::writer::writer_compress_zlib;
use packing_lib::normalize::convert_helper::Method;
use packing_lib::normalize::helper::{vec2binary, vec2binary_f32};
use std::process;

pub fn bit_main(matches: &ArgMatches) {
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

    let mut absolute_thresh: u16 = matches
        .value_of("absolute-threshold")
        .unwrap_or("0")
        .parse()
        .unwrap();
    let mut relative_thresh: f32 = matches.value_of("fraction").unwrap_or("0").parse().unwrap();
    let mut std: f32 = matches
        .value_of("standard-deviation")
        .unwrap_or("0")
        .parse()
        .unwrap();

    let method_string = matches.value_of("method").unwrap_or("nothing");
    let mut method = Method::from_str(method_string);
    let include_all = matches.is_present("non-covered");
    let want_sequence = !matches.is_present("node");

    if matches.is_present("absolute-threshold")
        && method == Method::Nothing
    {
        absolute_thresh = 0;
    }

    let real_thresh: f32;

    // Checking the output base (sequence, nodes) or pack file

    if !matches.is_present("absolute-threshold")
        && method == Method::Nothing
        && relative_thresh == 0.0
    {
        warn!("Nothing here");
        process::exit(0x0100);
    }

    if want_sequence && !pc.is_sequence {
        pc.calc_node_cov();
    }





    // Absolute threshold is adjusted is made with thresh
    if !matches.is_present("absolute-threshold") {
        real_thresh =
            PackCompact::get_threshold(&mut pc, include_all, relative_thresh, std, method);
    } else {
        real_thresh = absolute_thresh as f32;
        method = Method::Nothing;
        relative_thresh = 1.0;
        std = 0.0;
    }

    info!("New parameters");
    info!(
        "Feature: {}",
        if want_sequence { "node" } else { "sequence" }
    );
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
        info!("Number of samples: {}", pc.coverage.len());
        number_entries = pc.coverage.len();
        buffer = vec2binary(pc.coverage, &real_thresh);
    } else {
        info!("Number of samples: {}", pc.normalized_coverage.len());
        number_entries = pc.normalized_coverage.len();

        buffer = vec2binary_f32(pc.normalized_coverage, &real_thresh);
    }

    let mut bb = PackCompact::file_header(
        want_sequence,
        DataType::TypeBit,
        method,
        relative_thresh,
        std,
        real_thresh,
        number_entries as u32,
        &pc.name,
    );
    bb.extend(buffer);
    println!("{}", bb.len());

    writer_compress_zlib(&bb, matches.value_of("out").unwrap());
}
