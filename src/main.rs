use std::fs::File;
use md5::{Md5, Digest};

use std::path::PathBuf;
use structopt::StructOpt;
use std::io::BufReader;

use noodles::bam::Reader;
use std::ops::Deref;

#[derive(Debug, StructOpt)]
#[structopt(name = "BAM hash generator", about = "Generate hash of BAM file using all records bytes in BAM file.")]
struct Opt {
    /// Input file
    #[structopt(short = "i", parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let hash = generate_hash(opt.input).unwrap();
    println!("{}", hash);
}

fn generate_hash(input_file_path: PathBuf) -> std::io::Result<String> {
    let file_reader = BufReader::new(File::open(input_file_path)?);
    let mut reader = Reader::new(file_reader);
    reader.read_header()?;
    reader.read_reference_sequences()?;

    let mut hasher = Md5::new();

    for result in reader.records() {
        hasher.update(result?.deref());
    }

    let result = hasher.finalize();

    // https://stackoverflow.com/a/67070521
    Ok(format!("{:x}", result))
}

