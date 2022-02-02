use clap::Parser;

#[derive(Parser, Debug)]
pub struct Options {
    #[clap(short = 'e', long)]
    pub endpoint: String,
    #[clap(short = 'a', long)]
    pub authorization: Option<String>,
    #[clap(short = 'd', long)]
    pub cooldown: Option<u64>,
}
