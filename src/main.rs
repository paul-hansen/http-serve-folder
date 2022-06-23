#![deny(missing_docs)]
//! A configurable HTTP server that hosts a folder. Suitable for local testing.
use clap::Parser;
use std::fmt::Formatter;
use std::net::IpAddr;
use std::path::PathBuf;
use tracing::info;
use tracing::level_filters::LevelFilter;
use warp::http::header::HeaderName;
use warp::hyper::HeaderMap;
use warp::Filter;

#[derive(clap::Parser)]
struct Args {
    #[clap(value_enum)]
    level: Level,
}

#[derive(clap::ValueEnum, Clone)]
enum Level {
    Debug,
    Info,
    Warn,
    Error,
    Off,
}

impl From<Level> for LevelFilter {
    fn from(l: Level) -> Self {
        match l {
            Level::Debug => LevelFilter::DEBUG,
            Level::Info => LevelFilter::INFO,
            Level::Warn => LevelFilter::WARN,
            Level::Error => LevelFilter::ERROR,
            Level::Off => LevelFilter::OFF,
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::Info
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Debug => write!(f, "debug"),
            Level::Info => write!(f, "info"),
            Level::Warn => write!(f, "warn"),
            Level::Error => write!(f, "error"),
            Level::Off => write!(f, "off"),
        }
    }
}

///
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// The folder you want to host
    #[clap(value_parser, value_hint = clap::ValueHint::DirPath)]
    dir: Option<PathBuf>,
    /// Headers to add to all file responses.
    ///
    /// Should contain a colon to separate the name and value for the header.
    /// Can be included multiple times. E.g. -H "NAME: VALUE" -H "NAME2: VALUE2"
    #[clap(value_parser, short = 'H', long = "header")]
    headers: Vec<String>,
    /// The IP Address to bind to.
    #[clap(value_parser, short = 'i', long = "ip_address", default_value_t = IpAddr::from([127, 0, 0, 1]))]
    ip_address: IpAddr,
    /// The port number to bind to.
    #[clap(value_parser, short = 'p', long = "port", default_value_t = 4040)]
    port: u16,
    /// The level of logging to display.
    #[clap(value_parser, short = 'l', long = "log", default_value_t)]
    log: Level,
}

#[tokio::main]
async fn main() {
    let cli: Cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::from(cli.log))
        .init();

    let mut headers = HeaderMap::new();
    cli.headers.iter().for_each(|h| {
        let (name, value) = h
            .split_once(':')
            .unwrap_or_else(|| panic!("Failed to parse custom header {h}"));
        headers.insert(
            HeaderName::try_from(name.to_string().trim()).unwrap(),
            value.trim().parse().unwrap(),
        );
    });
    info!("Using custom headers: {headers:?}");

    let main = warp::fs::dir(cli.dir.unwrap_or_else(|| ".".into()))
        .with(warp::reply::with::headers(headers))
        .with(warp::log::log("TEST"));

    warp::serve(main).run((cli.ip_address, cli.port)).await;
}
