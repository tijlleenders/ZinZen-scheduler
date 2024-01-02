use chrono::NaiveDateTime;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Hour {
    Free,
    Occupied { activity_id: usize },
}

pub struct Calendar {
    pub start_date_time: NaiveDateTime,
    pub end_date_time: NaiveDateTime,
    pub hours: Vec<Rc<Hour>>,
}

impl Calendar {
    pub fn new(start_date_time: NaiveDateTime, end_date_time: NaiveDateTime) -> Self {
        let mut hours = Vec::with_capacity(24);
        for _ in 0..24 {
            hours.push(Rc::new(Hour::Free));
        }
        Self {
            start_date_time,
            end_date_time,
            hours,
        }
    }
}

impl Debug for Calendar {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for index in 0..self.hours.capacity() {
            if self.hours[index] == Rc::new(Hour::Free) {
                if Rc::weak_count(&self.hours[index]) == 0 {
                    write!(f, "{} -\n", index).unwrap();
                } else {
                    write!(
                        f,
                        "{} {:?} claims\n",
                        index,
                        Rc::weak_count(&self.hours[index])
                    )
                    .unwrap();
                }
            } else {
                write!(f, "{} {:?}\n", index, self.hours[index]).unwrap();
            }
        }
        Ok(())
    }
}
