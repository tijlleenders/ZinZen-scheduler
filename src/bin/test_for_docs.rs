extern crate scheduler;
use scheduler::models::input::Input;

fn main() {
    let json_input: serde_json::Value = serde_json::json!({
      "startDate": "2022-01-01T00:00:00",
      "endDate": "2022-01-09T00:00:00",
      "goals": {
        "uuid1": {
          "id": "uuid1",
          "title": "sleep",
          "min_duration": 8,
          "repeat": "daily",
          "filters": {
            "after_time": 22,
            "before_time": 8
          }
        }
      }
    });
    let input: Input = serde_json::from_value(json_input).unwrap();
    let output = scheduler::run_scheduler(input);
    dbg!(output);
}
