mod index;
mod info;
mod convert;
mod rename;
mod core;


use clap::{App, AppSettings, Arg};
use chrono::Local;
use env_logger::{Builder, Target};
use log::{debug, info, LevelFilter};
use std::io::Write;
use crate::convert::convert_main::convert_main;
use crate::index::index_main::index_main;
use crate::info::info_main::info_main;
use crate::rename::rename::rename_main;

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
        )


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
            .about("Change the name in the header of pc or pa")
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
                .about("Type of output: node|sequence|pack [default: node]")
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
                .takes_value(true))
            .arg(Arg::new("non-compressed")
                .long("non-compressed")
                .about("Non-compressed output")))

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
        index_main(matches);
    }

    // Info
    else if let Some(ref matches) = matches.subcommand_matches("info") {
        info_main(matches);
    }

    // Rename
    if let Some(ref matches) = matches.subcommand_matches("rename") {
        rename_main(matches);
    }

    // CONVERT
    if let Some(ref matches) = matches.subcommand_matches("convert") {
        convert_main(matches);
    }
}


