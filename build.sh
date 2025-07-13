#!/bin/bash

# Script de build para o conversor Markdown to PDF

echo "🔨 Compilando Markdown to PDF..."

# Verifica se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Erro: Rust não está instalado!"
    echo "Por favor, instale em: https://rustup.rs/"
    exit 1
fi

# Verifica se o Chrome/Chromium está instalado
if ! command -v google-chrome &> /dev/null && ! command -v chromium &> /dev/null && ! command -v chromium-browser &> /dev/null; then
    echo "⚠️  Aviso: Chrome/Chromium não encontrado!"
    echo "O aplicativo precisa do Chrome para funcionar."
    echo ""
    echo "Instale com:"
    echo "  Ubuntu/Debian: sudo apt install chromium-browser"
    echo "  Fedora: sudo dnf install chromium"
    echo "  Arch: sudo pacman -S chromium"
    echo ""
fi

# Cria diretório do projeto se não existir
PROJECT_DIR="markdown-to-pdf"
if [ ! -d "$PROJECT_DIR" ]; then
    echo "📁 Criando estrutura do projeto..."
    cargo new "$PROJECT_DIR" --bin
    cd "$PROJECT_DIR"
else
    cd "$PROJECT_DIR"
fi

# Cria diretório src se não existir
mkdir -p src

# Verifica se os arquivos foram criados
if [ ! -f "Cargo.toml" ] || [ ! -f "src/main.rs" ]; then
    echo "❌ Erro: Arquivos do projeto não encontrados!"
    echo "Certifique-se de que Cargo.toml e src/main.rs existem."
    exit 1
fi

# Compila em modo release
echo "🚀 Compilando em modo release..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilação concluída com sucesso!"
    
    # Strip do executável para reduzir tamanho (apenas Linux/Mac)
    if [[ "$OSTYPE" == "linux-gnu"* ]] || [[ "$OSTYPE" == "darwin"* ]]; then
        echo "🔧 Otimizando executável..."
        strip target/release/md2pdf
    fi
    
    echo ""
    echo "📍 Executável criado em: target/release/md2pdf"
    echo ""
    echo "📖 Como usar:"
    echo "  ./target/release/md2pdf -i arquivo.md"
    echo "  ./target/release/md2pdf -i arquivo.md -o saida.pdf"
    echo "  ./target/release/md2pdf -i arquivo.md -m 75"
    echo ""
    echo "💡 Dica: Copie o executável para um local no PATH:"
    echo "  sudo cp target/release/md2pdf /usr/local/bin/"
    
    # Cria exemplo de teste
    echo "📝 Criando arquivo de exemplo..."
    cat > exemplo.md << 'EOF'
# Exemplo de Markdown

Este é um documento de **exemplo** para testar o conversor.

## Funcionalidades

### Formatação de Texto

- **Negrito**
- *Itálico*
- `Código inline`

### Bloco de Código

```python
def hello_world():
    """Função de exemplo com syntax highlighting"""
    print("Olá, mundo!")
    return 42

# Chamando a função
resultado = hello_world()
print(f"O resultado é: {resultado}")
```

### Lista Ordenada

1. Primeiro item
2. Segundo item
   - Subitem 2.1
   - Subitem 2.2
3. Terceiro item

### Citação

> "A simplicidade é o último grau de sofisticação."
> — Leonardo da Vinci

### Tabela

| Linguagem | Ano | Paradigma |
|-----------|-----|-----------|
| Python    | 1991| Multi     |
| Rust      | 2010| Sistemas  |
| JavaScript| 1995| Multi     |

---

### Links e Imagens

Visite [Rust Lang](https://www.rust-lang.org/) para mais informações.

*Fim do exemplo*
EOF
    
    echo "✅ Arquivo 'exemplo.md' criado!"
    echo ""
    echo "🧪 Teste o conversor com:"
    echo "  ./target/release/md2pdf -i exemplo.md"
    
else
    echo "❌ Erro na compilação!"
    exit 1
fi