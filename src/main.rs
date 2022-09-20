
mod core;
mod vg_parser;
mod helper;
mod writer;
mod reader;
mod index;
mod info;


use clap::{App, AppSettings, Arg};
use crate::vg_parser::{parse_smart};
use crate::writer::{write_pack, writer_compress_zlib};
use crate::helper::{vec_u16_u8, normalizing, vec2binary, make_header};
use std::{ process};
use crate::core::PackCompact;
use std::path::Path;
use chrono::Local;
use env_logger::{Builder, Target};
use log::{debug, info, LevelFilter, warn};
use std::io::Write;
use packing_lib::rename::rename::test;
use crate::index::index_main::make_index;
use crate::info::info::stats_index;
use crate::reader::wrapper_meta;

fn main() {
    let matches = App::new("panSV")
        .version("0.1.0")
        .author("Sebastian V")
        .about("packing")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::new("verbose")
            .short('v')
            .about("-v = DEBUG | -vv = TRACE")
            .takes_value(true)
            .default_missing_value("v1")
            .global(true))
        .arg(Arg::new("quiet")
            .short('q')
            .about("No messages")
            .global(true))


        .subcommand(App::new("info")
            .about("Information about index or binary files (not compressed pack)")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)
            .help_heading("Input options")

            .arg(Arg::new("binary")
                .short('b')
                .long("binary")
                .about("Information about the binary")
                .takes_value(true))
            .arg(Arg::new("index")
                .short('i')
                .long("index")
                .about("Information about the index")
                .takes_value(true))
            .help_heading("Testing options")

            .arg(Arg::new("all")
                .short('a')
                .long("all")
                .about("Check all entries (for concatenated index)")))


        .subcommand(App::new("index")
            .about("Index a graph (gfa or VG pack)")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)
            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .display_order(1)
                .short('g')
                .long("gfa")
                .about("gfa for index")
                .takes_value(true))
            .arg(Arg::new("pack")
                .short('p')
                .long("pack")
                .about("pack format after alignment")
                .takes_value(true))
            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file")
                .takes_value(true)
                .required(true)))


        .subcommand(App::new("rename")
            .about("Rename compressed data")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)
            .arg(Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .about("Compressed input")
                .takes_value(true))
            .arg(Arg::new("output")
                .short('o')
                 .long("output")
                 .about("Output")
                 .required(true)
                 .takes_value(true))
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .required(true)
                .about("New name")
                .takes_value(true)))

        .subcommand(App::new("convert")
            .about("Convert VG PACK format for a compact index structure (partially reversible)")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)
            // Input
            .help_heading("Input options")

            .arg(Arg::new("pack")
                .short('p')
                .long("pack")
                .about("vg pack file")
                .takes_value(true))
            .arg(Arg::new("index")
                .short('i')
                .long("index")
                .about("Index file from 'packing index'")
                .takes_value(true))
            .arg(Arg::new("compressed pack")
                .long("compressed")
                .short('c')
                .about("Compressed pack file (only sequence). Original can only be accessed if the file is not normalized.")
                .takes_value(true))


            .help_heading("Normalization parameters")
            // Modification
            .arg(Arg::new("relative threshold")
                .short('r')
                .long("threshold")
                .about("Percentile (can be combined with 'normalize' flag")
                .takes_value(true))
            .arg(Arg::new("absolute threshold")
                .short('a')
                .long("absolute threshold")
                .about("Presence-absence according to absolute threshold")
                .takes_value(true))
            // If you normalize, pls use me
            .arg(Arg::new("normalize")
                .long("normalize")
                .about("Normalize everything"))
            .arg(Arg::new("binary")
                .short('b')
                .long("binary")
                .about("Make a presence-absence binary file"))
            .arg(Arg::new("non-covered")
                .long("non-covered")
                .about("Include non-covered entries (nodes or sequences) for dynamic normalizing calculations (e.g mean)"))
            .arg(Arg::new("stats")
                .short('s')
                .long("stats")
                .about("Normalize by mean or median (always in combination relative threshold)")
                .takes_value(true))
            .arg(Arg::new("type")
                .short('t')
                .long("type")
                .about("Type of output: node|sequence|pack [default: sequence]")
                .takes_value(true))
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .about("Name of the sample [default: name of the file]")
                .takes_value(true))


            //Output
            // As you might get mutiple file, takes value for everythin
            // Alternative only one run per process
            // ReaderBit and u16 with stats function
            // You iterate and lose information directly
            .help_heading("Output options")
            .arg(Arg::new("out")
                .short('o')
                .long("out")
                .about("Output name")
                .default_value("pack")
                .takes_value(true)))


        .get_matches();

    /*
    Input is  vg pack file
    If you want the coverage, make result
    if ou give a threshold, then the outcome is bitwise
    esle it is in u16 -> two byte
    For calculation:
    - if coverage + no thres -> wc-l -1 x2
    - if coverage + thresh - wc -l / 8
    - if node = thresh = cut -f 2 uniq wc -l x2
    - if node  tresh same as above but + 2
     */

    let mut level = LevelFilter::Info;
    // Checking verbose
    // Ugly, but needed - May end up in a small library later
    if matches.is_present("quiet") {
        level = LevelFilter::Warn;
    } else if matches.is_present("verbose") {
        if matches.value_of("verbose").unwrap() == "v1" {
            level = LevelFilter::Debug;
        } else if matches.value_of("verbose").unwrap() == "v" {
            level = LevelFilter::Trace
        }
    }

    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                     "{} [{}] - {}",
                     Local::now().format("%d/%m/%Y %H:%M:%S %p"),
                     record.level(),
                     record.args()
            )
        })
        .filter(None, level)
        .target(Target::Stderr)
        .init();


    // Collect the name
    info!("Running packing tool");


    // INDEX
    if let Some(ref matches) = matches.subcommand_matches("index") {
        debug!("Indexing");
        if matches.is_present("gfa") {
            debug!("GFA");
            let j = matches.value_of("gfa").unwrap();
            if Path::new(matches.value_of("gfa").unwrap()).exists() {
                let o = matches.value_of("output").unwrap();
                let buf = make_index(&j);
                writer_compress_zlib(&buf, o);
            } else {
                warn!("No file found");
                process::exit(0x0100);
            }
        } else if matches.is_present("pack") {
            debug!("PACK");
            let o = matches.value_of("output").unwrap();
            let p = parse_smart(matches.value_of("pack").unwrap());
            let buf = p.compress_only_node();
            writer_compress_zlib(&buf, o);
        } else {
            info!("No input")
        }
        process::exit(0x0100);
    }

    // Info
    if let Some(ref matches) = matches.subcommand_matches("info") {
        info!("Index info");
        if matches.is_present("index") | (matches.is_present("binary")) {
            if matches.is_present("index") {
                stats_index(matches.value_of("index").unwrap())
            } else {
                println!("{}", matches.value_of("binary").unwrap());
                if matches.is_present("all") {
                    info::info::stats(matches.value_of("binary").unwrap(), true, true);
                } else {
                    info::info::stats(matches.value_of("binary").unwrap(), true, false);
                }
            }
        }
    }

    // Rename
    if let Some(ref matches) = matches.subcommand_matches("rename") {
        info!("Renaming");
        test(matches.value_of("input").unwrap(), matches.value_of("name").unwrap().to_string(), matches.value_of("output").unwrap())
    }

    // CONVERT
    if let Some(ref matches) = matches.subcommand_matches("convert") {
        let mut s = "";
        let mut p: PackCompact = PackCompact::new();
        let mut no_file = false;
        // Determine Input format
        if matches.is_present("pack") | (matches.is_present("index") & matches.is_present("compressed pack")) {
            // READ "NORMAL" PACK FILE
            if matches.is_present("pack") {
                if Path::new(matches.value_of("pack").unwrap()).exists() {
                    p = parse_smart(matches.value_of("pack").unwrap());
                    let name: &str = matches.value_of("pack").unwrap();
                    let s2: Vec<&str> = name.split("/").collect();
                    s = s2.last().unwrap().clone();
                } else {
                    no_file = true;
                }
            }
            //READ COVERAGE AND META
            else {
                if Path::new(matches.value_of("index").unwrap()).exists() & Path::new(matches.value_of("compressed pack").unwrap()).exists() {
                    p = wrapper_meta(matches.value_of("index").unwrap(), matches.value_of("compressed pack").unwrap());
                    let name: &str = matches.value_of("compressed pack").unwrap();

                    let s2: Vec<&str> = name.split("/").collect();
                    s = s2.last().unwrap().clone();
                } else {
                    no_file = true;
                }
            }
        }

        // If name is set as argument, replace filename
        if matches.is_present("name") {
            s = matches.value_of("name").unwrap();
        }

        if no_file {
            info!("There is no input file");
            info!("[-h, --help] for help information");
            process::exit(0x0100);
        }
        if p.coverage.len() == 0 {
            info!("There is a problem with the input files. Run 'packing info' on your file.");
            info!("[-h, --help] for help information");
            process::exit(0x0100);
        } else {
            info!("File is {}", s)
        }


        // Checking the output base (sequence, nodes) or pack file
        let mut out_type = "sequence";
        if matches.is_present("type") {
            let ty = matches.value_of("type").unwrap();
            if ty == "node" {
                out_type = "node";
            } else if ty == "sequence" {
                out_type = "sequence";
            } else if ty == "pack" {
                out_type = "pack"
            } else {
                warn!("Not one of the available output types");
                warn!("Using default value: node");
            }
        }

        // Write the pack
        if out_type == "pack" {
            debug!("Writing pack file");
            write_pack(&p, matches.value_of("out").unwrap());
            process::exit(0x0100);
        }


        // If you want a binary file
        let mut bin = false;
        if matches.is_present("binary") {
            bin = true;
        }
        // Normalize the file
        let mut normalize = false;
        if matches.is_present("normalize") {
            normalize = true;
        }

        // Checking the for the real threshold.
        let mut absolute = false;
        let mut thresh: u16 = 0;
        let absolute_thresh;
        if matches.is_present("absolute threshold") {
            absolute = true;
            thresh = matches.value_of("absolute threshold").unwrap().parse().unwrap();
        }
        if matches.is_present("relative threshold") {
            thresh = matches.value_of("relative threshold").unwrap().parse().unwrap();
        }



        let mut stats: &str = "nothing";
        if matches.is_present("stats") {
            if (thresh != 0) & !absolute {
                let arg_stats = matches.value_of("stats").unwrap();
                if arg_stats == "mean" {
                    stats = arg_stats
                } else if arg_stats == "median" {
                    stats = arg_stats
                } else {
                    warn!("This metric is not available");
                    warn!("Normalized by percentile");
                }
            } else {
                warn!("You have not set additional threshold");
                warn!("Relative threshold is 100% (normalized by mean)");
                thresh = 100;
            }
        }


        // Want to include also the "zero" covered bases?
        let mut include_all = true;
        if matches.is_present("non-covered") {
            include_all = false;
        }

        // Absolute threshold is adjusted is made with thresh
        if !absolute {
            absolute_thresh = p.get_real_threshold(out_type == "node", include_all, thresh, stats);
        } else {
            p.node_coverage = p.get_node_cov_mean();
            absolute_thresh = thresh;
        }


        let mut output: Vec<u16>;
        if out_type == "node" {
            output = p.node_coverage;
        } else {
            output = p.coverage;
        }

        if normalize {
            output = normalizing(output, &absolute_thresh);
        }


        let buffer: Vec<u8>;
        if bin {
            buffer = vec2binary(output, &absolute_thresh);
        } else {
            buffer = vec_u16_u8(&output);
        }
        let mut bb = make_header(&(out_type == "node"), &absolute_thresh, &buffer, s);
        bb.extend(buffer);
        writer_compress_zlib(&bb, matches.value_of("out").unwrap());
    }
}


