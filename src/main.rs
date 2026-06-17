extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::{imageops, ImageBuffer, Rgb};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

fn file_handle(limit: Option<u64>, skip: u64, path: &str) -> io::Result<Box<dyn Read>> {
    let mut f = File::open(path)?;
    f.seek(SeekFrom::Start(skip))?;
    Ok(match limit {
        None => Box::new(f),
        Some(n) => Box::new(f.take(n)),
    })
}

fn byte_color(byte: u8) -> Rgb<u8> {
    match byte {
        0x00 => Rgb([0, 0, 0]),
        0x01..=0x1f => Rgb([0, byte.saturating_mul(8), 48]),
        0x20..=0x7e => Rgb([32, 96, byte.saturating_add(96)]),
        0xff => Rgb([255, 255, 255]),
        _ => Rgb([byte, 32, 32]),
    }
}

fn print_file(path: &str, limit: Option<u64>, skip: u64, width: u32, out: &str) -> io::Result<()> {
    let mut f = file_handle(limit, skip, path)?;
    let mut buf = vec![];
    f.read_to_end(&mut buf)?;
    let n = buf.len();
    if n == 0 {
        return Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "no bytes to render",
        ));
    }
    let h: u32 = width;
    let w = (n as f32 / h as f32).ceil() as u32;
    buf.resize_with((w * h) as usize, Default::default);
    let mut img = ImageBuffer::from_fn(w, h, |x, y| {
        let i = (x * h + y) as usize;
        if i < buf.len() {
            byte_color(buf[i])
        } else {
            byte_color(0)
        }
    });
    img = imageops::resize(&img, 16 * w, 16 * h, imageops::FilterType::Nearest);
    img = imageops::rotate90(&img);
    img.save(out).map_err(io::Error::other)?;
    Ok(())
}

fn main() -> io::Result<()> {
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
            Arg::with_name("skip")
                .short("s")
                .long("skip")
                .help("number of bytes to skip before rendering")
                .takes_value(true)
                .default_value("0"),
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
    let skip = matches.value_of("skip").unwrap().parse::<u64>().unwrap();
    let out = matches.value_of("out").unwrap();

    print_file(file, limit, skip, width.parse().unwrap(), out)
}

#[cfg(test)]
mod tests {
    use super::{byte_color, print_file};
    use image::Rgb;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn maps_common_byte_classes_to_distinct_colors() {
        assert_eq!(byte_color(0x00), Rgb([0, 0, 0]));
        assert_eq!(byte_color(0x0a), Rgb([0, 80, 48]));
        assert_eq!(byte_color(b'A'), Rgb([32, 96, 161]));
        assert_eq!(byte_color(0x80), Rgb([128, 32, 32]));
        assert_eq!(byte_color(0xff), Rgb([255, 255, 255]));
    }

    #[test]
    fn reports_empty_slice_instead_of_panicking() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let input = std::env::temp_dir().join(format!("printb-empty-{stamp}.bin"));
        let output = std::env::temp_dir().join(format!("printb-empty-{stamp}.png"));
        fs::write(&input, [1, 2, 3]).unwrap();

        let err = print_file(
            input.to_str().unwrap(),
            Some(16),
            16,
            8,
            output.to_str().unwrap(),
        )
        .unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::UnexpectedEof);

        fs::remove_file(input).unwrap();
    }
}
