pub struct Chip8 {
	memory: [u8; 4096], // 4KB memory
	registers: [u8; 16], // 16 registers (V0 - Vf)
	pc: u16, // Program counter
	i: u16, // Index register
	delay_timer: u8,
	sound_timer: u8,
	stack: [u16; 16], // 16 level stack
	sp: u8, // Stack pointer
	display: [u8; 64 * 32], // 64 x 32 pixels, monochrome, 1 byte/pixel
	keypad: [bool; 16],
}

impl Chip8 {
	pub fn new() -> Self {
		Chip8 {
			memory: [0; 4096],
			registers: [0; 16],
			pc: 0x200,
			i: 0,
			delay_timer: 0,
			sound_timer: 0,
			stack: [0; 16],
			sp: 0,
			display: [0; 64 * 32],
			keypad: [false; 16],
		}
	}

	pub fn load_program(&mut self, program: &[u8]) {
		for (i, &byte) in program.iter().enumerate() {
			self.memory[0x200 + i] = byte;
		}
	}

	pub fn reset(&mut self) {
		self.memory = [0; 4096];
		self.registers = [0; 16];
		self.pc = 0x200;
		self.i = 0;
		self.delay_timer = 0;
		self.sound_timer = 0;
		self.stack = [0; 16];
		self.sp = 0;
		self.display = [0; 64 * 32];
		self.keypad = [false; 16];
	}

	pub fn read_memory_full(&self) -> &[u8; 4096] {
		&self.memory
	}

	pub fn write_memory_full(&mut self, memory: &[u8; 4096]) {
		self.memory.copy_from_slice(memory);
	}

	pub fn read_memory_block(&self, start: usize, size: usize) -> Option<&[u8]> {
		if start >= 0x1000 || start + size > 0x1000 {
			eprintln!("Invalid read memory block start or size - start: {} - size: {}", start, size);
			return None;
		}

		Some(&self.memory[start..start + size])
	}

	pub fn write_memory_block(&mut self, start: usize, size: usize, memory: Option<&[u8]>) {
		if start >= 0x1000 || size > 0x1000 || start + size > 0x1000 {
			eprintln!("Invalid write memory block start or size - start: {} - size: {}", start, size);
			return;
		}

		let data = match memory {
			Some(d) => d,
			None => {
				eprintln!("No memory data provided");
				return;
			}
		};

		if data.len() != size {
			eprintln!("Data size does not match the supplied size - data size: {} - size: {}", data.len(), size);
			return;
		}

		self.memory[start..start + size].copy_from_slice(data);
	}

	pub fn read_memory_single(&self, addr: usize) -> Option<&u8> {
		if addr >= 0x1000 {
			eprintln!("Invalid read address - address: {}", addr);
			return None;
		}

		Some(&self.memory[addr])
	}

	pub fn write_memory_single(&mut self, addr: usize, value: u8) {
		if addr >= 0x1000 {
			eprintln!("Invalid write address - address: {}", addr);
			return;
		}

		self.memory[addr] = value;
	}
}