use chrono::Weekday;
use clap::Parser;
use std::str::FromStr;

#[derive(Parser, Debug)]
pub struct Options {
    #[clap(short, long)]
    pub endpoint: String,
    #[clap(short = 'N', long)]
    pub latitude: f64,
    #[clap(short = 'E', long)]
    pub longitude: f64,
    #[clap(short = 'o', long, default_value = "0")]
    pub offset: i64,
    #[clap(short = 'd', long, default_value = "Mon,Tue,Wed,Thu,Fri,Sat,Sun")]
    pub weekdays: Weekdays,
    #[clap(short = 'f', long)]
    pub fixed_off: Option<u32>,
}

#[derive(Debug)]
pub struct Weekdays(pub Vec<Weekday>);

impl FromStr for Weekdays {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|p| p.parse().map_err(|err| format!("{:?}", err)))
            .collect::<Result<_, _>>()
            .map(|ws| Weekdays(ws))
    }
}
