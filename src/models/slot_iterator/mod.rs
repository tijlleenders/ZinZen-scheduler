pub mod filter;
pub mod iterator;
pub mod utils;

use super::{goal::TimeFilter, repetition::Repetition, slot::Slot};
use chrono::NaiveDateTime;

/* TODO 2023-04-20
Below proposed to resolving https://github.com/tijlleenders/ZinZen-scheduler/issues/289
- Will add a field in TimeSlotsIterator as below:
    ```rust
        /// Decide if slots can be split or will be a solid slot
        splittable: bool,
    ```

- When impl Iterator for TimeSlotsIterator, will check if splittable is false will add below logic:
    ```
    if next_start_position.le(&slot.end) && next_start_position.gt(&slot.start) {
        if self.splittable == false{
            next_start_position = slot.end;
            self.current_start_position = next_start_position;
            indexes_to_delete_count += 1;

            result.push(*slot);
        }
    }
    ```

*/

// derive Debug for TimeSlotsIterator
#[derive(Debug)]
/// An iterator that returns slots that conform to a repetition,
/// with optional filters (after/before/Mondays/Weekdays/Weekends/Bank holidays...)
/// e.g. iterate over all MONDAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all DAYS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all HOURS between 1st September 2022 and 30th September 2022.
/// e.g. iterate over all 'every two hours' slots between times 10-21 for time period X-Y
/// e.g. iterate in 24h steps over a time period that has certain filters applied, starting at after filter time value on day of the first available slot.
pub(crate) struct TimeSlotsIterator {
    timeline: Vec<Slot>,
    repetition: Option<Repetition>,
    filters: Option<TimeFilter>,
    current_start_position: NaiveDateTime,
}

impl TimeSlotsIterator {
    pub fn new(
        start: NaiveDateTime,
        end: NaiveDateTime,
        repetition: Option<Repetition>,
        filters: Option<TimeFilter>,
    ) -> TimeSlotsIterator {
        let mut result = TimeSlotsIterator {
            timeline: vec![Slot { start, end }],
            repetition,
            filters,
            current_start_position: start, //override after applying filters
        };
        result.apply_filters();
        result.current_start_position = result.timeline[0].start;
        result
    }
}
