use clap::Parser;

#[derive(Parser, Debug)]
pub struct Options {
    #[clap(short, long)]
    pub endpoint: String,
    #[clap(short = 'N', long)]
    pub latitude: f64,
    #[clap(short = 'E', long)]
    pub longitude: f64,
    #[clap(short = 'f', long)]
    pub fixed_off: Option<u32>,
}
