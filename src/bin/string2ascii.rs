use image2ascii::string2ascii;
use image2ascii::Char2DArray;
//use std::path::PathBuf;
use std::fs::File;
use std::io::{BufWriter, Write};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CmdOpt {
    #[structopt(short, long)]
    input: String,
    #[structopt(short, long)]
    ch: char,
    #[structopt(short, long)]
    output: Option<String>,
    #[structopt(short, long)]
    height: u32,
    #[structopt(short,long)]
    font: Option<String>,
}

fn main() {
    let opt = CmdOpt::from_args();
    let c2d: Char2DArray;
    if let Some(font_file) = opt.font {
        c2d = string2ascii(&opt.input, opt.height as f32, opt.ch, Option::None, Some(&font_file)).unwrap();
    }else{
        c2d = string2ascii(&opt.input, opt.height as f32, opt.ch, Option::None, None).unwrap();
    }
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
