mod chip8;

fn main() {
	let mut chip8 = chip8::Chip8::new();

	let program: Vec<u8> = vec! [
		0x00, 0xE0 // Clear the screen
	];

	chip8.reset();
	chip8.load_program(&program);
	
	for i in 0..16 {
		chip8.write_memory_single(i, (i * i) as u8);
	}

	println!("{:?}", chip8.read_memory_block(0, 16).unwrap());
}