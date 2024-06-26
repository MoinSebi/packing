use clap::ArgMatches;
use log::info;
use packing_lib::core::core::{DataType, PackCompact};
use packing_lib::core::reader::read_input;
use packing_lib::normalize::helper::{
    calculate_std_deviation, mean, median, remove_zero, remove_zero_f32,
};

use std::fs::File;
use std::io::{self, Write};

pub fn stats_main(matches: &ArgMatches) {
    info!("Stats main");
    let (mut pc, index_present) = read_input(matches);
    let output = matches.value_of("output").unwrap_or("-");
    if output == "-" {
        stats_wrapper(&mut pc, index_present, &mut None);
    } else {
        let mut file = File::create(output).expect("Unable to create file");
        stats_wrapper(&mut pc, index_present, &mut Some(&mut file));
    }
}

fn write_to_file_or_stdout(file: &mut Option<&mut File>, content: &str) -> io::Result<()> {
    match file {
        Some(f) => {
            f.write_all(content.as_bytes())?;
        }
        None => {
            println!("{}", content);
        }
    }
    Ok(())
}

pub fn stats_wrapper(pc: &mut PackCompact, _index_present: bool, file2: &mut Option<&mut File>) {
    write_to_file_or_stdout(file2, &format!("Name: {}", pc.name)).expect("Can not write file");
    write_to_file_or_stdout(
        file2,
        &format!("Input {}", if pc.is_sequence { "Sequence" } else { "Node" }),
    )
    .expect("dasjdka");
    write_to_file_or_stdout(file2, "Stats\tZeros\tN/S\tValue").expect("Can not write file");
    if pc.data_type == DataType::TypeBit {
        info!("Is bit!");
        write_to_file_or_stdout(
            file2,
            &format!(
                "Number presence entries {}",
                pc.bin_coverage.iter().filter(|x| **x).count()
            ),
        )
        .expect("Can not write file");
        write_to_file_or_stdout(
            file2,
            &format!(
                "Number absence entries {}",
                pc.bin_coverage.iter().filter(|x| !**x).count()
            ),
        )
        .expect("Can not write file");
    } else if pc.data_type == DataType::TypeU16 {
        info!("Compressed values");
        if !pc.is_sequence {
            let mut workon = pc.coverage.clone();
            chaotic_input(file2, &mut workon, true, true);

            remove_zero(&mut workon);
            chaotic_input(file2, &mut workon, false, true);
        } else {
            let mut workon = pc.coverage.clone();
            chaotic_input(file2, &mut workon, true, false);

            remove_zero(&mut workon);
            chaotic_input(file2, &mut workon, false, false);

            if _index_present {
                pc.calc_node_cov();
                let mut workon2 = pc.normalized_coverage.clone();
                chaotic_input(file2, &mut workon2, true, true);
                remove_zero_f32(&mut workon2);
                chaotic_input(file2, &mut workon2, false, true);
            }
        }
    } else {
        println!("Normalized values");
        if !pc.is_sequence {
            let mut workon = pc.normalized_coverage.clone();
            chaotic_input(file2, &mut workon, true, true);

            remove_zero_f32(&mut workon);
            chaotic_input(file2, &mut workon, false, true);
        } else {
            let mut workon = pc.normalized_coverage.clone();
            chaotic_input(file2, &mut workon, true, false);

            remove_zero_f32(&mut workon);
            chaotic_input(file2, &mut workon, false, false);
            if _index_present {
                pc.calc_node_cov();
                let mut workon2 = pc.normalized_coverage.clone();
                chaotic_input(file2, &mut workon2, true, true);
                remove_zero_f32(&mut workon2);
                chaotic_input(file2, &mut workon2, false, true);
            }
        }
    }
}

pub fn chaotic_input<T>(
    file2: &mut Option<&mut File>,
    workon: &mut [T],
    with_zeros: bool,
    is_node: bool,
) where
    T: Into<f64> + Copy,
    T: std::ops::Add<Output = T> + std::convert::From<u8> + Copy,
    f64: std::convert::From<T>,
    T: PartialOrd + Copy,
    f64: From<T>,
{
    write_to_file_or_stdout(
        file2,
        &format!(
            "Average\t{}\t{}\t{}",
            with_zeros,
            if is_node { "Node" } else { "Seq" },
            mean(workon)
        ),
    )
    .expect("Can not write file");
    write_to_file_or_stdout(
        file2,
        &format!(
            "Std\t{}\t{}\t{}",
            with_zeros,
            if is_node { "Node" } else { "Seq" },
            calculate_std_deviation(workon)
        ),
    )
    .expect("Can not write");

    write_to_file_or_stdout(
        file2,
        &format!(
            "Median\t{}\t{}\t{}",
            with_zeros,
            if is_node { "Node" } else { "Seq" },
            median(workon)
        ),
    )
    .expect("Can not write file");
}
