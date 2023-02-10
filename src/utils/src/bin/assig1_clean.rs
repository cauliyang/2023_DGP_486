#[allow(unused)]
use clap::Parser;
use color_eyre::eyre::Result;
use env_logger::Builder;
use log::info;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "assig1_clean")]
#[command(version = "0.1.0")]
struct Args {
    /// The input file of svs
    #[arg(short = 'c', long)]
    count: PathBuf,

    /// The input file of gfs
    #[arg(short = 's', long)]
    sample: PathBuf,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn cli() -> Result<(PathBuf, PathBuf)> {
    let args = Args::parse();
    let count = args.count;
    let sample = args.sample;

    // if !count.exists() | !sample.exists() {
    //     panic!("count matrix file does not exist");
    // }

    info!("count: {:?} sample {:?}", count, sample);

    Ok((count, sample))
}

fn clean_count(count: PathBuf) -> Result<()> {
    // // read  txt file
    let new_path = count
        .canonicalize()?
        .parent()
        .unwrap()
        .join("clean_count.txt");

    let mut output = File::create(new_path)?;

    let file = File::open(count)?;
    let mut content = BufReader::new(file).lines().skip(1);

    let header = content.next().unwrap().unwrap();

    // wirte header

    let new_header = header
        .split('\t')
        .map(|x| {
            x.replace("1Bam/", "")
                .replace(".bam", "")
                .replace("l0", "l")
        })
        .collect::<Vec<String>>()[6..]
        .join("\t");

    output.write_all(("Geneid".to_string() + "\t" + new_header.as_str() + "\n").as_bytes())?;

    for line in content {
        let line = line.unwrap();
        let line = line.split('\t').collect::<Vec<&str>>();

        let mut line_content = line.into_iter();

        let _gene = line_content.next().unwrap();
        let _chr = line_content.next().unwrap();
        let _start = line_content.next().unwrap();
        let _end = line_content.next().unwrap();
        let _strand = line_content.next().unwrap();
        let _length = line_content.next().unwrap();

        let mut sum = 0;

        for i in line_content.clone() {
            sum += i.parse::<i32>().unwrap();
        }

        info!("gene: {} sum: {}", _gene, sum);

        if sum != 0 {
            let mut new_line = String::new();
            new_line.push_str(_gene);
            new_line.push('\t');
            new_line.push_str(line_content.collect::<Vec<&str>>().join("\t").as_str());
            new_line.push('\n');
            output.write_all(new_line.as_bytes())?;
        }
    }

    Ok(())
}

fn clean_sample(sample: PathBuf) -> Result<()> {
    // get absolute path
    let new_path = sample
        .canonicalize()?
        .parent()
        .unwrap()
        .join("clean_sample.txt");

    let mut output = File::create(new_path)?;

    let file = File::open(sample)?;
    let mut content = BufReader::new(file).lines().skip(1);

    Ok(())
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let (count, sample) = cli()?;

    clean_count(count).expect("failed to clean count matrix");
    // clean_sample(sample).expect("failed to clean sample matrix");

    Ok(())
}
