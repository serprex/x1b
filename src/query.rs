use std::io;
use std::io::{Read,Write};
use std::fs::{File,OpenOptions};

pub fn query_start(esc: &[u8]) -> io::Result<File> {
	let mut tty = try!(OpenOptions::new().read(true).write(true).open("/dev/tty"));
	try!(tty.write_all(esc));
	try!(tty.flush());
	Ok(tty)
}

enum Curp {
	Nil,
	Esc,
	Esc2,
	Y(u16, u16),
	Semi(u16),
	X(u16, u16, u16),
}

fn ascii2digit(x: u8) -> u16 {
	(x-b'0') as u16
}

fn get_cursor_xy_with_esc(esc: &[u8]) -> io::Result<(u16, u16)> {
	let mut state = Curp::Nil;
	for b in try!(query_start(esc)).bytes() {
		let b = try!(b);
		state = match (state, b) {
			(Curp::Nil, b'\x1b') => Curp::Esc,
			(Curp::Nil, _) => Curp::Nil,
			(Curp::Esc, b'[') => Curp::Esc2,
			(Curp::Esc, _) => Curp::Nil,
			(Curp::Esc2, b'0'...b'9') => Curp::Y(10, ascii2digit(b)),
			(Curp::Esc2, _) => Curp::Nil,
			(Curp::Y(_, y), b';') => Curp::Semi(y),
			(Curp::Y(z, y), b'0'...b'9') => Curp::Y(z*10, y+ascii2digit(b)*z),
			(Curp::Y(_, _), _) => Curp::Nil,
			(Curp::Semi(y), b'0'...b'9') => Curp::X(10, ascii2digit(b), y),
			(Curp::Semi(_), _) => Curp::Nil,
			(Curp::X(_, x, y), b'R') => return Ok((x, y)),
			(Curp::X(z, x, y), b'0'...b'9') => Curp::X(z*10, x+ascii2digit(b)*z, y),
			(Curp::X(_, _, _), _) => Curp::Nil,
		}
	}
	Err(io::Error::new(io::ErrorKind::NotFound, "End of tty before cursor pos matched"))
}

pub fn get_cursor_xy() -> io::Result<(u16, u16)> {
	get_cursor_xy_with_esc(b"\x1b[6n")
}


pub fn get_tty_wh_dirty() -> io::Result<(u16, u16)> {
	get_cursor_xy_with_esc(b"\x1b[999;999H\x1b[6n")
}

pub fn get_tty_wh() -> io::Result<(u16, u16)> {
	get_cursor_xy_with_esc(b"\x1b7\x1b[999;999H\x1b[6n\x1b8")
}
