use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "tldr-rs", author)]
pub struct Arguments {
    /// Print version and exit
    #[clap(short, long)]
    pub version: bool,

    /// Update local database
    #[clap(short, long)]
    pub update: bool,

    /// Clear local database
    #[clap(short, long)]
    pub clear_cache: bool,

    /// List all entries in the local database
    #[clap(short, long)]
    pub list: bool,

    /// Select platform
    #[arg(value_enum)]
    #[clap(short, long)]
    pub platform: Option<Platform>,

    /// Render a local page for testing purposes
    #[clap(short, long, value_name = "PATH")]
    pub render: Option<String>,

    pub item: Option<String>,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Platform {
    Linux,
    Osx,
    Sunos,
    Windows,
    Common,
}

impl Platform {
    pub(crate) fn to_str(&self) -> &'static str {
        match self {
            Platform::Linux => "linux",
            Platform::Osx => "osx",
            Platform::Sunos => "sunos",
            Platform::Windows => "windows",
            Platform::Common => "common",
        }
    }
}
