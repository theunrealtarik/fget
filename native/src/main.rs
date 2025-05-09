use clap::Parser;
use lib::args::*;

use lib::core::download_file;
use lib::listener;

fn main() {
    let args = Args::parse();

    if args.listen {
        cliclack::log::info(format!("Listening 127.0.0.1:{}", listener::PORT)).unwrap();

        match listener::listen() {
            Ok((url, final_url)) => {
                handle_download_result(download_file(&args, url).or_else(|e| {
                    cliclack::log::remark(format!("Primary URL failed: {}. Trying fallback...", e))
                        .unwrap();
                    download_file(&args, final_url)
                }));
            }
            Err(err) => {
                cliclack::log::error(err.to_string()).unwrap();
                std::process::exit(1);
            }
        }
    }

    let url = lib::helpers::resolvers::url(&args);
    handle_download_result(download_file(&args, url));
}

fn handle_download_result(dl: Result<(), std::io::Error>) {
    match dl {
        Ok(_) => {
            cliclack::outro("Download completed successfully").unwrap();
            std::process::exit(0);
        }
        Err(e) => {
            cliclack::log::error(format!("Error: {}", e)).unwrap();
            std::process::exit(1);
        }
    }
}
