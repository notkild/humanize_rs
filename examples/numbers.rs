extern crate humanize;
#[allow(unused_imports)]
use humanize::numbers::{HumanizeNumbers, to_text};

fn test_to_string(num: u64) {
	println!{"{}", to_text(num)};
}

fn main() {
	// let scoreboard = ["Bob", "Victor", "Richard", "John",
	// "Lisa"];

	// for (i, name) in scoreboard.iter().enumerate() {
	// 	println!("{}: {}", (i+1).ord(), name);
	// }
	let num: u64 = 53286397154;
	test_to_string(num);

}