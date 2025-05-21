use clap::Parser;

pub fn parse_cli_args() -> Args {
    Args::parse()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub network_id: u128,

    #[arg(short, long)]
    pub endpoint: Option<String>,

    #[arg(short, long)]
    pub auth_key: Option<String>,

    #[arg(short, long)]
    pub out_path: Option<String>,
}
