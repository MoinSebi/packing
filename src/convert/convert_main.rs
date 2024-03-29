use crate::convert::convert_helper::{Method};
use crate::convert::helper::{
    get_real_threshold, make_header, normalize_u16_u16, vec2binary, vec_u16_to_u8,
};
use crate::core::reader::read_input;
use crate::core::writer::{writer_compress, writer_compress_zlib};
use clap::ArgMatches;
use log::{debug, info, warn};

use std::process;


pub fn convert_main(matches: &ArgMatches) {
    let (mut pc, index_present) = read_input(matches);

    // you need the index to convert
    if !index_present {
        warn!("There is no index file.");
        process::exit(0x0100);
    }

    // If it is binary, you are not able to convert anything
    if !pc.bin_coverage.is_empty() {
        warn!("You loaded a presence/absence file. You are not able to further convert it.");
        process::exit(0x0100);
    }

    // If name is set as argument, replace filename
    if matches.is_present("name") {
        pc.name = matches.value_of("name").unwrap().to_string();
    }

    // Check if there is data
    if pc.coverage.is_empty() {
        info!("There is a problem with the input files. Run 'packing info' on your file.");
        info!("[-h, --help] for help information");
        process::exit(0x0100);
    } else {
        info!("Name is {}", pc.name)
    }

    let normalize = matches.is_present("normalize");
    let absolute_thresh: u16 = matches
        .value_of("absolute threshold")
        .unwrap_or("0")
        .parse()
        .unwrap();
    let relative_thresh: u16 = matches
        .value_of("relative threshold")
        .unwrap_or("0")
        .parse()
        .unwrap();

    let method_string = matches.value_of("method").unwrap_or("nothing");
    let mut method = Method::from_str(method_string);
    let include_all = matches.is_present("non-covered");
    let compress = matches.is_present("compress");
    let node = matches.is_present("node");

    if node && !pc.is_sequence {
        info!("Input is node-based, output should be sequence. This does not work");
        process::exit(0x0100)
    }

    let bin = !matches.is_present("normalization") || !matches.is_present("compress");

    if compress {
        method = Method::Nothing;
    }

    if !matches.is_present("absolute threshold")
        && method == Method::Nothing
        && matches.is_present("relative threshold")
    {
        method = Method::Percentile;
    }

    info!("Method: {}", method.to_string());
    info!("Absolute threshold: {}", absolute_thresh);
    info!("Relative threshold: {}", relative_thresh);
    info!("Include all: {}", include_all);
    info!("Normalize: {}", normalize);

    let real_thresh: f64;

    // Checking the output base (sequence, nodes) or pack file
    let sequence_out = matches.is_present("node");

    info!(
        "Feature: {}",
        if sequence_out { "node" } else { "sequence" }
    );

    // Write the pack
    if matches.is_present("output-pack") {
        debug!("Writing pack file");
        pc.write_pack(matches.value_of("out").unwrap());
        process::exit(0x0100);
    }

    if matches.is_present("relative threshold") && relative_thresh == 0 {
        warn!("Relative threshold is 0");
        process::exit(0x0100);
    }
    if absolute_thresh == 0 && method == Method::Nothing && relative_thresh == 0 {
        warn!("Relative threshold is 0");
        process::exit(0x0100);
    }

    if matches.is_present("node") {
        pc.calc_node_cov();
    }

    // Absolute threshold is adjusted is made with thresh
    if matches.is_present("absolute threshold") {
        real_thresh = get_real_threshold(&mut pc, include_all, relative_thresh, method);
    } else {
        real_thresh = absolute_thresh as f64;
    }
    info!("Real threshold: {}", real_thresh);

    // The vector we work with
    if normalize {
        pc.coverage = normalize_u16_u16(&pc.coverage, &real_thresh);
    }


    let number_entries = pc.coverage.len();
    let buffer: Vec<u8>;
    if bin {
        buffer = vec2binary(pc.coverage, &real_thresh);
    } else {
        buffer = vec_u16_to_u8(&pc.coverage);
    }
    let mut bb = make_header(
        sequence_out,
        bin,
        method,
        relative_thresh,
        &(real_thresh as u16),
        number_entries as u32,
        &pc.name,
    );
    bb.extend(buffer);
    if matches.is_present("non-compressed") {
        writer_compress(&bb, matches.value_of("out").unwrap());
    } else {
        writer_compress_zlib(&bb, matches.value_of("out").unwrap());
    }
}
