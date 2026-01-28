extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::{imageops, ImageBuffer};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn file_handle(limit: Option<u64>, path: &str) -> io::Result<Box<dyn Read>> {
    let f = File::open(path)?;
    Ok(match limit {
        None => Box::new(f),
        Some(n) => Box::new(f.take(n)),
    })
}

fn print_file(path: &str, limit: Option<u64>, width: u32, out: &str) -> io::Result<()> {
    let mut f = file_handle(limit, path)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;
    let n = buf.len();
    let h: u32 = width;
    let w = (n as f32 / h as f32).ceil() as u32;
    buf.resize_with((w * h) as usize, Default::default);
    let mut img = ImageBuffer::from_fn(w, h, |x, y| {
        let i = (x * h + y) as usize;
        if i < buf.len() {
            image::Luma([buf[i]])
        } else {
            image::Luma([0u8])
        }
    });
    img = imageops::resize(&img, 16 * w, 16 * h, imageops::FilterType::Nearest);
    img = imageops::rotate90(&img);
    img.save(out).unwrap();
    Ok(())
}

fn main() {
    let matches = App::new("printb")
        .arg(
            Arg::with_name("file")
                .help("binary filepath to print")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .help("number of bytes image width")
                .takes_value(true)
                .default_value("64"),
        )
        .arg(
            Arg::with_name("limit")
                .short("n")
                .help("limit number of bytes to read")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .short("o")
                .help("out file path to save image")
                .takes_value(true)
                .default_value("image.png"),
        )
        .get_matches();

    let file = matches.value_of("file").unwrap();
    let width = matches.value_of("width").unwrap();
    let limit = matches.value_of("limit").map(|n| n.parse::<u64>().unwrap());
    let out = matches.value_of("out").unwrap();

    print_file(file, limit, width.parse().unwrap(), out).unwrap();
}
