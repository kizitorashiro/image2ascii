use image2ascii::image2ascii;
//use std::path::PathBuf;
use std::fs::File;
use std::io::{BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CmdOpt {
    #[structopt(short, long)]
    input: String,
    #[structopt(short, long)]
    output: Option<String>,
    #[structopt(short, long)]
    width: u32,
    #[structopt(short, long, default_value = "30.0")]
    contrast: f32,
}

fn main() {
    let opt = CmdOpt::from_args();
    let c2d = image2ascii(&opt.input, opt.width, Option::Some(opt.contrast), Option::None).unwrap();
    let lines = c2d.to_lines();
    if let Some(output_file) = opt.output {
        println!("output to file {}", output_file);
        let mut out = BufWriter::new(File::create(output_file).unwrap());
        for line in lines {
            out.write(line.as_bytes()).unwrap();
            out.write("\n".as_bytes()).unwrap();
        }
        out.flush().unwrap();
    } else {
        for line in lines {
            println!("{}", line);
        }
    }
}
