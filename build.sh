#!/bin/bash

# Build script for Markdown to PDF converter

echo "🔨 Compiling Markdown to PDF..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust is not installed!"
    echo "Please install from: https://rustup.rs/"
    exit 1
fi

# Check if Chrome/Chromium is installed
if ! command -v google-chrome &> /dev/null && ! command -v chromium &> /dev/null && ! command -v chromium-browser &> /dev/null; then
    echo "⚠️  Warning: Chrome/Chromium not found!"
    echo "The application needs Chrome to work."
    echo ""
    echo "Install with:"
    echo "  Ubuntu/Debian: sudo apt install chromium-browser"
    echo "  Fedora: sudo dnf install chromium"
    echo "  Arch: sudo pacman -S chromium"
    echo ""
fi

# Create project directory if it doesn't exist
PROJECT_DIR="markdown-to-pdf"
if [ ! -d "$PROJECT_DIR" ]; then
    echo "📁 Creating project structure..."
    cargo new "$PROJECT_DIR" --bin
    cd "$PROJECT_DIR"
else
    cd "$PROJECT_DIR"
fi

# Create src directory if it doesn't exist
mkdir -p src

# Check if files were created
if [ ! -f "Cargo.toml" ] || [ ! -f "src/main.rs" ]; then
    echo "❌ Error: Project files not found!"
    echo "Make sure Cargo.toml and src/main.rs exist."
    exit 1
fi

# Compile in release mode
echo "🚀 Compiling in release mode..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilation completed successfully!"
    
    # Strip executable to reduce size (Linux/Mac only)
    if [[ "$OSTYPE" == "linux-gnu"* ]] || [[ "$OSTYPE" == "darwin"* ]]; then
        echo "🔧 Optimizing executable..."
        strip target/release/md2pdf
    fi
    
    echo ""
    echo "📍 Executable created at: target/release/md2pdf"
    echo ""
    echo "📖 How to use:"
    echo "  ./target/release/md2pdf -i arquivo.md"
    echo "  ./target/release/md2pdf -i arquivo.md -o saida.pdf"
    echo "  ./target/release/md2pdf -i arquivo.md -m 75"
    echo ""
    echo "💡 Tip: Copy the executable to a PATH location:"
    echo "  sudo cp target/release/md2pdf /usr/local/bin/"
    
    # Create test example
    echo "📝 Creating example file..."
    cat > exemplo.md << 'EOF'
# Markdown Example

This is an **example** document to test the converter.

## Features

### Text Formatting

- **Bold**
- *Italic*
- `Inline code`

### Code Block

```python
def hello_world():
    """Example function with syntax highlighting"""
    print("Hello, world!")
    return 42

# Calling the function
result = hello_world()
print(f"The result is: {result}")
```

### Ordered List

1. First item
2. Second item
   - Subitem 2.1
   - Subitem 2.2
3. Third item

### Quote

> "Simplicity is the ultimate sophistication."
> — Leonardo da Vinci

### Table

| Language  | Year | Paradigm  |
|-----------|------|-----------|
| Python    | 1991 | Multi     |
| Rust      | 2010 | Systems   |
| JavaScript| 1995 | Multi     |

---

### Links and Images

Visit [Rust Lang](https://www.rust-lang.org/) for more information.

*End of example*
EOF
    
    echo "✅ File 'exemplo.md' created!"
    echo ""
    echo "🧪 Test the converter with:"
    echo "  ./target/release/md2pdf -i exemplo.md"
    
else
    echo "❌ Compilation error!"
    exit 1
fi