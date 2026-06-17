# printb

Render binary files as color images.

`printb` maps each byte to a pixel in a fixed-width grid. The default palette
marks null bytes as black, control bytes as green, printable ASCII as blue,
high-value bytes as red, and `0xff` as white.

## Install

```bash
cargo install --git https://github.com/arclabs561/printb
```

Or build from a checkout:

```bash
cargo build --release
```

## Usage

```bash
printb target/debug/printb -s 3500000 -n 4096 -w 64 -o examples/image.png
```

![Color rendering of a byte range from the printb binary](examples/image.png)

The `-s` flag skips bytes before rendering, `-n` limits how many bytes are read,
`-w` sets the byte-grid width, and `-o` sets the output path.

## Comparing Binary Versions

Binary images are useful when the exact bytes matter but a hex dump is too narrow
to scan. A small byte-level change can become a visible block, band, or repeated
pattern.

This example renders the same byte range from `printb`, then from a copy with an
ASCII marker patched into that range:

```bash
cp target/debug/printb target/printb-patched
perl -e 'open my $f, "+<", "target/printb-patched" or die $!; binmode $f; seek $f, 3500000, 0; print $f ("PRINTB-DEMO-ASCII-PATCH\0" x 170); close $f;'

printb target/debug/printb -s 3500000 -n 4096 -w 64 -o examples/image.png
printb target/printb-patched -s 3500000 -n 4096 -w 64 -o examples/patched.png
```

Unmodified slice:

![Unmodified byte range from the printb binary](examples/image.png)

Patched slice:

![The same byte range after patching in a repeated ASCII marker](examples/patched.png)

The patched image exposes the repeated marker as a regular diagonal pattern. The
same trick can help inspect packed regions, appended payloads, stripped symbols,
or unexpected changes between builds.

## Related Work

Binary visualization has a long reverse-engineering lineage. Greg Conti and
Erik Dean's [visual reverse-engineering work](https://vizsec.org/files/2008/Conti.pdf)
framed byte plots, entropy, byte frequency, strings, and n-grams as
file-independent ways to inspect unknown files. Christopher Domas's
["The Future of RE: Dynamic Binary Visualization"](https://www.youtube.com/watch?v=4bM3Gut1hIk)
is the well-known talk version of the idea. Aldo Cortesi's
[`binvis`](https://corte.si/posts/visualisation/binvis/) and
[`binvis.io`](https://binvis.io/) popularized Hilbert-curve file views; newer
examples include Battelle's
[CantorDust](https://inside.battelle.org/blog-details/battelle-publishes-open-source-binary-visualization-tool),
Stairwell's
[Hilbert-curve malware-analysis writeup](https://stairwell.com/blog/hilbert-curves-visualizing-binary-files-with-color-and-patterns/),
and 8dcc's [`bin-graph`](https://github.com/8dcc/bin-graph).

## License

Dual-licensed under MIT or the UNLICENSE.
