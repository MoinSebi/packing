mod bit;
mod comp;
mod compress;
mod index;
mod info;
mod rename;
mod stats;
mod view;

use crate::index::index_main::index_main;
use crate::info::info_main::info_main;
use crate::rename::rename_main::rename_main1;
use crate::stats::stats_main::stats_main;
use crate::view::view_main::view_main;
use chrono::Local;
use clap::{App, AppSettings, Arg};
use env_logger::{Builder, Target};
use log::{info, LevelFilter};

use crate::bit::bit_main::bit_main;
use crate::comp::comp_main::comp_main;
use crate::compress::compress_main::compress_main;
use packing_lib::normalize::normalize_main::normalize_main;
use std::io::Write;

fn main() {
    let matches = App::new("packing")
        .version("0.1.0")
        .author("Sebastian V")
        .about("Working with coverage (pack) files")
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
            .about("Information about index or compressed files. 'Normal' packs are excluded")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)

            .help_heading("Input options")
            .arg(Arg::new("pack compressed")
                .short('c')
                .long("compressed")
                .about("Information about the binary")
                .takes_value(true))
            .arg(Arg::new("index")
                .short('i')
                .long("index")
                .about("Information about the index")
                .takes_value(true))
        )


        .subcommand(App::new("index")
            .about("Index a GFA or plain-text coverage file (pack format)")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)

            .help_heading("Input options")
            .arg(Arg::new("gfa")
                .display_order(1)
                .short('g')
                .long("gfa")
                .about("Graphical Fragment Assembly (GFA) file")
                .takes_value(true))
            .arg(Arg::new("pack")
                .short('p')
                .long("pack")
                .about("Pack coverage file (plain-text)")
                .takes_value(true))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file")
                .takes_value(true)
                .required(true)))


        .subcommand(App::new("rename")
            .about("Change the name of a compressed pack file. 'Normal' packs are excluded")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)

            .help_heading("Input options")
            .arg(Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .about("Pack file input")
                .takes_value(true))

            .help_heading("Modification options")
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .required(true)
                .about("New name")
                .takes_value(true))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                 .long("output")
                 .about("Output file")
                 .required(true)
                 .takes_value(true)))


        .subcommand(App::new("normalize")
            .about("Normalize (compress) a pack file with a custom threshold")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)

            .help_heading("Input options")
            .arg(Arg::new("pack")
                .short('p')
                .long("pack")
                .about("Input pack file (plain-text)")
                .takes_value(true))
            .arg(Arg::new("index")
                .short('i')
                .long("index")
                .about("Index file from 'packing index'")
                .takes_value(true))
            .arg(Arg::new("pack compressed")
                .long("compressed")
                .short('c')
                .about("Compressed or normalized pack file")
                .takes_value(true))


            .help_heading("Normalization parameters")
            .arg(Arg::new("absolute-threshold")
                .short('a')
                .long("absolute-threshold")
                .about("Set a absolute threshold")
                .takes_value(true)
                .display_order(0))
            // Modification
            .arg(Arg::new("method")
                .short('m')
                .long("method")
                .about("Normalization method (mean|median|percentile|nothing) [default: nothing]")
                .takes_value(true)
                .display_order(1)
            )

            .arg(Arg::new("fraction")
                .short('f')
                .long("fraction")
                .about("Fraction")
                .takes_value(true)
                .display_order(2)
            )
            .arg(Arg::new("keep-zeros")
                .long("keep-zeros")
                .about("Include non-covered entries (nodes or sequences) for dynamic threshold calculations (e.g mean)")
                .display_order(4)
            )
            .arg(Arg::new("node")
                .short('n')
                .long("node")
                .about("Merge coverage on node level [default: off -> sequence-level]. The default will adjust if input is already node-level.")
                .display_order(5)
            )



            .help_heading("Modification options")
            .arg(Arg::new("name")
                .long("name")
                .about("Change the name of the sample using this value [default: name of the file]")
                .takes_value(true))

            .help_heading("Output options")
            .arg(Arg::new("out")
                .short('o')
                .long("out")
                .about("Output name")
                .takes_value(true)
                .required(true))
        )


        .subcommand(App::new("view")
            .about("View the compressed pack data in plain text")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)

            .help_heading("Input options")
            .arg(Arg::new("pack compressed")
                .short('c')
                .long("compressed")
                .about("compressed pack file")
                .takes_value(true)
                .required(true))
            .arg(Arg::new("index")
                .short('i')
                .long("index")
                .about("Index file")
                .takes_value(true))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file name")
                .takes_value(true))
        )

        .subcommand(App::new("stats")
            .about("Statistics on pack files (simple and dumb)")
            .version("0.1.0")
            .setting(AppSettings::ArgRequiredElseHelp)

            .help_heading("Input options")
            .arg(Arg::new("pack")
                .short('p')
                .long("pack")
                .about("Plain text pack file")
                .takes_value(true))
            .arg(Arg::new("index")
                .short('i')
                .long("index")
                .about("Index file from 'packing index'")
                .takes_value(true))
            .arg(Arg::new("pack compressed")
                .long("compressed")
                .short('c')
                .about("Compressed pack file. Original can only be accessed if the file is not normalized.")
                .takes_value(true))

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file name")
                .takes_value(true))

        )
        .subcommand(App::new("compress")
            .about("Compress a plain-text coverage file to reduce memory")
            .setting(AppSettings::ArgRequiredElseHelp)

            .help_heading("Input options")
            .arg(Arg::new("pack")
                .short('p')
                .long("pack")
                .about("Plain text pack file")
                .takes_value(true))

            .help_heading("Modification options")
            .arg(Arg::new("name")
                .short('n')
                .long("name")
                .about("Name of the sample [default: name of the file]")
                .takes_value(true)
            )

            .help_heading("Output options")
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .about("Output file name")
                .takes_value(true)
                .required(true))
        )

        .subcommand(App::new("compare")
            .about("Compare two compressed pack files and check if they made based on the same stats")
            .setting(AppSettings::ArgRequiredElseHelp)

            .help_heading("Input options")
            .arg(Arg::new("pack compressed1")
                .long("compressed1")
                .about("Compressed pack file (1)")
                .takes_value(true))

            .arg(Arg::new("pack compressed2")
                .long("compressed2")
                .about("Compressed pack file (2)")
                .takes_value(true))


        )

        .subcommand(App::new("bit")
            .about("Convert a pack file to a presence-absence file (binary) using custom threshold")
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
            .arg(Arg::new("pack compressed")
                .long("compressed")
                .short('c')
                .about("Compressed pack file.")
                .takes_value(true))


            .help_heading("Threshold parameters")
            .arg(Arg::new("absolute-threshold")
                .short('a')
                .long("absolute-threshold")
                .about("Presence-absence according to absolute threshold")
                .takes_value(true)
                .display_order(0))
            // Modification
            .arg(Arg::new("method")
                .short('m')
                .long("method")
                .about("Normalization method (mean|median|percentile|nothing) [default: nothing]")
                .takes_value(true)
                .display_order(1)
            )

            .arg(Arg::new("fraction")
                .short('f')
                .long("fraction")
                .about("Fraction of (can be combined with 'normalize' flag")
                .takes_value(true)
                .display_order(2)
            )
            .arg(Arg::new("keep-zeros")
                .long("keep-zeros")
                .about("Include non-covered entries (nodes or sequences) for dynamic threshold calculations (e.g mean)")
                .display_order(4)

            )
            .arg(Arg::new("node")
                .short('n')
                .long("node")
                .about("Merge coverage on node level [default: off -> sequence-level]")
                .display_order(5)

            )

            .help_heading("Modification options")
            .arg(Arg::new("name")
                .long("name")
                .about("Name of the sample [default: name of the file]")
                .takes_value(true))


            //Output
            .help_heading("Output options")
            .arg(Arg::new("out")
                .short('o')
                .long("out")
                .about("Output name")
                .takes_value(true)
                .required(true))
        )



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
            writeln!(
                buf,
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
    if let Some(matches) = matches.subcommand_matches("index") {
        index_main(matches);
    }
    // Info
    else if let Some(matches) = matches.subcommand_matches("info") {
        info_main(matches);
    }

    // Rename
    if let Some(matches) = matches.subcommand_matches("rename") {
        rename_main1(matches);
    }

    // CONVERT
    if let Some(matches) = matches.subcommand_matches("view") {
        view_main(matches);
    }
    if let Some(matches) = matches.subcommand_matches("stats") {
        stats_main(matches);
    }

    if let Some(matches) = matches.subcommand_matches("bit") {
        bit_main(matches);
    }
    if let Some(matches) = matches.subcommand_matches("normalize") {
        normalize_main(matches);
    }
    if let Some(matches) = matches.subcommand_matches("compress") {
        compress_main(matches);
    }
    if let Some(matches) = matches.subcommand_matches("compare") {
        comp_main(matches);
    }
}
