use crate::utils;

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

	pub fn read_memory_block(&self, start: usize, size: usize) -> &[u8] {
		if start >= 0x1000 || start + size > 0x1000 {
			eprintln!("Invalid read memory block start or size - start: {} - size: {}", start, size);
			return &[];
		}

		&self.memory[start..start + size]
	}

	pub fn write_memory_block(&mut self, addr: usize, data: &[u8]) {
		if addr >= 0x1000 || addr + data.len() > 0x1000 {
			eprintln!("Invalid write memory block start or overflowed - start: {} - size: {}", addr, data.len());
			return;
		}

		self.memory[addr..addr + data.len()].copy_from_slice(data);
	}

	pub fn read_memory_single(&self, addr: usize) -> &u8 {
		if addr >= 0x1000 {
			eprintln!("Invalid read address - address: {}", addr);
			return &0;
		}

		&self.memory[addr]
	}

	pub fn write_memory_single(&mut self, addr: usize, value: u8) {
		if addr >= 0x1000 {
			eprintln!("Invalid write address - address: {}", addr);
			return;
		}

		self.memory[addr] = value;
	}

	pub fn decrement_timers(&mut self) {
		if self.delay_timer > 0 {
			self.delay_timer -= 1;
		}

		if self.sound_timer > 0 {
			self.sound_timer -= 1;
		}
	}	

	pub fn fetch_decode_execute(&mut self) {
		// fetch
		let block: [u8; 2] = {
			let temp: &[u8] = self.read_memory_block(self.pc as usize, 2);
			[temp[0], temp[1]]
		};
		let mut instruction: u16 = (block[0] as u16) << 8;
		instruction += block[1] as u16;
		self.pc += 2;

		// decode
		let c:   u16   =  (instruction & 0xf000) >> 12; // high-level instruction category
		let vx:  usize = ((instruction & 0x0f00) >> 8) as usize; // VX register
		let vy:  usize = ((instruction & 0x00f0) >> 4) as usize; // VY register
		let n:   u8    =  (instruction & 0x000f) as u8; // 4 bit number
		let nn:  u8    =  (instruction & 0x00ff) as u8; // 8 bit number
		let nnn: u16   =   instruction & 0x0fff; // memory address

		//println!("Running: 0x{:04x}, c: {:x}", instruction, c);

		// execute
		match c {
			0x0 => {
				if instruction == 0x00e0 {
					// clear screen
					println!("Clear screen");
					self.display = [0; 64 * 32];
				// } else if instruction == 0x0000 {
				// 	panic!("0x0000 instruction read, exiting...");
				} else {
					Self::unhandled_instruction(&instruction);
				}
			},
			0x1 => {
				// jump
				self.pc = nnn;
				println!("Jump to 0x{:04x}", nnn);
			},
			0x2 => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0x3 => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0x4 => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0x5 => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0x6 => {
				// set register VX
				self.registers[vx] = nn;
				println!("Set V{:x} to 0x{:02x}", vx, nn);
			},
			0x7 => {
				// add value to register VX
				self.registers[vx] += nn;
				println!("Add to V{:x} amount 0x{:02x}", vx, nn);
			},
			0x8 => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0x9 => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0xa => {
				// set index register i
				self.i = nnn;
				println!("Set i reg to 0x{:04x}", nnn);
			},
			0xb => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0xc => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0xd => {
				// draw screen
				println!("Draw (0x{:04x}) - x: {}, y: {}, height: {}", instruction, self.registers[vx], self.registers[vy], n);

				let vx: usize = self.registers[vx] as usize;
				let vy: usize = self.registers[vy] as usize;
				let mut x: usize = vx % 64;
				let mut y: usize = vy % 32;
				self.registers[0xf] = 0;

				for row in 0..n {
					let byte: u8 = self.memory[(self.i as usize) + (row as usize)];
					for col in 0..8 {
						if (byte >> (7 - col)) & 1 == 1 {
							if self.display[y * 64 + x] == 1 {
								self.registers[0xf] = 1;
							}
							self.display[y * 64 + x] ^= 1;
						}
						x += 1;
						if x >= 64 {
							break;
						}
					}
					x = vx % 64;
					y += 1;
					if y >= 32 {
						break;
					}
				}

				utils::draw_display(&self.display);
			},
			0xe => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			0xf => {
				// todo
				Self::unhandled_instruction(&instruction);
			},
			default => {
				Self::unhandled_instruction(&instruction);
			}
		}
	}

	fn unhandled_instruction(instruction: &u16) {
		eprintln!("Error: Unhandled instruction: 0x{:04x}", instruction);
	}
}