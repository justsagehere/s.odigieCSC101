fn main() {
	let toshiba = 2 * 450_000;
	let mac = 1 * 1_500_000;
	let hp = 3 * 750_000;
	let dell = 3 * 2_850_000;
	let acer = 1 * 250_000;

	let a = toshiba + mac + hp + dell + acer;
	println!("sum is {}", a);
	let b = (a) / 5;
	println!("average is {}", b);
}