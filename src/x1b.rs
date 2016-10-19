use std::convert::From;
use std::io::{self, Write};

pub trait RGB {
	fn fg(&self, buf: &mut Vec<u8>);
	fn bg(&self, buf: &mut Vec<u8>);
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum RGB4 {
	Default,
	Black,
	Red,
	Green,
	Yellow,
	Blue,
	Magenta,
	Cyan,
	LightGray,
	DarkGray,
	LightRed,
	LightGreen,
	LightYellow,
	LightBlue,
	LightMagenta,
	LightCyan,
	White,
}

impl Default for RGB4 {
	#[inline(always)]
	fn default() -> Self {
		RGB4::Default
	}
}

impl RGB for RGB4 {
	fn fg(&self, buf: &mut Vec<u8>) {
		buf.extend_from_slice(match *self {
			RGB4::Default => b"\x1b[39m",
			RGB4::Black => b"\x1b[30m",
			RGB4::Red => b"\x1b[31m",
			RGB4::Green => b"\x1b[32m",
			RGB4::Yellow => b"\x1b[33m",
			RGB4::Blue => b"\x1b[34m",
			RGB4::Magenta => b"\x1b[35m",
			RGB4::Cyan => b"\x1b[36m",
			RGB4::LightGray => b"\x1b[37m",
			RGB4::DarkGray => b"\x1b[90m",
			RGB4::LightRed => b"\x1b[91m",
			RGB4::LightGreen => b"\x1b[92m",
			RGB4::LightYellow => b"\x1b[93m",
			RGB4::LightBlue => b"\x1b[94m",
			RGB4::LightMagenta => b"\x1b[95m",
			RGB4::LightCyan => b"\x1b[96m",
			RGB4::White => b"\x1b[97m",
		});
	}
	fn bg(&self, buf: &mut Vec<u8>) {
		buf.extend_from_slice(match *self {
			RGB4::Default => b"\x1b[49m",
			RGB4::Black => b"\x1b[40m",
			RGB4::Red => b"\x1b[41m",
			RGB4::Green => b"\x1b[42m",
			RGB4::Yellow => b"\x1b[43m",
			RGB4::Blue => b"\x1b[44m",
			RGB4::Magenta => b"\x1b[45m",
			RGB4::Cyan => b"\x1b[46m",
			RGB4::LightGray => b"\x1b[47m",
			RGB4::DarkGray => b"\x1b[100m",
			RGB4::LightRed => b"\x1b[101m",
			RGB4::LightGreen => b"\x1b[102m",
			RGB4::LightYellow => b"\x1b[103m",
			RGB4::LightBlue => b"\x1b[104m",
			RGB4::LightMagenta => b"\x1b[105m",
			RGB4::LightCyan => b"\x1b[106m",
			RGB4::White => b"\x1b[107m",
		});
	}
}

impl RGB for () {
	#[inline(always)]
	fn fg(&self, _buf: &mut Vec<u8>) { }
	#[inline(always)]
	fn bg(&self, _buf: &mut Vec<u8>) { }
}

pub struct RGB8(pub u8);
impl RGB8 {
	#[inline(always)]
	pub fn rgb(r: u8, g: u8, b: u8) -> RGB8 {
		RGB8(16 + r*36 + g*6 + b)
	}
	#[inline(always)]
	pub fn gray(s: u8) -> RGB8 {
		RGB8(232 + s)
	}
}

impl From<RGB4> for RGB8 {
	fn from(c: RGB4) -> Self {
		RGB8(match c {
			RGB4::Default => 0,
			RGB4::Black => 0,
			RGB4::Red => 1,
			RGB4::Green => 2,
			RGB4::Yellow => 3,
			RGB4::Blue => 4,
			RGB4::Magenta => 5,
			RGB4::Cyan => 6,
			RGB4::LightGray => 7,
			RGB4::DarkGray => 8,
			RGB4::LightRed => 9,
			RGB4::LightGreen => 10,
			RGB4::LightYellow => 11,
			RGB4::LightBlue => 12,
			RGB4::LightMagenta => 13,
			RGB4::LightCyan => 14,
			RGB4::White => 15,
		})
	}
}

impl RGB for RGB8 {
	#[inline(always)]
	fn fg(&self, buf: &mut Vec<u8>) {
		buf.reserve(11);
		buf.extend_from_slice(b"\x1b[38;5;");
		unsafe { extend_from_u8(buf, self.0); }
		buf.push(b'm');
	}
	#[inline(always)]
	fn bg(&self, buf: &mut Vec<u8>) {
		buf.reserve(11);
		buf.extend_from_slice(b"\x1b[48;5;");
		unsafe { extend_from_u8(buf, self.0); }
		buf.push(b'm');
	}
}

impl RGB for (u8, u8, u8) {
	fn fg(&self, buf: &mut Vec<u8>) {
		buf.reserve(17);
		buf.extend_from_slice(b"\x1b[38;2;");
		unsafe {
			extend_from_u8(buf, self.0);
			buf.push(b';');
			extend_from_u8(buf, self.1);
			buf.push(b';');
			extend_from_u8(buf, self.2);
			buf.push(b'm');
		}
	}
	fn bg(&self, buf: &mut Vec<u8>) {
		buf.reserve(17);
		buf.extend_from_slice(b"\x1b[48;2;");
		unsafe {
			extend_from_u8(buf, self.0);
			buf.push(b';');
			extend_from_u8(buf, self.1);
			buf.push(b';');
			extend_from_u8(buf, self.2);
			buf.push(b'm');
		}
	}
}

unsafe fn extend_from_u8(v: &mut Vec<u8>, x: u8) {
	let mut vlen = v.len();
	let buf = v.as_mut_ptr().offset(vlen as isize);
	if x < 10 {
		*buf.offset(0) = b'0' + x;
		vlen += 1;
	} else if x < 100 {
		*buf.offset(0) = b'0' + (x/10);
		*buf.offset(1) = b'0' + x%10;
		vlen += 2;
	} else {
		*buf.offset(0) = if x<200 { b'1' } else { b'2' };
		*buf.offset(1) = b'0' + (x/10)%10;
		*buf.offset(2) = b'0' + x%10;
		vlen += 3;
	}
	v.set_len(vlen);
}

unsafe fn extend_from_u16(v: &mut Vec<u8>, x: u16) {
	let mut vlen = v.len();
	let buf = v.as_mut_ptr().offset(vlen as isize);
	if x < 10 {
		*buf.offset(0) = b'0' + x as u8;
		vlen += 1;
	} else if x < 100 {
		*buf.offset(0) = b'0' + (x/10) as u8;
		*buf.offset(1) = b'0' + (x%10) as u8;
		vlen += 2;
	} else if x < 1000 {
		*buf.offset(0) = b'0' + (x/100) as u8;
		*buf.offset(1) = b'0' + ((x/10)%10) as u8;
		*buf.offset(2) = b'0' + (x%10) as u8;
		vlen += 3;
	} else if x < 10000 {
		*buf.offset(0) = b'0' + (x/1000) as u8;
		*buf.offset(1) = b'0' + ((x/100)%10) as u8;
		*buf.offset(2) = b'0' + ((x/10)%10) as u8;
		*buf.offset(3) = b'0' + (x%10) as u8;
		vlen += 4;
	} else {
		*buf.offset(0) = b'1';
		*buf.offset(1) = b'0' + ((x-10000)/1000) as u8;
		*buf.offset(2) = b'0' + ((x/100)%10) as u8;
		*buf.offset(3) = b'0' + ((x/10)%10) as u8;
		*buf.offset(4) = b'0' + (x%10) as u8;
		vlen += 4;
	}
	v.set_len(vlen);
}

#[derive(Default)]
pub struct Cursor(pub Vec<u8>);

impl Cursor {
	#[inline(always)]
	pub fn ext(&mut self, s: &[u8]){
		self.0.extend_from_slice(s)
	}
	#[inline(always)]
	pub fn clearattr(&mut self){
		self.ext(b"\x1b[m")
	}
	#[inline(always)]
	pub fn setbold(&mut self){
		self.ext(b"\x1b[1m")
	}
	#[inline(always)]
	pub fn setdim(&mut self){
		self.ext(b"\x1b[2m")
	}
	#[inline(always)]
	pub fn setunder(&mut self){
		self.ext(b"\x1b[4m")
	}
	#[inline(always)]
	pub fn setblink(&mut self){
		self.ext(b"\x1b[5m")
	}
	#[inline(always)]
	pub fn setrev(&mut self){
		self.ext(b"\x1b[7m")
	}
	#[inline(always)]
	pub fn unsetbold(&mut self){
		self.ext(b"\x1b[21m")
	}
	#[inline(always)]
	pub fn unsetrev(&mut self){
		self.ext(b"\x1b[27m")
	}
	#[inline(always)]
	pub fn wrapon(&mut self){
		self.ext(b"\x1b7h")
	}
	#[inline(always)]
	pub fn wrapoff(&mut self){
		self.ext(b"\x1b7l")
	}
	#[inline(always)]
	pub fn up1(&mut self){
		self.ext(b"\x1b[A")
	}
	#[inline(always)]
	pub fn down1(&mut self){
		self.ext(b"\x1b[B")
	}
	#[inline(always)]
	pub fn right1(&mut self){
		self.ext(b"\x1b[C")
	}
	#[inline(always)]
	pub fn left1(&mut self){
		self.ext(b"\x1b[D")
	}
	#[inline(always)]
	fn u16ch(&mut self, ch: u8, n: u16){
		self.0.reserve(8);
		self.ext(b"\x1b[");
		unsafe { extend_from_u16(&mut self.0, n); }
		self.0.push(ch);
	}
	#[inline(always)]
	pub fn up(&mut self, n: u16){
		self.u16ch(b'A', n)
	}
	#[inline(always)]
	pub fn down(&mut self, n: u16){
		self.u16ch(b'B', n)
	}
	#[inline(always)]
	pub fn right(&mut self, n: u16){
		self.u16ch(b'C', n)
	}
	#[inline(always)]
	pub fn left(&mut self, n: u16){
		self.u16ch(b'D', n)
	}
	#[inline(always)]
	pub fn x1down(&mut self, n: u16){
		self.u16ch(b'E', n)
	}
	#[inline(always)]
	pub fn x1up(&mut self, n: u16){
		self.u16ch(b'F', n)
	}
	#[inline(always)]
	pub fn setx(&mut self, x: u16){
		self.u16ch(b'G', x)
	}
	#[inline(always)]
	pub fn sety(&mut self, y: u16){
		self.u16ch(b'd', y)
	}
	#[inline(always)]
	pub fn resetxy(&mut self){
		self.ext(b"\x1b[H")
	}
	#[inline(always)]
	pub fn mv(&mut self, x: u16, y: u16){
		self.0.reserve(14);
		self.ext(b"\x1b[");
		unsafe {
			extend_from_u16(&mut self.0, y);
			self.0.push(b';');
			extend_from_u16(&mut self.0, x);
			self.0.push(b'H');
		}
	}
	#[inline(always)]
	pub fn erasebelow(&mut self){
		self.ext(b"\x1b[J")
	}
	#[inline(always)]
	pub fn eraseabove(&mut self){
		self.ext(b"\x1b[1J")
	}
	#[inline(always)]
	pub fn eraseall(&mut self){
		self.ext(b"\x1b[2J")
	}
	#[inline(always)]
	pub fn eraseleft(&mut self){
		self.ext(b"\x1b[K")
	}
	#[inline(always)]
	pub fn eraseright(&mut self){
		self.ext(b"\x1b[1K")
	}
	#[inline(always)]
	pub fn eraseline(&mut self){
		self.ext(b"\x1b[2K")
	}
	#[inline(always)]
	pub fn delln(&mut self){
		self.ext(b"\x1b[M")
	}
	#[inline(always)]
	pub fn dellns(&mut self, n: u16){
		self.u16ch(b'M', n)
	}
	#[inline(always)]
	pub fn delch(&mut self){
		self.ext(b"\x1b[S")
	}
	#[inline(always)]
	pub fn delchs(&mut self, n: u16){
		self.u16ch(b'S', n)
	}
	#[inline(always)]
	pub fn showcur(&mut self){
		self.ext(b"\x1b[?23h")
	}
	#[inline(always)]
	pub fn hidecur(&mut self){
		self.ext(b"\x1b[?25l")
	}
	#[inline(always)]
	pub fn spame(&mut self){
		self.ext(b"\x1b#8")
	}
	#[inline(always)]
	pub fn setfg<TColor: RGB>(&mut self, rgb: TColor) {
		rgb.fg(&mut self.0);
	}
	#[inline(always)]
	pub fn setbg<TColor: RGB>(&mut self, rgb: TColor) {
		rgb.bg(&mut self.0);
	}
	#[inline(always)]
	pub fn prchr(&mut self, c: char){
		let len = c.len_utf8();
		if len == 1 {
			self.0.push(c as u32 as u8);
		} else {
			// Waiting on char::encode_utf8
			let cs = c.to_string();
			self.ext(cs.as_bytes());
		}
	}
	#[inline(always)]
	pub fn print(&mut self, s: &str){
		self.ext(s.as_bytes())
	}
	#[inline(always)]
	pub fn clear(&mut self) {
		self.0.clear();
		self.ext(b"\x1bc");
	}
	#[inline(always)]
	pub fn dropclear() -> io::Result<()> {
		io::stdout().write_all(b"\x1bc")
	}
	#[inline(always)]
	pub fn flush(&mut self) -> io::Result<()> {
		let mut out = io::stdout();
		try!(out.write_all(&self.0));
		try!(out.flush());
		Ok(self.0.clear())
	}
}
