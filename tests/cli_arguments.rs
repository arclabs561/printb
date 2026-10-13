use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn sandbox() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "printb-cli-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&path).unwrap();
    path
}

#[test]
fn numeric_errors_precede_input_io() {
    let dir = sandbox();
    for (flag, value) in [
        ("-w", Some("0")),
        ("-w", Some("invalid")),
        ("-n", Some("invalid")),
        ("-s", Some("invalid")),
        ("-w", Some("4294967296")),
        ("-n", Some("18446744073709551616")),
        ("-s", Some("18446744073709551616")),
        ("-w", None),
        ("-n", None),
        ("-s", None),
    ] {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_printb"));
        cmd.current_dir(&dir).arg("missing.bin").arg(flag);
        if let Some(value) = value {
            cmd.arg(value);
        }
        let output = cmd.output().unwrap();
        let error = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(2), "{flag} {value:?}: {error}");
        assert!(error.contains(flag), "{}", error);
        assert!(!error.contains("panicked"), "{}", error);
        assert!(!error.contains("No such file"), "{}", error);
        assert!(output.stdout.is_empty());
        assert!(!dir.join("image.png").exists());
    }
    std::fs::remove_dir(&dir).unwrap();
}

#[test]
fn positive_width_limit_and_skip_preserve_pixels() {
    let dir = sandbox();
    std::fs::write(dir.join("input.bin"), [0, b'A', 0xff, 0x80]).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_printb"))
        .current_dir(&dir)
        .args([
            "input.bin",
            "-w",
            "1",
            "-n",
            "2",
            "--skip",
            "1",
            "-o",
            "out.png",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let image = image::open(dir.join("out.png")).unwrap().to_rgb8();
    assert_eq!(image.dimensions(), (8, 16));
    for y in 0..16 {
        for x in 0..8 {
            assert_eq!(
                image.get_pixel(x, y).0,
                if y < 8 {
                    [32, 96, 161]
                } else {
                    [255, 255, 255]
                }
            );
        }
    }
    let zero = Command::new(env!("CARGO_BIN_EXE_printb"))
        .current_dir(&dir)
        .args(["input.bin", "-n", "0", "-s", "0", "-o", "zero.png"])
        .output()
        .unwrap();
    assert_eq!(zero.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&zero.stderr).contains("no bytes to render"));
    assert!(!dir.join("zero.png").exists());
    std::fs::remove_file(dir.join("input.bin")).unwrap();
    std::fs::remove_file(dir.join("out.png")).unwrap();
    std::fs::remove_dir(dir).unwrap();
}
