use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "To display the help, just pass `-h`", name = "json-diff", about = "json-diff arguments.")]
struct Opt {
    /// Input file 1
    file1: PathBuf,

    /// Input file 2
    file2: PathBuf,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,

    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool
}


fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
