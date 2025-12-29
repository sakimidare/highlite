use anyhow::Context;
use regex::{Regex, RegexBuilder};
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, IsTerminal, Write};

// --- Modules ---

pub mod rules {
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize)]
    pub struct Rule {
        pub keyword: String,
        pub color: Color,
        #[serde(default)]
        pub is_regex: bool,
    }

    #[derive(Debug, Copy, Clone, Deserialize)]
    #[serde(tag = "type", rename_all = "PascalCase", content = "value")]
    pub enum PresetColor {
        Red,
        Yellow,
        Blue,
        Green,
        Cyan,
        Magenta,
    }

    #[derive(Debug, Copy, Clone, Deserialize)]
    #[serde(untagged)]
    pub enum Color {
        Preset(PresetColor),
        RGB { r: u8, g: u8, b: u8 },
    }

    impl Color {
        pub fn to_ansi(&self) -> String {
            match self {
                Color::Preset(p) => match p {
                    PresetColor::Red => "\x1b[31m".to_string(),
                    PresetColor::Yellow => "\x1b[33m".to_string(),
                    PresetColor::Blue => "\x1b[34m".to_string(),
                    PresetColor::Green => "\x1b[32m".to_string(),
                    PresetColor::Cyan => "\x1b[36m".to_string(),
                    PresetColor::Magenta => "\x1b[35m".to_string(),
                },
                Color::RGB { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
            }
        }
    }
}

pub mod arg_parser {
    use crate::rules::Rule;
    use clap::Parser;
    use serde::Deserialize;
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[derive(Debug, Parser)]
    #[command(name = "hilite", about = "Highlight lines from stdin or a file")]
    pub struct CliArgs {
        #[arg(short, long)]
        pub ignore_case: bool,

        #[arg(short, long, help = "Path to the input file (defaults to stdin)")]
        pub file: Option<PathBuf>,

        #[arg(short, long, help = "Path to the YAML config file (required)")]
        pub config: Option<PathBuf>,
    }

    #[derive(Debug, Deserialize)]
    pub struct FileConfig {
        pub include: Option<Vec<String>>,
        pub rules: Option<Vec<Rule>>,
    }

    pub fn load_rules_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<Rule>> {
        let mut loaded_files = HashSet::new();
        load_rules_recursive(path.as_ref(), &mut loaded_files)
    }

    fn load_rules_recursive(
        path: &Path,
        loaded: &mut HashSet<String>,
    ) -> anyhow::Result<Vec<Rule>> {
        let canonical_path = fs::canonicalize(path)?.to_string_lossy().to_string();

        if !loaded.insert(canonical_path) {
            return Ok(vec![]);
        }

        let text = fs::read_to_string(path)?;
        let file_config: FileConfig = serde_yml::from_str(&text)?;
        let mut all_rules = Vec::new();

        if let Some(includes) = file_config.include {
            let parent_dir = path.parent().unwrap_or_else(|| Path::new("."));
            for inc_path in includes {
                let full_path = parent_dir.join(inc_path);
                all_rules.append(&mut load_rules_recursive(&full_path, loaded)?);
            }
        }

        if let Some(current_rules) = file_config.rules {
            all_rules.extend(current_rules);
        }

        Ok(all_rules)
    }
}

// --- Optimized Processor ---
pub mod highlight {
    pub struct HighlightingEngine {
        regex: crate::Regex,
        ansi_colors: Vec<String>,
    }

    impl HighlightingEngine {
        pub fn new(rules: &[crate::rules::Rule], ignore_case: bool) -> anyhow::Result<Self> {
            let mut patterns = Vec::with_capacity(rules.len());
            let mut ansi_colors = Vec::with_capacity(rules.len());

            for (i, rule) in rules.iter().enumerate() {
                let pat = if rule.is_regex {
                    rule.keyword.clone()
                } else {
                    regex::escape(&rule.keyword)
                };
                // 使用命名捕获组 rN 以便匹配后快速索引颜色
                patterns.push(format!(r"(?P<r{}>{})", i, pat));
                ansi_colors.push(rule.color.to_ansi());
            }

            let combined_re = crate::RegexBuilder::new(&patterns.join("|"))
                .case_insensitive(ignore_case)
                .multi_line(true)
                .dot_matches_new_line(false)
                .build()?;

            Ok(Self {
                regex: combined_re,
                ansi_colors,
            })
        }

        pub fn render_line(&self, input: &str, output: &mut String) {
            output.clear();
            let mut last_match = 0;

            for caps in self.regex.captures_iter(input) {
                let whole_match = caps.get(0).unwrap();

                // 写入匹配项之前的文本
                output.push_str(&input[last_match..whole_match.start()]);

                // 寻找是哪个规则触发了匹配
                for (i, color_code) in self.ansi_colors.iter().enumerate() {
                    if let Some(m) = caps.name(&format!("r{}", i)) {
                        output.push_str(color_code);
                        output.push_str(m.as_str());
                        output.push_str("\x1b[0m");
                        break;
                    }
                }
                last_match = whole_match.end();
            }
            // 写入剩余文本
            output.push_str(&input[last_match..]);
        }
    }
}
// --- Main Logic ---

pub fn run(cli_args: arg_parser::CliArgs) -> anyhow::Result<()> {
    let config_path = cli_args
        .config
        .context("Missing config file. Use --config <PATH>")?;
    let raw_rules = arg_parser::load_rules_from_file(&config_path)?;

    // 1. 预编译引擎
    let engine = highlight::HighlightingEngine::new(&raw_rules, cli_args.ignore_case)?;

    // 2. 准备带缓冲的输出
    let stdout = std::io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    // 3. 处理输入
    if let Some(path) = cli_args.file {
        let f = fs::File::open(path)?;
        process_stream(BufReader::new(f), &engine, &mut writer)?;
    } else {
        if std::io::stdin().is_terminal() {
            eprintln!("(Info: Waiting for stdin... Press Ctrl+D to end)");
        }
        process_stream(BufReader::new(std::io::stdin()), &engine, &mut writer)?;
    }

    writer.flush()?;
    Ok(())
}

fn process_stream<R: BufRead, W: Write>(
    mut reader: R,
    engine: &highlight::HighlightingEngine,
    writer: &mut W,
) -> anyhow::Result<()> {
    let mut line_buffer = String::new();
    let mut out_buffer = String::new();

    // 循环复用 String 内存，避免每行都分配内存
    while reader.read_line(&mut line_buffer)? > 0 {
        engine.render_line(&line_buffer, &mut out_buffer);
        writer.write_all(out_buffer.as_bytes())?;
        line_buffer.clear();
    }
    Ok(())
}
