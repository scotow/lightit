use chrono::{DateTime, Datelike, Duration, Local, Weekday};
use lightit::State;
use std::time::Duration as StdDuration;

pub struct Scheduler {
    pub latitude: f64,
    pub longitude: f64,
    pub offset: i64,
    pub weekdays: Vec<Weekday>,
    pub fixed_off: Vec<u32>,
}

impl Scheduler {
    pub fn next(&self) -> Option<(StdDuration, State)> {
        let now: DateTime<Local> = Local::now();
        (0..)
            .map(|day| now + Duration::days(day))
            .filter(|day| self.weekdays.contains(&day.weekday()))
            .take(2)
            .flat_map(|date_time| {
                let (sunrise, sunset) = sunrise::sunrise_sunset(
                    self.latitude,
                    self.longitude,
                    date_time.year(),
                    date_time.month(),
                    date_time.day(),
                );
                let mut points = vec![
                    (sunrise + self.offset * 60, State::Off),
                    (sunset - self.offset * 60, State::On),
                ];
                for &off_hour in self.fixed_off.iter() {
                    points.push((
                        date_time.date().and_hms(off_hour, 0, 0).timestamp(),
                        State::Off,
                    ));
                }
                points.sort_by_key(|&(ut, _)| ut);
                points
            })
            .find(|&(ut, _)| ut > now.timestamp())
            .map(|(ut, s)| (StdDuration::from_secs((ut - now.timestamp()) as u64), s))
    }
}
