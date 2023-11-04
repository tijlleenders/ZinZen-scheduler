use super::{Day, Goal, TimeFilter};
use crate::models::budget::Budget;
use crate::models::repetition::Repetition;
use chrono::NaiveDateTime;
use log::info;
use serde::{Deserialize, Deserializer};

impl From<String> for Day {
    fn from(day: String) -> Self {
        info!("From<String> day-string: {:?}", day);

        match day.to_lowercase().as_str() {
            "fri" => Day::Friday,
            "sat" => Day::Saturday,
            "sun" => Day::Sunday,
            "mon" => Day::Monday,
            "tue" => Day::Tuesday,
            "wed" => Day::Wednesday,
            "thu" => Day::Thursday,
            _ => panic!("Invalid day selection"),
        }
    }
}

impl From<Day> for String {
    fn from(day: Day) -> Self {
        info!("From<Days> day: {:?}", day);
        match day {
            Day::Friday => "Fri".into(),
            Day::Saturday => "Sat".into(),
            Day::Sunday => "Sun".into(),
            Day::Monday => "Mon".into(),
            Day::Tuesday => "Tue".into(),
            Day::Wednesday => "Wed".into(),
            Day::Thursday => "Thu".into(),
        }
    }
}

// Todo 2023-05-05  | Check all these setters - Why are they needed? Why public?
impl Goal {
    pub fn new(id: usize) -> Self {
        Self {
            id: id.to_string(),
            title: String::from("Test"),
            ..Default::default()
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn duration(mut self, min_duration: usize) -> Self {
        self.min_duration = Some(min_duration);
        self
    }

    pub fn repeat(mut self, repetition: Repetition) -> Self {
        self.repeat = Some(repetition);
        self
    }

    pub fn start(mut self, start: NaiveDateTime) -> Self {
        self.start = Some(start);
        self
    }

    pub fn deadline(mut self, deadline: NaiveDateTime) -> Self {
        self.deadline = Some(deadline);
        self
    }

    pub fn deserialize_budget_vec<'de, D>(deserializer: D) -> Result<Option<Vec<Budget>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<Vec<Budget>> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            if !s.is_empty() {
                return Ok(Some(s));
            }
        }
        Ok(None)
    }
}

// imple Disply for TimeFilter
impl std::fmt::Display for TimeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TimeFilter [ after_time: {:?}, before_time: {:?}, on_days: {:?}, not_on: {:?} ]",
            self.after_time, self.before_time, self.on_days, self.not_on
        )
    }
}

#[cfg(test)]
mod tests {
    mod serializing_deserializing {
        use crate::models::budget::Budget;
        use crate::models::budget::BudgetType::Weekly;
        use crate::models::goal::Goal;

        #[test]
        fn deserializing_of_empty_budgetlist_produces_none() {
            let goal_deserialized: Goal = serde_json::from_str(
                r#"{    
                  "id": "1",
                  "title": "testing new api",
                  "budgets": []
                  }"#,
            )
            .unwrap();
            assert_eq!(goal_deserialized.budgets, None);
        }

        #[test]
        fn deserializing_of_non_empty_budgetlist_produces_correct_budgetlist() {
            let goal_deserialized: Goal = serde_json::from_str(
                r#"{    
                  "id": "1",
                  "title": "testing new api",
                  "budgets": [{
                    "budget_type": "Weekly",
                    "min": 40
                    }]
                  }"#,
            )
            .unwrap();
            assert_eq!(
                goal_deserialized.budgets,
                Some(vec![Budget {
                    budget_type: Weekly,
                    min: Some(40),
                    max: None
                }])
            );
        }

        // test that we can add a 'created_at' field without breaking the deserialization
        // the unwrap() would panic if it was impossible
        #[test]
        fn extra_fields_are_ignored() {
            let _: Goal = serde_json::from_str(
                r#"{    
                  "id": "1",
                  "title": "testing new api",
                  "created_at": "2023-09-03T10:38:35.505Z"
                  }"#,
            )
            .unwrap();
        }
    }
}
