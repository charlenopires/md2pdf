# md2pdf - Markdown to PDF Converter

A fast and elegant Markdown to PDF converter built with Rust, featuring syntax highlighting and beautiful typography.

## Features

- 🎨 **Beautiful Typography** - Professional document styling with Crimson Text and Inter fonts
- 🌈 **Syntax Highlighting** - Code blocks with syntax highlighting using Syntect
- 📱 **Responsive Design** - Optimized for both screen and print
- ⚡ **Fast Conversion** - Built with Rust for maximum performance
- 🎯 **Easy CLI** - Simple command-line interface
- 📄 **Rich Markdown Support** - Tables, lists, blockquotes, images, and more

## Installation

### Prerequisites

- **Rust**: Install from [https://rustup.rs/](https://rustup.rs/)
- **Chrome/Chromium**: Required for PDF generation
  - Ubuntu/Debian: `sudo apt install chromium-browser`
  - Fedora: `sudo dnf install chromium`
  - Arch: `sudo pacman -S chromium`
  - macOS: Install Chrome from official website
  - Windows: Install Chrome from official website

### Build from Source

1. Clone the repository:
```bash
git clone <repository-url>
cd md2pdf
```

2. Build the project:
```bash
./build.sh
```

Or manually:
```bash
cargo build --release
```

3. The executable will be created at `target/release/md2pdf`

### Install to System

Copy the executable to a directory in your PATH:
```bash
sudo cp target/release/md2pdf /usr/local/bin/
```

## Usage

### Basic Usage

```bash
# Convert markdown file to PDF
md2pdf -i document.md

# Specify output file
md2pdf -i document.md -o output.pdf

# Custom page margins (in pixels)
md2pdf -i document.md -m 75
```

### Command Line Options

```
USAGE:
    md2pdf [OPTIONS] --input <INPUT>

OPTIONS:
    -i, --input <INPUT>      Input Markdown file
    -o, --output <OUTPUT>    Output PDF file (default: same name as input with .pdf)
    -m, --margin <MARGIN>    Page margin in pixels (default: 50)
    -h, --help              Print help information
    -V, --version           Print version information
```

### Examples

```bash
# Convert README.md to README.pdf
md2pdf -i README.md

# Convert with custom output name
md2pdf -i documentation.md -o manual.pdf

# Convert with larger margins
md2pdf -i article.md -m 100

# Convert guide with custom name and margins
md2pdf -i guide.md -o user-guide.pdf -m 75
```

## Supported Markdown Features

- **Headers** (H1-H6) with elegant typography
- **Text formatting** (bold, italic, strikethrough)
- **Code blocks** with syntax highlighting
- **Inline code** with distinct styling
- **Lists** (ordered and unordered)
- **Tables** with alternating row colors
- **Blockquotes** with left border styling
- **Links** with hover effects
- **Images** with rounded corners and shadows
- **Horizontal rules**
- **Task lists**
- **Footnotes**

## Configuration

The converter uses a built-in CSS template optimized for PDF generation. The styling includes:

- **Fonts**: Crimson Text (serif) for body text, Inter (sans-serif) for headings
- **Colors**: Professional color scheme with good contrast
- **Layout**: Responsive design with proper spacing
- **Code**: Dark theme syntax highlighting
- **Print**: Optimized styles for PDF output

## Dependencies

- `pulldown-cmark` - Markdown parsing
- `syntect` - Syntax highlighting
- `headless_chrome` - PDF generation
- `tokio` - Async runtime
- `clap` - Command line argument parsing
- `anyhow` - Error handling

## Development

### Project Structure

```
md2pdf/
├── src/
│   └── main.rs          # Main application code
├── md/                  # Example markdown files
├── Cargo.toml          # Rust dependencies
├── build.sh            # Build script
└── README.md           # This file
```

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Creating a Release Build

```bash
cargo build --release
```

## Example Output

The converter generates professional-looking PDFs with:

- Clean, readable typography
- Proper spacing and margins
- Syntax-highlighted code blocks
- Responsive tables
- Styled blockquotes and lists

## Troubleshooting

### Chrome/Chromium Not Found

If you get an error about Chrome/Chromium not being found:

1. Install Chrome or Chromium using your system's package manager
2. Make sure it's accessible in your PATH
3. On some systems, you may need to install additional dependencies

### Permission Errors

If you encounter permission errors:

```bash
chmod +x target/release/md2pdf
```

### Large Files

For very large markdown files, you may need to increase the timeout or process them in smaller chunks.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Changelog

### v0.1.0
- Initial release
- Basic Markdown to PDF conversion
- Syntax highlighting support
- Professional typography
- Command line interface

## Acknowledgments

- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) for Markdown parsing
- [syntect](https://github.com/trishume/syntect) for syntax highlighting
- [headless_chrome](https://github.com/atroche/rust-headless-chrome) for PDF generation
- [Crimson Text](https://fonts.google.com/specimen/Crimson+Text) and [Inter](https://fonts.google.com/specimen/Inter) fonts