# hilite

hilite is a fast, rule-based CLI highlighter for stdin and files, written in Rust.

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

---

## Installation

### From crates.io

```bash
cargo install hilite
```

### From source

```bash
git clone https://github.com/sakimidare/hilite.git
cd hilite
cargo build --release
```

## Usage

Highlight stdin:

```bash
cat example.c | hilite --config rules.yaml
```

Highlight a file:

```bash
hilite --config rules.yaml --file example.c
```

Ignore case:

```bash
hilite --config rules.yaml --ignore-case < input.txt
```

If stdin is a TTY, hilite will wait for input until EOF is received.

## Configuration
The configuration file is written in YAML.

### Basic structure

```yaml
include:
  - common_optional.yaml

rules:
  - keyword: "TODO"
    color: { type: Yellow }

  - keyword: "//.*|/\\*.*\\*/"
    is_regex: true
    color: { r: 106, g: 153, b: 85 }
```

### Rules

Each rule has the following fields:

- `keyword`
  The keyword or regukar expression to match.

- `is_regex` (optional, default: `false`)
  Whether `keyword` should be treated as a regular expression.

- `color`
  The highlight color, either a preset name or an RGB value.

### Colors

#### Preset colors
```yaml
color: { type: Red }
color: { type: Yellow }
color: { type: Blue }
color: { type: Green }
color: { type: Cyan }
color: { type: Magenta }
```

#### RGB colors

```yaml
color: { r: 106, g: 153, b: 85 }
```

### Examples

See `examples/cpp_example.yaml`.


## Design

- All rules are merged into a single regular expression.

- Each rule corresponds to a named capture group.

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

