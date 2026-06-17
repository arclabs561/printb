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
perl - <<'PL' > target/example-before.bin
binmode STDOUT;
print "\0" x 512;
print "printb metadata section .text .rodata strings\n" x 13;
print chr($_ % 32) for 0..511;
print chr(128 + (($_ * 37) % 96)) for 0..1023;
print "\xff" x 512;
print "usage: printb input.bin -n 4096 -w 64 -o image.png\n" x 10;
print "\0" x 496;
PL

printb target/example-before.bin -n 4096 -w 64 -o examples/image.png
```

![Color rendering of a synthetic binary with padding, text, control bytes, high-byte data, and 0xff regions](examples/image.png)

The `-s` flag skips bytes before rendering, `-n` limits how many bytes are read,
`-w` sets the byte-grid width, and `-o` sets the output path.

## Comparing Binary Versions

Binary images are useful when the exact bytes matter but a hex dump is too narrow
to scan. A small byte-level change can become a visible block, band, or repeated
pattern.

This example renders the same synthetic binary, then a second version with an
ASCII marker patched into the high-byte region:

```bash
cp target/example-before.bin target/example-after.bin
perl - <<'PL'
open my $f, "+<", "target/example-after.bin" or die $!;
binmode $f;
seek $f, 1536, 0;
print $f ("PATCHED-VERSION\0" x 64);
close $f;
PL

printb target/example-before.bin -n 4096 -w 64 -o examples/image.png
printb target/example-after.bin -n 4096 -w 64 -o examples/patched.png
```

Before:

![Synthetic binary before patching](examples/image.png)

After:

![The same byte range after patching in a repeated ASCII marker](examples/patched.png)

The patched image exposes the inserted marker as a blue block inside a region
that was previously high-byte data. The same trick can help inspect packed
regions, appended payloads, stripped symbols, or unexpected changes between
builds.

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

`printb` is deliberately smaller than those tools: it writes a fixed-width PNG
from bytes on disk and leaves interactive navigation, digraph plots, and
Hilbert-curve layouts to heavier analyzers.

## License

Dual-licensed under MIT or the UNLICENSE.
