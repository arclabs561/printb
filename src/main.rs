use clap::{Arg, Command};
use image::{imageops, ImageBuffer, Rgb};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;

const PIXEL_SCALE: u32 = 8;

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

/// Lay `buf` out row-major: byte `i` is at column `i % width`, row `i / width`,
/// each drawn as a `PIXEL_SCALE` square. Missing tail bytes render as 0x00.
fn render(buf: &[u8], width: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let w = width;
    let h = (buf.len() as f32 / w as f32).ceil() as u32;
    let img = ImageBuffer::from_fn(w, h, |x, y| {
        byte_color(buf.get((y * w + x) as usize).copied().unwrap_or(0))
    });
    imageops::resize(
        &img,
        PIXEL_SCALE * w,
        PIXEL_SCALE * h,
        imageops::FilterType::Nearest,
    )
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
    let img = render(&buf, width);
    img.save(out).map_err(io::Error::other)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("printb")
        .arg(
            Arg::new("file")
                .help("binary filepath to print")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .help("number of bytes image width")
                .num_args(1)
                .default_value("64"),
        )
        .arg(
            Arg::new("limit")
                .short('n')
                .help("limit number of bytes to read")
                .num_args(1),
        )
        .arg(
            Arg::new("skip")
                .short('s')
                .long("skip")
                .help("number of bytes to skip before rendering")
                .num_args(1)
                .default_value("0"),
        )
        .arg(
            Arg::new("out")
                .short('o')
                .help("out file path to save image")
                .num_args(1)
                .default_value("image.png"),
        )
        .get_matches();

    let file = matches.get_one::<String>("file").unwrap();
    let width = matches.get_one::<String>("width").unwrap();
    let limit = matches
        .get_one::<String>("limit")
        .map(|n| n.parse::<u64>().unwrap());
    let skip = matches
        .get_one::<String>("skip")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let out = matches.get_one::<String>("out").unwrap();

    print_file(file, limit, skip, width.parse().unwrap(), out)
}

#[cfg(test)]
mod tests {
    use super::{byte_color, print_file, render, PIXEL_SCALE};
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
    fn renders_bytes_row_major_left_to_right() {
        // Width 3, two rows: 0xff is white, 0x00 black, 'A' = Rgb([32, 96, 161]).
        let img = render(&[0xff, 0x00, b'A', 0x00, 0xff, 0x00], 3);
        assert_eq!(img.dimensions(), (3 * PIXEL_SCALE, 2 * PIXEL_SCALE));
        let px = |col: u32, row: u32| *img.get_pixel(col * PIXEL_SCALE, row * PIXEL_SCALE);
        assert_eq!(px(0, 0), Rgb([255, 255, 255]));
        assert_eq!(px(1, 0), Rgb([0, 0, 0]));
        assert_eq!(px(2, 0), Rgb([32, 96, 161]));
        assert_eq!(px(0, 1), Rgb([0, 0, 0]));
        assert_eq!(px(1, 1), Rgb([255, 255, 255]));
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
