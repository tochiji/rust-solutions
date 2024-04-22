use clap::Parser;

#[derive(Parser)]
#[command(version = "0.1.0", about = "Rust echo")]
struct Cli {
    /// Input text
    #[arg(required = true)]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();

    let text = cli.text.join(" ");
    let omit_newline = if cli.omit_newline { "" } else { "\n" };
    print!("{}{}", text, omit_newline);
}
