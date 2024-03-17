mod parser;

use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please provide a file as the first argument");
        return;
    }

    let filename = fs::canonicalize(&args[1]).unwrap();
    println!("Loading {}", filename.to_string_lossy().trim_start_matches("\\\\?\\"));

    let binary = BufReader::new(File::open(filename.clone()).unwrap());
    let dom = rbx_binary::from_reader(binary);

    match dom {
        Ok(dom) => {
            let refs = dom.root().children();

            println!("Found {} top-level DOM refs", refs.len());

            let output = BufWriter::new(File::create(format!("{}x", filename.clone().display())).unwrap());

            match rbx_xml::to_writer_default(output, &dom, refs) {
                Ok(_) => {
                    println!("DOM XML conversion successful");
                }
                Err(err) => {
                    println!("Error during XML conversion: {:?}", err);
                }
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}