use num_format::{Locale, ToFormattedString};
use took::Timer;

use aoc::get_days;

fn main() {
    for day in get_days() {
        let timer = Timer::new();
        let ans = day.0();
        let duration = timer.took();
        println!(
            "{}: {} ... {}",
            day.1,
            ans.to_formatted_string(&Locale::en),
            duration
        );
    }
}
