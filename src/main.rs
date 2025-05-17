use minifb::{Key, Scale, Window, WindowOptions};
use std::fs::File;
use std::io::{self, Read};
use std::time::{Duration, Instant};

mod chip8;

const DISPLAY_FPS: u64 = 60;
const DISPLAY_DELAY: Duration = Duration::from_micros(1_000_000 / DISPLAY_FPS);
const TIMER_FPS: u64 = 60;
const TIMER_DELAY: Duration = Duration::from_micros(1_000_000 / TIMER_FPS);

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

const FONT_DATA: [u8; 5 * 16] = [
	0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
	0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
	0x90, 0x90, 0xf0, 0x10, 0x10, // 4
	0xf0, 0x80, 0xf0, 0x10, 0xf0, // 5
	0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
	0xf0, 0x10, 0x20, 0x40, 0x40, // 7
	0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
	0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
	0xf0, 0x90, 0xf0, 0x90, 0x90, // A
	0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
	0xf0, 0x80, 0x80, 0x80, 0xf0, // C
	0xe0, 0x90, 0x90, 0x90, 0xe0, // D
	0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
	0xf0, 0x80, 0xf0, 0x80, 0x80  // F
];

const PROGRAM_NAME: &str = "tests/flags";

fn main() {
	let mut chip8: chip8::Chip8 = chip8::Chip8::new();
	let mut frame_buffer: [u32; DISPLAY_WIDTH * DISPLAY_HEIGHT] = [0; DISPLAY_WIDTH * DISPLAY_HEIGHT];

	let mut window: Window = Window::new(
		"CHIP-8 Emulator - FPS: 0",
		DISPLAY_WIDTH,
		DISPLAY_HEIGHT,
		WindowOptions {
			resize: false,
			scale: Scale::X16,
			..WindowOptions::default()
		},
	).unwrap_or_else(|e| {
		panic!("Unable to open window: {}", e);
	});

	chip8.reset();
	chip8.write_memory(0x050, &FONT_DATA);

	let file_path = std::env::current_dir()
		.map(|current_dir| current_dir.join(format!("roms/{}.ch8", PROGRAM_NAME)))
		.unwrap_or_else(|_| panic!("Error getting current directory"));

	if file_path.exists() {
		match read_file(file_path.to_str().unwrap()) {
			Ok(bytes) => chip8.write_memory(0x200, &bytes),
			Err(_) => panic!("Unable to read ROM: {}", file_path.display()),
		}
	} else {
		panic!("Unable to find ROM: {}", file_path.display());
	}

	let mut frame_count: u16 = 0;
	let mut last_fps_time: Instant = Instant::now();

	// main loop
	let mut last_frame_time: Instant = Instant::now();
	while window.is_open() && !window.is_key_down(Key::Escape) {
		let now: Instant = Instant::now();
		let elapsed: Duration = now.duration_since(last_frame_time);
		if elapsed < DISPLAY_DELAY {
			std::thread::sleep(DISPLAY_DELAY - elapsed);
		}

		// emulate
		chip8.decrement_timers();
		chip8.fetch_decode_execute();

		// render
		let chip8_display: &[u8; DISPLAY_WIDTH * DISPLAY_HEIGHT] = chip8.get_display();
		for y in 0..DISPLAY_HEIGHT {
			for x in 0..DISPLAY_WIDTH {
				frame_buffer[y * DISPLAY_WIDTH + x] = (chip8_display[y * DISPLAY_WIDTH + x] as u32) * 0xffffff;
			}
		}
		let _ = window.update_with_buffer(&frame_buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT);

		// fps counter
		frame_count += 1;
		if last_fps_time.elapsed() >= Duration::from_secs(1) {
			window.set_title(&format!("CHIP-8 Emulator - FPS: {}", frame_count));
			frame_count = 0;
			last_fps_time = Instant::now();
		}

		last_frame_time += DISPLAY_DELAY;
	}
}

fn read_file(file_path: &str) -> io::Result<Vec<u8>> {
	let mut buffer: Vec<u8> = Vec::new();
	File::open(file_path)?.read_to_end(&mut buffer)?;
	Ok(buffer)
}