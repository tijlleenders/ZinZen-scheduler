use super::Slot;

pub fn convert_into_1h_slots(slots: Vec<Slot>) -> Vec<Slot> {
    let mut all_slots: Vec<Slot> = vec![];
    for slot in slots.iter() {
        let mut slots_1h = slot.divide_into_1h_slots();
        all_slots.append(slots_1h.as_mut());
    }

    all_slots
}
