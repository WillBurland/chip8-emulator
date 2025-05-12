pub fn dump_memory_hex(memory: &[u8], start: usize) {
	for i in 0..memory.len() {
		if i % 8 == 0 {
			print!(" ")
		}
		if i % 32 == 0 {
			print!("\n{:04x}: ", i + start);
		}
		print!("{:02x} ", memory[i]);
	}
	print!("\n");
}

pub fn dump_memory_dec(memory: &[u8], start: usize) {
	for i in 0..memory.len() {
		if i % 8 == 0 {
			print!(" ")
		}
		if i % 32 == 0 {
			print!("\n{:04x}: ", i + start);
		}
		print!("{:03} ", memory[i]);
	}
	print!("\n");
}

pub fn draw_display(display_data: &[u8; 64 * 32]) {
	for y in 0..32 {
		let mut line:String = String::new();
		for x in 0..64 {
			if display_data[y * 64 + x] == 0 {
				line += ".";
			} else {
				line += "#";
			}
		}
		println!("|{}|", line);
	}
}