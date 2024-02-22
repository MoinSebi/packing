use clap::ArgMatches;
use crate::info::info::{stats, stats_index};

pub fn info_main(matches: &ArgMatches) {
    if matches.is_present("index") | (matches.is_present("binary")) {
        if matches.is_present("index") {
            stats_index(matches.value_of("index").unwrap())
        } else {
            if matches.is_present("all") {
                stats(matches.value_of("binary").unwrap());
            } else {
                stats(matches.value_of("binary").unwrap());
            }
        }
    }
}