use std::io::{self, Write};

pub trait RGB {
	fn fg(&self, buf: &mut String);
	fn bg(&self, buf: &mut String);
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
	fn default() -> Self {
		RGB4::Default
	}
}

impl RGB for RGB4 {
	fn fg(&self, buf: &mut String) {
		buf.push_str(match *self {
			RGB4::Default => "\x1b[39m",
			RGB4::Black => "\x1b[30m",
			RGB4::Red => "\x1b[31m",
			RGB4::Green => "\x1b[32m",
			RGB4::Yellow => "\x1b[33m",
			RGB4::Blue => "\x1b[34m",
			RGB4::Magenta => "\x1b[35m",
			RGB4::Cyan => "\x1b[36m",
			RGB4::LightGray => "\x1b[37m",
			RGB4::DarkGray => "\x1b[90m",
			RGB4::LightRed => "\x1b[91m",
			RGB4::LightGreen => "\x1b[92m",
			RGB4::LightYellow => "\x1b[93m",
			RGB4::LightBlue => "\x1b[94m",
			RGB4::LightMagenta => "\x1b[95m",
			RGB4::LightCyan => "\x1b[96m",
			RGB4::White => "\x1b[97m",
		});
	}
	fn bg(&self, buf: &mut String) {
		buf.push_str(match *self {
			RGB4::Default => "\x1b[49m",
			RGB4::Black => "\x1b[40m",
			RGB4::Red => "\x1b[41m",
			RGB4::Green => "\x1b[42m",
			RGB4::Yellow => "\x1b[43m",
			RGB4::Blue => "\x1b[44m",
			RGB4::Magenta => "\x1b[45m",
			RGB4::Cyan => "\x1b[46m",
			RGB4::LightGray => "\x1b[47m",
			RGB4::DarkGray => "\x1b[100m",
			RGB4::LightRed => "\x1b[101m",
			RGB4::LightGreen => "\x1b[102m",
			RGB4::LightYellow => "\x1b[103m",
			RGB4::LightBlue => "\x1b[104m",
			RGB4::LightMagenta => "\x1b[105m",
			RGB4::LightCyan => "\x1b[106m",
			RGB4::White => "\x1b[107m",
		});
	}
}

impl RGB for () {
	fn fg(&self, _buf: &mut String) { }
	fn bg(&self, _buf: &mut String) { }
}

pub struct RGB8;
impl RGB8 {
	pub fn rgb4(c: RGB4) -> u8 {
		match c {
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
		}
	}

	pub fn rgb(r: u8, g: u8, b: u8) -> u8 {
		16 + r*36 + g*6 + b
	}

	pub fn gray(s: u8) -> u8 {
		232 + s
	}
}

impl RGB for u8 {
	fn fg(&self, buf: &mut String) {
		buf.push_str(&format!("\x1b[38;5;{}m", *self))
	}
	fn bg(&self, buf: &mut String) {
		buf.push_str(&format!("\x1b[48;5;{}m", *self))
	}
}

impl RGB for (u8, u8, u8) {
	fn fg(&self, buf: &mut String) {
		buf.push_str(&format!("\x1b[{};2;{};{};{}m", 38, self.0, self.1, self.2));
	}
	fn bg(&self, buf: &mut String) {
		buf.push_str(&format!("\x1b[{};2;{};{};{}m", 48, self.0, self.1, self.2));
	}
}

#[derive(Default)]
pub struct Cursor(pub String);

impl Cursor {
	pub fn esc(&mut self, s: &str){
		self.0.push('\x1b');
		self.0.push('[');
		self.0.push_str(s)
	}
	pub fn escch(&mut self, c: char){
		self.0.push('\x1b');
		self.0.push('[');
		self.0.push(c)
	}
	pub fn clearattr(&mut self){
		self.escch('m')
	}
	pub fn setbold(&mut self){
		self.esc("1m")
	}
	pub fn setdim(&mut self){
		self.esc("2m")
	}
	pub fn setunder(&mut self){
		self.esc("4m")
	}
	pub fn setblink(&mut self){
		self.esc("5m")
	}
	pub fn setrev(&mut self){
		self.esc("7m")
	}
	pub fn unsetbold(&mut self){
		self.esc("21m")
	}
	pub fn unsetrev(&mut self){
		self.esc("27m")
	}
	pub fn wrapon(&mut self){
		self.0.push_str("\x1b7h")
	}
	pub fn wrapoff(&mut self){
		self.0.push_str("\x1b7l")
	}
	pub fn up1(&mut self){
		self.escch('A')
	}
	pub fn down1(&mut self){
		self.escch('B')
	}
	pub fn right1(&mut self){
		self.escch('C')
	}
	pub fn left1(&mut self){
		self.escch('D')
	}
	pub fn up(&mut self, n: u16){
		self.esc(&format!("{}A", n))
	}
	pub fn down(&mut self, n: u16){
		self.esc(&format!("{}B", n))
	}
	pub fn right(&mut self, n: u16){
		self.esc(&format!("{}C", n))
	}
	pub fn left(&mut self, n: u16){
		self.esc(&format!("{}D", n))
	}
	pub fn x1down(&mut self, n: u16){
		self.esc(&format!("{}E", n))
	}
	pub fn x1up(&mut self, n: u16){
		self.esc(&format!("{}F", n))
	}
	pub fn setx(&mut self, x: u16){
		self.esc(&format!("{}G", x))
	}
	pub fn sety(&mut self, y: u16){
		self.esc(&format!("{}d", y))
	}
	pub fn resetxy(&mut self){
		self.escch('H')
	}
	pub fn mv(&mut self, x: u16, y: u16){
		self.esc(&format!("{};{}H",y,x));
	}
	pub fn erasebelow(&mut self){
		self.escch('J')
	}
	pub fn eraseabove(&mut self){
		self.esc("1J")
	}
	pub fn eraseall(&mut self){
		self.esc("2J")
	}
	pub fn eraseleft(&mut self){
		self.escch('K')
	}
	pub fn eraseright(&mut self){
		self.esc("1K")
	}
	pub fn eraseline(&mut self){
		self.esc("2K")
	}
	pub fn delln(&mut self){
		self.escch('M')
	}
	pub fn dellns(&mut self, n: u16){
		self.esc(&format!("{}M", n))
	}
	pub fn delch(&mut self){
		self.escch('S')
	}
	pub fn delchs(&mut self, n: u16){
		self.esc(&format!("{}S", n))
	}
	pub fn showcur(&mut self){
		self.esc("?23h")
	}
	pub fn hidecur(&mut self){
		self.esc("?25l")
	}
	pub fn spame(&mut self){
		self.0.push_str("\x1b#8")
	}
	pub fn setfg<TColor: RGB>(&mut self, rgb: TColor) {
		rgb.fg(&mut self.0);
	}
	pub fn setbg<TColor: RGB>(&mut self, rgb: TColor) {
		rgb.bg(&mut self.0);
	}
	pub fn prchr(&mut self, c: char){
		self.0.push(c)
	}
	pub fn print(&mut self, s: &str){
		self.0.push_str(s)
	}
	pub fn clear(&mut self) {
		self.0.clear();
		self.0.push_str("\x1bc");
	}
	pub fn dropclear() -> io::Result<()> {
		io::stdout().write_all(b"\x1bc")
	}
	pub fn flush(&mut self) -> io::Result<()> {
		let mut out = io::stdout();
		try!(out.write_all(self.0.as_bytes()));
		try!(out.flush());
		Ok(self.0.clear())
	}
}
