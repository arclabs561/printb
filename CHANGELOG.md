# Changelog

Earlier history has not been reconstructed in this changelog.

## Source history (unversioned)

These entries identify source changes without assigning a package release.

- Validate numeric CLI arguments and require a positive byte-grid width
  before reading input.
  ([9815b0c](https://github.com/arclabs561/printb/commit/9815b0cf33e1004be5fe556a88aad6ef8ed85038))
- Use integer ceiling division for byte-row counts, avoiding loss of the
  final row beyond floating-point integer precision. The existing image-height
  saturation is retained.
  ([59a1f29](https://github.com/arclabs561/printb/commit/59a1f29f2e91137822346f57e0af45e8ed46bf3e))
