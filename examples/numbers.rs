extern crate humanize;

use humanize::numbers::HumanizeNumbers;

fn main() {
	let scoreboard = ["Bob", "Victor", "Richard", "John",
	"Lisa"];

	for (i, name) in scoreboard.iter().enumerate() {
		println!("{}: {}", (i+1).ord(), name);
	}
}