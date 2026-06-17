#!/usr/bin/env perl
use strict;
use warnings;

mkdir "target" unless -d "target";

open my $before, ">:raw", "target/example-before.bin" or die $!;
print {$before} "\0" x 512;
print {$before} "printb metadata section .text .rodata strings\n" x 13;
print {$before} chr($_ % 32) for 0..511;
print {$before} chr(128 + (($_ * 37) % 96)) for 0..1023;
print {$before} "\xff" x 512;
print {$before} "usage: printb input.bin -n 4096 -w 64 -o image.png\n" x 10;
print {$before} "\0" x 496;
close $before;

open my $in, "<:raw", "target/example-before.bin" or die $!;
open my $out, ">:raw", "target/example-after.bin" or die $!;
print {$out} do { local $/; <$in> };
close $in;

seek $out, 1536, 0;
print {$out} ("PATCHED-VERSION\0" x 64);
close $out;
