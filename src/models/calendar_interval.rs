use std::collections::HashSet;
use std::fmt;
use std::fmt::Debug;

use crate::models::interval::Interval;

#[derive(Debug, Clone)]
pub struct CalendarInterval {
    pub interval: Interval,
    pub status: CalIntStatus,
}

impl CalendarInterval {
    pub(crate) fn claim_by(&mut self, act_index: usize) {
        self.status.claim_by(act_index);
    }
}

#[derive(Clone)]
pub enum CalIntStatus {
    Claimable(HashSet<usize>),
    Occupied(usize, String),
    //TODO: add goal id and budget id to occupied registration so budget object is not necessary anymore!locked,
}

impl CalIntStatus {
    pub(crate) fn claim_by(&mut self, act_index: usize) {
        match self {
            CalIntStatus::Claimable(ref mut claims) => claims.insert(act_index),
            CalIntStatus::Occupied { .. } => false,
        };
    }
}

impl Debug for CalIntStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalIntStatus::Claimable(claims) => {
                write!(f, "Number of claims: {}", claims.len()).expect("expecting result");
                for act_index in claims {
                    write!(f, "\nby {}", &act_index).expect("expecting result");
                }
                Ok(())
            }
            CalIntStatus::Occupied(act_index, goal_id) => {
                write!(
                    f,
                    "Occupied {{ act_index {}, goal_id ...{} }}",
                    act_index,
                    &goal_id[goal_id.len().saturating_sub(5)..]
                )
                .expect("expecting result");
                Ok(())
            }
        }
    }
}
