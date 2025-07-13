use anyhow::{Context, Result};
use clap::Parser;
use headless_chrome::{Browser, LaunchOptions};
use pulldown_cmark::{Event, Options, Parser as MdParser, Tag};
use std::fs;
use std::path::PathBuf;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::html::{styled_line_to_highlighted_html, IncludeBackground};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Arquivo Markdown de entrada
    #[arg(short, long)]
    input: PathBuf,

    /// Arquivo PDF de saída (padrão: mesmo nome do input com .pdf)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Margem da página em pixels (padrão: 50)
    #[arg(short, long, default_value = "50")]
    margin: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Define arquivo de saída
    let output_path = args.output.unwrap_or_else(|| {
        let mut path = args.input.clone();
        path.set_extension("pdf");
        path
    });

    // Lê o arquivo Markdown
    let markdown_content = fs::read_to_string(&args.input)
        .with_context(|| format!("Erro ao ler arquivo: {:?}", args.input))?;

    // Converte Markdown para HTML
    let html_content = markdown_to_html(&markdown_content)?;

    // Gera o PDF
    generate_pdf(&html_content, &output_path, args.margin)?;

    println!("✅ PDF gerado com sucesso: {:?}", output_path);
    Ok(())
}

fn markdown_to_html(markdown: &str) -> Result<String> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = MdParser::new_ext(markdown, options);
    let mut html_output = String::new();
    
    // Configuração do syntect
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["Solarized (dark)"];
    
    let mut code_block = String::new();
    let mut in_code_block = false;
    let mut code_lang = String::new();

    // Adiciona o CSS e estrutura HTML
    html_output.push_str(&get_html_template());
    html_output.push_str("<body><div class=\"container\">");

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_block.clear();
                code_lang = match kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                    _ => String::new(),
                };
            }
            Event::End(Tag::CodeBlock(_)) => {
                in_code_block = false;
                let highlighted = highlight_code(&code_block, &code_lang, &ps, theme)?;
                html_output.push_str(&format!(
                    "<div class=\"code-block\"><pre><code>{}</code></pre></div>",
                    highlighted
                ));
            }
            Event::Text(text) => {
                if in_code_block {
                    code_block.push_str(&text);
                } else {
                    html_output.push_str(&escape_html(&text));
                }
            }
            Event::Code(text) => {
                html_output.push_str(&format!("<code class=\"inline-code\">{}</code>", escape_html(&text)));
            }
            Event::Start(Tag::Heading(level, _, _)) => {
                let level_num = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };
                html_output.push_str(&format!("<h{}>", level_num));
            }
            Event::End(Tag::Heading(level, _, _)) => {
                let level_num = match level {
                    pulldown_cmark::HeadingLevel::H1 => 1,
                    pulldown_cmark::HeadingLevel::H2 => 2,
                    pulldown_cmark::HeadingLevel::H3 => 3,
                    pulldown_cmark::HeadingLevel::H4 => 4,
                    pulldown_cmark::HeadingLevel::H5 => 5,
                    pulldown_cmark::HeadingLevel::H6 => 6,
                };
                html_output.push_str(&format!("</h{}>", level_num));
            }
            Event::Start(Tag::Paragraph) => html_output.push_str("<p>"),
            Event::End(Tag::Paragraph) => html_output.push_str("</p>"),
            Event::Start(Tag::List(None)) => html_output.push_str("<ul>"),
            Event::End(Tag::List(None)) => html_output.push_str("</ul>"),
            Event::Start(Tag::List(Some(_))) => html_output.push_str("<ol>"),
            Event::End(Tag::List(Some(_))) => html_output.push_str("</ol>"),
            Event::Start(Tag::Item) => html_output.push_str("<li>"),
            Event::End(Tag::Item) => html_output.push_str("</li>"),
            Event::Start(Tag::BlockQuote) => html_output.push_str("<blockquote>"),
            Event::End(Tag::BlockQuote) => html_output.push_str("</blockquote>"),
            Event::Start(Tag::Emphasis) => html_output.push_str("<em>"),
            Event::End(Tag::Emphasis) => html_output.push_str("</em>"),
            Event::Start(Tag::Strong) => html_output.push_str("<strong>"),
            Event::End(Tag::Strong) => html_output.push_str("</strong>"),
            Event::Start(Tag::Link(_, url, title)) => {
                html_output.push_str(&format!(
                    "<a href=\"{}\" title=\"{}\">",
                    url,
                    escape_html(&title)
                ));
            }
            Event::End(Tag::Link(_, _, _)) => html_output.push_str("</a>"),
            Event::Start(Tag::Image(_, url, title)) => {
                html_output.push_str(&format!(
                    "<img src=\"{}\" alt=\"{}\" />",
                    url,
                    escape_html(&title)
                ));
            }
            Event::Start(Tag::Table(_)) => html_output.push_str("<table>"),
            Event::End(Tag::Table(_)) => html_output.push_str("</table>"),
            Event::Start(Tag::TableHead) => html_output.push_str("<thead>"),
            Event::End(Tag::TableHead) => html_output.push_str("</thead>"),
            Event::Start(Tag::TableRow) => html_output.push_str("<tr>"),
            Event::End(Tag::TableRow) => html_output.push_str("</tr>"),
            Event::Start(Tag::TableCell) => html_output.push_str("<td>"),
            Event::End(Tag::TableCell) => html_output.push_str("</td>"),
            Event::HardBreak => html_output.push_str("<br />"),
            Event::SoftBreak => html_output.push_str(" "),
            Event::Rule => html_output.push_str("<hr />"),
            _ => {}
        }
    }

    html_output.push_str("</div></body></html>");
    Ok(html_output)
}

