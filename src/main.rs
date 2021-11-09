use chrono::{DateTime, Utc};
use std::str::FromStr;
use zinzen_scheduler::{self};

fn main() {
    let test_goal = zinzen_scheduler::Goal::new();
    print!("title:{}\n", test_goal.title);

    let test_goal2 = zinzen_scheduler::Goal::from_str("");
    match test_goal2 {
        Ok(goal) => print!("title:{}\n", goal.title),
        Err(error) => print!("title:{}", error),
    }

    let test_goal3 = zinzen_scheduler::Goal::from_str("goal 3");
    match test_goal3 {
        Ok(goal) => print!("title:{}\n", goal.title),
        Err(error) => print!("title:{}", error),
    }

    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!(
        "UTC now in a custom format is: {}",
        now.format("%a %b %e %T %Y")
    );

    let rfc3339 = DateTime::parse_from_rfc3339("2021-12-13T13:37:00+01:00").expect("oops!");
    println!("Parsed ISO String is:{}", rfc3339);
}
