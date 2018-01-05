
fn main() {
	let mut a = 1;
	let mut b = 0;
	let mut c = 0;
	let mut d = 0;
	let mut e = 0;
	let mut f = 0;
	let mut g = 0;
	let mut h = 0;

	// Initialize the original value for b and c.
	b = 84;
	c = b;

	b = b*100 + 100000;
	c = b + 17000;

	// Look for prime.
	loop {
		f = 1;
		d = 2;
		e = 2;

		while d * d <= b {

			if b % d == 0 {
				f = 0;
				break;
			}

			d += 1;
		}

		if f == 0 {
			h += 1;
		}

		g = b - c;
		b += 17;

		if g == 0 {
			break;
		}

	}
	print!("{:?}", h);

}