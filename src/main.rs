mod constants;

use std::path::PathBuf;
use structopt::StructOpt;
use std::{fs, process};
use constants::Message;
use assert_json_diff::assert_json_eq;
use serde_json::{json, from_str, Value};

#[derive(Debug, StructOpt)]
#[structopt(about = "To display the help, just pass `-h`", name = "json-diff", about = "json-diff arguments.")]
struct Opt {
    /// Input file 1
    file1: PathBuf,

    /// Input file 2
    file2: PathBuf,

   /* /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,

    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    file_name: Option<String>,

    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool*/
}

fn error_exit(messages: Vec<constants::Message>) -> ! {
    for msg in messages {
        eprintln!("{}", msg);
    }
    process::exit(1);
}

fn compare_jsons(data1: &str, data2: &str) {
    let json1: Result<Value, serde_json::Error> = from_str(data1);
    let json2: Result<Value, serde_json::Error> = from_str(data2);

    match(json1, json2) {
        (Ok(value1), Ok(value2)) => {
            assert_json_eq!(json!(value1), json!(value2));
        },
        (_, Ok(_)) => error_exit(vec!(Message::JSON1)),
        (Ok(_), _) => error_exit(vec!(Message::JSON2)),
        (_, _) => error_exit(vec!(Message::JSON1, Message::JSON2))
    }
}

fn main() {
    let opt = Opt::from_args();
    let json_file_1_result = fs::read_to_string(opt.file1);
    let json_file_2_result = fs::read_to_string(opt.file2);

    match (json_file_1_result, json_file_2_result) {
        (Ok(data1), Ok(data2)) => {
            compare_jsons(&data1, &data2);
        }
        (_, Ok(_)) => error_exit(vec!(Message::SOURCE1)),
        (Ok(_), _) => error_exit(vec!(Message::SOURCE2)),
        (_, _) => error_exit(vec!(Message::SOURCE1, Message::SOURCE2))
    }
}
