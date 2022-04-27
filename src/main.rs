use zinzen_scheduler::calendar::Calendar;

pub(crate) fn main() {
	let mut calendar = Calendar::new(168, String::from("h"));

	// print!("Calendar:{:#?}\n", calendar);

	// print!("\nexpect Calendar with two goals not overlapping\n");
	calendar.schedule();

	// calendar.print_slots_for_range(12, 14);
	println!("Calendar: {calendar:?}");
}