fn highlight_code(code: &str, lang: &str, ps: &SyntaxSet, theme: &syntect::highlighting::Theme) -> Result<String> {
    let syntax = ps.find_syntax_by_token(lang)
        .or_else(|| ps.find_syntax_by_extension(lang))
        .unwrap_or_else(|| ps.find_syntax_plain_text());
    
    let mut highlighter = HighlightLines::new(syntax, theme);
    let mut highlighted = String::new();
    
    for line in LinesWithEndings::from(code) {
        let ranges: Vec<(Style, &str)> = highlighter
            .highlight_line(line, ps)
            .context("Erro ao destacar linha")?;
        let html = styled_line_to_highlighted_html(&ranges[..], IncludeBackground::No)?;
        highlighted.push_str(&html);
    }
    
    Ok(highlighted)
}

fn escape_html(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn get_html_template() -> String {
    r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <style>
        @import url('https://fonts.googleapis.com/css2?family=Crimson+Text:ital,wght@0,400;0,600;0,700;1,400&family=Inter:wght@400;500;600;700&display=swap');
        
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        
        body {
            font-family: 'Crimson Text', serif;
            line-height: 1.8;
            color: #2c3e50;
            background-color: #fdfcfb;
            font-size: 18px;
        }
        
        .container {
            max-width: 800px;
            margin: 0 auto;
            padding: 60px 40px;
        }
        
        h1, h2, h3, h4, h5, h6 {
            font-family: 'Inter', sans-serif;
            color: #1a202c;
            margin-top: 2.5em;
            margin-bottom: 0.8em;
            font-weight: 700;
            line-height: 1.3;
        }
        
        h1 {
            font-size: 2.5em;
            border-bottom: 3px solid #e74c3c;
            padding-bottom: 0.3em;
            margin-bottom: 1em;
        }
        
        h2 {
            font-size: 1.9em;
            color: #2c3e50;
        }
        
        h3 {
            font-size: 1.5em;
            color: #34495e;
        }
        
        p {
            margin-bottom: 1.5em;
            text-align: justify;
            hyphens: auto;
        }
        
        a {
            color: #3498db;
            text-decoration: none;
            border-bottom: 1px solid transparent;
            transition: border-bottom 0.3s ease;
        }
        
        a:hover {
            border-bottom-color: #3498db;
        }
        
        code.inline-code {
            font-family: 'Consolas', 'Monaco', monospace;
            background-color: #f4f4f5;
            color: #e74c3c;
            padding: 0.2em 0.4em;
            border-radius: 3px;
            font-size: 0.9em;
        }
        
        .code-block {
            background-color: #1e1e1e;
            border-radius: 8px;
            padding: 1.5em;
            margin: 1.5em 0;
            overflow-x: auto;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        
        .code-block pre {
            margin: 0;
            font-family: 'Consolas', 'Monaco', monospace;
            font-size: 0.9em;
            line-height: 1.6;
        }
        
        .code-block code {
            color: #d4d4d4;
            background: none;
            padding: 0;
        }
        
        blockquote {
            border-left: 4px solid #e74c3c;
            padding-left: 1.5em;
            margin: 1.5em 0;
            font-style: italic;
            color: #555;
            background-color: #f9f9f9;
            padding: 1em 1.5em;
            border-radius: 0 8px 8px 0;
        }
        
        ul, ol {
            margin-bottom: 1.5em;
            padding-left: 2em;
        }
        
        li {
            margin-bottom: 0.5em;
        }
        
        hr {
            border: none;
            border-top: 2px solid #ecf0f1;
            margin: 3em 0;
        }
        
        table {
            width: 100%;
            border-collapse: collapse;
            margin: 1.5em 0;
            font-size: 0.95em;
        }
        
        th, td {
            padding: 0.75em;
            text-align: left;
            border-bottom: 1px solid #ecf0f1;
        }
        
        th {
            background-color: #34495e;
            color: white;
            font-family: 'Inter', sans-serif;
            font-weight: 600;
        }
        
        tr:nth-child(even) {
            background-color: #f8f9fa;
        }
        
        img {
            max-width: 100%;
            height: auto;
            border-radius: 8px;
            margin: 1.5em 0;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }
        
        @media print {
            body {
                font-size: 16px;
            }
            
            .container {
                padding: 20px;
            }
            
            h1, h2, h3, h4, h5, h6 {
                page-break-after: avoid;
            }
            
            .code-block {
                page-break-inside: avoid;
            }
        }
    </style>
</head>
"#.to_string()
}

#[tokio::main]
async fn generate_pdf(html: &str, output_path: &PathBuf, margin: u32) -> Result<()> {
    // Salva HTML temporário
    let temp_html = output_path.with_extension("html");
    fs::write(&temp_html, html)?;
    
    // Configura o navegador
    let options = LaunchOptions {
        headless: true,
        sandbox: false,
        enable_gpu: false,
        ..Default::default()
    };
    
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;
    
    // Carrega o HTML
    let file_url = format!("file://{}", temp_html.display());
    tab.navigate_to(&file_url)?;
    tab.wait_until_navigated()?;
    
    // Espera o conteúdo carregar
    std::thread::sleep(std::time::Duration::from_secs(2));
    
    // Gera o PDF
    let pdf_options = headless_chrome::protocol::cdp::Page::PrintToPdfParams {
        landscape: Some(false),
        display_header_footer: Some(false),
        print_background: Some(true),
        scale: Some(1.0),
        paper_width: Some(8.5),
        paper_height: Some(11.0),
        margin_top: Some(margin as f64 / 96.0),
        margin_bottom: Some(margin as f64 / 96.0),
        margin_left: Some(margin as f64 / 96.0),
        margin_right: Some(margin as f64 / 96.0),
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: None,
        prefer_css_page_size: Some(true),
        transfer_mode: None,
    };
    
    let pdf_data = tab.print_to_pdf(Some(pdf_options))?;
    fs::write(output_path, pdf_data)?;
    
    // Remove arquivo temporário
    fs::remove_file(&temp_html)?;
    
    Ok(())
}