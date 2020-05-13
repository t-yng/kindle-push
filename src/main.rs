//! A somewhat comprehensive example of a typical `StructOpt` usage.use
// clapのREADME.mdにあるdriveのexampleは正しくないので clap_drive/examples を参照してください。
// @see: https://github.com/clap-rs/clap/blob/master/clap_derive/examples

use clap::Clap;
use std::path::PathBuf;

// driveの使い方の例
// @see: https://github.com/clap-rs/clap/blob/master/clap_derive/examples/basic.rs
#[derive(Clap, Debug)]
#[clap(name = "basic")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,

    /// Set speed
    #[clap(short, long, default_value = "42")]
    speed: f64,

    /// Output file
    #[clap(short, long, parse(from_os_str))]
    output: PathBuf,

    // the long option will be translated by default to kebab case,
    // i.e. `--nb-cars`.
    /// Number of cars
    #[clap(short = "c", long)]
    nb_cars: Option<i32>,

    /// admin_level to consider
    #[clap(short, long)]
    level: Vec<String>,

    /// Files to process
    // #[clap(name = "FILE", parse(from_os_str))]
    // files: Vec<PathBuf>,

    #[clap(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::parse();
    println!("{:#?}", opt);
}