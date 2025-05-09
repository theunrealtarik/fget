use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Provide the file's direct download link
    #[arg(long)]
    pub url: Option<String>,

    /// Explicitly sets the downloaded file's name
    #[arg(long, short)]
    pub filename: Option<String>,

    /// Listen for incomming download requests from your browser
    #[arg(long, short, default_value_t = false)]
    pub listen: bool,
}
