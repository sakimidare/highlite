# hilite

hilite 是一个用 Rust 编写的高性能、基于规则的命令行（CLI）文本着色工具，支持标准输入（stdin）和文件。

它采用逐行读取的方式，并使用 ANSI 转义码对匹配内容进行高亮显示，非常适合处理大文件、流式输入以及 Unix 风格的管道操作。

## 特性

- **高性能**：启动时将所有规则编译为单个正则表达式。
- **基于规则的高亮**：支持关键字或正则表达式匹配。
- **丰富的色彩支持**：支持预设 ANSI 颜色和 24 位 RGB 真彩色。
- **YAML 配置**：支持可选的递归包含（include）功能。
- **专为流式设计**：适用于标准输入、管道和大文件。
- **内存优化**：在处理过程中尽量减少内存分配。

## 安装

### 通过 crates.io 安装
```bash
cargo install hilite
```

### 从源码安装
```bash
git clone https://github.com/sakimidare/hilite.git
cd hilite
cargo build --release
```

## 使用方法

高亮显示标准输入：
```bash
cat example.c | hilite --config rules.yaml
```


高亮显示文件：
```bash
hilite --config rules.yaml --file example.c
```

忽略大小写：
```bash
hilite --config rules.yaml --ignore-case < input.txt
```

如果标准输入是一个终端（TTY），hilite 会持续等待输入直到接收到 EOF（文件结束符）。

## 配置
配置文件使用 YAML 格式。

### 基本结构

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

### 规则字段说明
- `keyword`: 要匹配的关键字或正则表达式。
- `is_regex`（可选，默认值：`false`）: 是否将 keyword 视为正则表达式。
- `color`: 高亮颜色，可以是预设名称或 RGB 数值。

### 颜色设置

#### 预设颜色
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

#### RGB 颜色
```yaml
color: { r: 106, g: 153, b: 85 }
```

## 设计原理
- 所有规则会被合并为一个单一的正则表达式。
- 每个规则对应一个命名的捕获组（named capture group）。
- 每一行文本仅需经过一次扫描即可完成高亮。
- 重用输出缓冲区以最小化内存分配。

## 局限性
- 不支持嵌套高亮（例如：字符串内部的注释）。
- 不支持跨行高亮（例如：多行注释 `/* */`）。
- 不具备语言感知的解析能力；匹配纯粹基于正则表达式。
- ANSI 颜色输出需要兼容的终端支持。

## 开源协议
本项目采用 GNU General Public License v3.0 协议。

## 贡献
欢迎提交 Issue 和 Pull Request。

