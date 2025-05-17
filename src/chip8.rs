use rand::Rng;

use crate::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

static SUPER_CHIP: bool = false;

pub struct Chip8 {
	memory: [u8; 4096], // 4KB memory
	registers: [u8; 16], // 16 registers (V0 - Vf)
	pc: u16, // Program counter
	i: u16, // Index register
	delay_timer: u8,
	sound_timer: u8,
	stack: [u16; 16], // 16 level stack
	sp: u8, // Stack pointer
	display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT], // 64 x 32 pixels, monochrome, 1 byte/pixel
	keypad_held: [bool; 16],
	keypad_released: [bool; 16],
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
			display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
			keypad_held: [false; 16],
			keypad_released: [false; 16],
		}
	}

	pub fn read_memory(&self, start: usize, size: usize) -> &[u8] {
		&self.memory[start..start + size]
	}

	pub fn write_memory(&mut self, addr: usize, data: &[u8]) {
		self.memory[addr..addr + data.len()].copy_from_slice(data);
	}

	pub fn decrement_timers(&mut self) {
		if self.delay_timer > 0 {
			self.delay_timer -= 1;
		}

		if self.sound_timer > 0 {
			self.sound_timer -= 1;
		}
	}

	pub fn get_display(&self) -> &[u8; DISPLAY_WIDTH * DISPLAY_HEIGHT] {
		&self.display
	}

	pub fn set_keypad_held(&mut self, key: usize, value: bool) {
		self.keypad_held[key] = value;
	}

	pub fn set_keypad_released(&mut self, key: usize, value: bool) {
		self.keypad_released[key] = value;
	}

	pub fn fetch_decode_execute(&mut self) {
		// fetch
		let block: [u8; 2] = {
			let temp: &[u8] = self.read_memory(self.pc as usize, 2);
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

		// execute
		match c {
			0x0 => {
				match instruction {
					0x00e0 => self.display = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT], // clear screen
					0x00ee => {
						self.sp -= 1;
						self.pc = self.stack[self.sp as usize];
					} // return from subroutine
					_ => Self::unhandled_instruction(&instruction),
				}
			},
			0x1 => self.pc = nnn, // jump
			0x2 => { // subroutine jump
				self.stack[self.sp as usize] = self.pc;
				self.sp += 1;
				self.pc = nnn;
			}
			0x3 => if self.registers[vx] == nn { self.pc += 2 }, // skip conditionally,
			0x4 => if self.registers[vx] != nn { self.pc += 2 }, // skip conditionally,
			0x5 => if self.registers[vx] == self.registers[vy] { self.pc += 2 }, // skip conditionally,
			0x6 => self.registers[vx] = nn, // set register VX
			0x7 => self.registers[vx] += nn, // add value to register VX
			0x8 => {
				match n {
					0x0 => self.registers[vx]  = self.registers[vy], // set VX to  VY
					0x1 => self.registers[vx] |= self.registers[vy], //  OR VX and VY
					0x2 => self.registers[vx] &= self.registers[vy], // AND VX and VY
					0x3 => self.registers[vx] ^= self.registers[vy], // XOR VX and VY
					0x4 => { // add VX and VY
						let (sum, carry) = self.registers[vx].overflowing_add(self.registers[vy]);
						self.registers[vx] = sum;
						self.registers[0xf] = carry as u8;
					},
					0x5 => { // set VX to VX - VY
						let carry: bool = self.registers[vx] >= self.registers[vy];
						self.registers[vx] -= self.registers[vy];
						self.registers[0xf] = carry as u8;
					},
					0x6 => { // shift
						let reg: usize = if SUPER_CHIP { vx } else { vy };
						let carry: u8 = self.registers[reg] & 0x1;
						self.registers[vx] = self.registers[reg] >> 1;
						self.registers[0xf] = carry;
					},
					0x7 => { // set VX to VY - VX
						let carry: bool = self.registers[vy] >= self.registers[vx];
						self.registers[vx]  = self.registers[vy] - self.registers[vx];
						self.registers[0xf] = carry as u8;
					},
					0xe => { // shift
						let reg: usize = if SUPER_CHIP { vx } else { vy };
						let carry: u8 = (self.registers[reg] >> 7) & 0x1;
						self.registers[vx] = self.registers[reg] << 1;
						self.registers[0xf] = carry;
					},
					_ => Self::unhandled_instruction(&instruction),
				}
			},
			0x9 => if self.registers[vx] != self.registers[vy] { self.pc += 2 }, // skip conditionally,
			0xa => self.i = nnn, // set index register i
			0xb => { // jump with offset
				if SUPER_CHIP {
					self.pc = nnn + self.registers[vx] as u16;
				} else {
					self.pc = nnn + self.registers[0x0] as u16;
				}
			},
			0xc => self.registers[vx] = nn & rand::rng().random::<u8>(), // random number
			0xd => {
				// draw screen
				let vx_val: usize = self.registers[vx] as usize;
				let vy_val: usize = self.registers[vy] as usize;
				let mut x: usize = vx_val % DISPLAY_WIDTH;
				let mut y: usize = vy_val % DISPLAY_HEIGHT;
				self.registers[0xf] = 0;

				for row in 0..n {
					let byte: u8 = self.memory[(self.i as usize) + (row as usize)];
					for col in 0..8 {
						if (byte >> (7 - col)) & 1 == 1 {
							if self.display[y * DISPLAY_WIDTH + x] == 1 {
								self.registers[0xf] = 1;
							}
							self.display[y * DISPLAY_WIDTH + x] ^= 1;
						}
						x += 1;
						if x >= DISPLAY_WIDTH {
							break;
						}
					}
					x = vx_val % DISPLAY_WIDTH;
					y += 1;
					if y >= DISPLAY_HEIGHT {
						break;
					}
				}
			},
			0xe => {
				match nn {
					0x9e => { // skip if key[reg[vx]] held
						if self.keypad_held[self.registers[vx] as usize] {
							self.pc += 2;
						}
					},
					0xa1 => { // skip if key[reg[vx]] not held
						if !self.keypad_held[self.registers[vx] as usize] {
							self.pc += 2;
						}
					},
					_ => Self::unhandled_instruction(&instruction),
				}
			},
			0xf => {
				match nn {
					0x07 => self.registers[vx] = self.delay_timer, // set VX to delay timer
					0x0a => { // block until key pressed and released
						for i in 0..16 {
							if self.keypad_released[i] {
								self.registers[vx] = i as u8;
								return;
							}
						}
						self.pc -= 2;
					},
					0x15 => self.delay_timer = self.registers[vx], // set delay timer to VX
					0x18 => self.sound_timer = self.registers[vx], // set sound timer to VX
					0x1e => { // add VX to I reg
						if self.i <= 0xfff && (self.i + self.registers[vx] as u16) >= 0x1000 {
							self.registers[0xf] = 1;
						}
						self.i += self.registers[vx] as u16;
					},
					0x29 => self.i = (self.registers[vx] as u16) * 5, // point to font
					0x33 => { // binary coded decimal conversion
						self.memory[self.i as usize + 0] = self.registers[vx] / 100;
						self.memory[self.i as usize + 1] = (self.registers[vx] % 100) / 10;
						self.memory[self.i as usize + 2] = self.registers[vx] % 10;
					},
					0x55 => { // store memory
						if SUPER_CHIP {
							for r in 0..=vx {
								self.memory[self.i as usize + r] = self.registers[r];
							}
						} else {
							for r in 0..=vx {
								self.memory[self.i as usize] = self.registers[r];
								self.i += 1;
							}
						}
						
					},
					0x65 => { // load memory
						if SUPER_CHIP {
							for r in 0..=vx {
								self.registers[r] = self.memory[self.i as usize + r];
							}
						} else {
							for r in 0..=vx {
								self.registers[r] = self.memory[self.i as usize];
								self.i += 1;
							}
						}
					},
					_ => Self::unhandled_instruction(&instruction),
				}
			},
			_ => Self::unhandled_instruction(&instruction),
		}
	}

	fn unhandled_instruction(instruction: &u16) {
		eprintln!("Error: Unhandled instruction: 0x{:04x}", instruction);
	}
}