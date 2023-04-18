use super::Slot;

pub fn convert_into_1h_slots(slots: Vec<Slot>) -> Vec<Slot> {
    let mut all_slots: Vec<Slot> = vec![];
    for slot in slots.iter() {
        let mut slots_1h = slot.divide_into_1h_slots();
        all_slots.append(slots_1h.as_mut());
    }

    all_slots

    // let mut slots_as_hours: Vec<Slot> = vec![];
    // for slot in all_slots.iter() {
    //     for hour in 0..slot.num_hours() {
    //         slots_as_hours.push(Slot {
    //             start: slot.start.add(chrono::Duration::hours(hour as i64)),
    //             end: slot.start.add(chrono::Duration::hours((hour + 1) as i64)),
    //         })
    //     }
    // }
    // slots_as_hours
}
