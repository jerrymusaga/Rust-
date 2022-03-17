
enum TimeUnit {
    Seconds, Minutes, Hours, Days, Months, Years
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32)
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years"
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_right_matches('s')
    }
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InTheFuture(units, 1) =>
            format!("a {} from now", units.singular()),
        RoughTime::InThePast(units,count) => 
            format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => 
            format!("Just now"),
        RoughTime::InTheFuture(units,count) =>
            format!("{} {} ago", count, units.plural()),
    }
}


fn main() {
   let future_min = RoughTime::InTheFuture(TimeUnit::Minutes, 1);
   println!("{}",rough_time_to_english(future_min));
}
