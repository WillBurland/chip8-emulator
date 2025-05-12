pub struct Input {
	pub zero: bool,
	pub one: bool,
	pub two: bool,
	pub three: bool,
	pub four: bool,
	pub five: bool,
	pub six: bool,
	pub seven: bool,
	pub eight: bool,
	pub nine: bool,
	pub a: bool,
	pub b: bool,
	pub c: bool,
	pub d: bool,
	pub e: bool,
	pub f: bool,
}

impl Input {
	pub fn new() -> Self {
		Input {
			zero: false,
			one: false,
			two: false,
			three: false,
			four: false,
			five: false,
			six: false,
			seven: false,
			eight: false,
			nine: false,
			a: false,
			b: false,
			c: false,
			d: false,
			e: false,
			f: false,
		}
	}

    pub fn check_inputs(&mut self) {
        // todo
    }
}