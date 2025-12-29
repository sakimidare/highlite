# highlite

[简体中文](/docs/README_zh-CN.md) / [English](/README.md)

highlite is a fast, rule-based CLI highlighter for stdin and files, written in Rust.

It reads text line by line and highlights matches using ANSI colors, making it suitable
for large files, streaming input, and Unix-style pipelines.

---

## Features

- High performance: all rules are compiled into a single regex at startup
- Rule-based highlighting using keywords or regular expressions
- Supports preset ANSI colors and 24-bit RGB colors
- YAML configuration with optional recursive includes
- Designed for streaming input (stdin, pipes, large files)
- Minimal memory allocation during processing
- Per-rule and global case-insensitive matching

---

## Installation

### From crates.io

```bash
cargo install highlite
```

### From source

```bash
git clone https://github.com/sakimidare/highlite.git
cd highlite
cargo build --release
```

## Usage

Highlight stdin:

```bash
cat examples/logs/example_cpp.cpp | highlite --config examples/rules/cpp_rules.yaml
```

Highlight a file:

```bash
highlite --config examples/rules/log_rules.yaml --file examples/logs/example_log.log
```

Force case-insensitive matching for all rules:

```bash
highlite --config examples/rules/cpp_rules.yaml --ignore-case < examples/logs/example_cpp.cpp
```

If stdin is a TTY, highlite will wait for input until EOF is received.

## Configuration
The configuration file is written in YAML.

### Basic structure

```yaml
include:
  - common_optional.yaml

rules:
  - keyword: "TODO"
    color: { type: Yellow }
    ignore_case: true

  - keyword: "//.*|/\\*.*\\*/"
    is_regex: true
    ignore_case: false
    color: { r: 106, g: 153, b: 85 }
```

### Rules

Each rule has the following fields:

- `keyword`
  The keyword or regular expression to match.

- `is_regex` (optional, default: `false`)
  Whether `keyword` should be treated as a regular expression.

- `ignore_case` (optional, default: `false`)
  Whether this rule should match text case-insensitively.

  **Note**:
  If the CLI flag `--ignore-case` is provided, all rules will be treated as
  case-insensitive, regardless of this setting.
- 
- `color`
  The highlight color, either a preset name or an RGB value.

### Colors

#### Preset colors
```yaml
color: { type: Red }
```
```yaml
color: { type: Yellow }
```
```yaml
color: { type: Blue }
```
```yaml
color: { type: Green }
```
```yaml
color: { type: Cyan }
```
```yaml
color: { type: Magenta }
```

#### RGB colors

```yaml
color: { r: 106, g: 153, b: 85 }
```

### Examples

See `examples/rules`.


## Design

- All rules are merged into a single regular expression.

- Each rule corresponds to a named capture group.

- Case sensitivity is handled per rule using inline regex flags.

- Highlighting is performed in a single pass per line.

- Output buffers are reused to minimize allocations.

- This design keeps the implementation simple while maintaining high performance.

## Limitations
- No nested highlighting (for example, comments inside strings).

- No cross-line strings or comments (for example: multiline `/* */`).

- No language-aware parsing; matching is purely regex-based.

- ANSI color output requires a compatible terminal.

## License
This project is licensed under the GNU General Public License v3.0.

## Contributing
Issues and pull requests are welcome.

