use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rat")]
struct Rat {
    /// Equivalent to -vET
    #[structopt(short = "A", long = "show-all")]
    show_all: bool,

    /// Number nonempty output lines
    #[structopt(short = "b", long = "number-nonblank")]
    number_nonblank: bool,

    /// Display $ at the end of each line
    #[structopt(short = "E", long = "show-ends")]
    show_ends: bool,

    /// Equivalent to -vE
    #[structopt(short = "e")]
    e: bool,

    /// Number all output lines
    #[structopt(short = "n", long)]
    number: bool,

    /// Suppress repeated empty output lines
    #[structopt(short = "s", long = "squeeze-blank")]
    squeeze_blank: bool,

    /// Display TAB characters as ^I
    #[structopt(short = "T", long = "show-tabs")]
    show_tabs: bool,

    /// Equivalent to -vT
    #[structopt(short = "t")]
    t: bool,

    /// Use ^ and M- notation, except for LFD and TAB
    /// (NOT YET SUPPORTED)
    #[structopt(short = "v", long = "show-nonprinting")]
    show_nonprinting: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn handle_file(fp: &PathBuf, opts: &Rat) -> Result<()> {
    let file = File::open(fp)?;
    let mut lc = 1;
    let mut last_empty = false;
    for line in BufReader::new(file).lines() {
        let mut line = line?;

        if opts.show_tabs {
            line = line.replace("\t", "^I");
        }

        if opts.squeeze_blank && last_empty && line.is_empty() {
            continue;
        }

        if opts.number || (opts.number_nonblank && !line.is_empty()) {
            print!("{:6}\t", lc);
            lc += 1;
        }

        print!("{}", line);

        if opts.show_ends {
            print!("$");
        }

        println!();

        last_empty = line.is_empty();
    }

    Ok(())
}

fn main() {
    let mut opts = Rat::from_args();

    if opts.show_all {
        opts.show_nonprinting = true;
        opts.show_ends = true;
        opts.show_tabs = true;
    }

    if opts.e {
        opts.show_nonprinting = true;
        opts.show_ends = true;
    }

    if opts.t {
        opts.show_nonprinting = true;
        opts.show_tabs = true;
    }

    for f in &opts.files {
        if let Err(e) = handle_file(f, &opts) {
            println!("{}", e);
            break;
        }
    }
}
