use std::io::{self, Write};
bitflags! {
	pub flags TextAttr: u8 {
		const TA_BOLD = 1,
		const TA_DIM = 2,
		const TA_UNDER = 4,
		const TA_BLINK = 8,
		const TA_REV = 16,
	}
}

const TA_CHARS: [(TextAttr, char); 5] = [
	(TA_BOLD, '1'),
	(TA_DIM, '2'),
	(TA_UNDER, '4'),
	(TA_BLINK, '5'),
	(TA_REV, '7')];

impl TextAttr {
	pub fn clear(&mut self) -> bool {
		let ret = self.bits != 0;
		self.bits = 0;
		ret
	}
}

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
			RGB4::Default => "\x1b[49m",
			RGB4::Black => "\x1b[40m",
			RGB4::Red => "\x1b[41m",
			RGB4::Green => "\x1b[42m",
			RGB4::Yellow => "\x1b[43m",
			RGB4::Blue => "\x1b[44m",
			RGB4::Magenta => "\x1b[45m",
			RGB4::Cyan => "\x1b[46m",
			RGB4::LightGray => "\x1b[47m",
			RGB4::DarkGray => "\x1b[40;1m",
			RGB4::LightRed => "\x1b[41;1m",
			RGB4::LightGreen => "\x1b[42;1m",
			RGB4::LightYellow => "\x1b[43;1m",
			RGB4::LightBlue => "\x1b[44;1m",
			RGB4::LightMagenta => "\x1b[45;1m",
			RGB4::LightCyan => "\x1b[46;1m",
			RGB4::White => "\x1b[47;1m",
		});
	}
	fn bg(&self, buf: &mut String) {
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
			RGB4::DarkGray => "\x1b[30;1m",
			RGB4::LightRed => "\x1b[31;1m",
			RGB4::LightGreen => "\x1b[32;1m",
			RGB4::LightYellow => "\x1b[33;1m",
			RGB4::LightBlue => "\x1b[34;1m",
			RGB4::LightMagenta => "\x1b[35;1m",
			RGB4::LightCyan => "\x1b[36;1m",
			RGB4::White => "\x1b[37;1m",
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

pub struct Cursor<TColor: RGB> {
	pub buf: String,
	pub fg: TColor,
	pub bg: TColor,
	pub attr: TextAttr,
	pub x: u16,
	pub y: u16,
}

impl<TColor: RGB + Default> Default for Cursor<TColor> {
	fn default() -> Self {
		Cursor::<TColor> {
			buf: String::new(),
			attr: TextAttr::empty(),
			fg: Default::default(),
			bg: Default::default(),
			x: 1,
			y: 1,
		}
	}
}

impl<TColor: RGB + Eq> Cursor<TColor> {
	pub fn new(fg: TColor, bg: TColor) -> Self {
		Cursor::<TColor> {
			buf: String::new(),
			attr: TextAttr::empty(),
			fg: fg,
			bg: bg,
			x: 1,
			y: 1,
		}
	}

	pub fn esc(&mut self, s: &str){
		self.buf.push('\x1b');
		self.buf.push('[');
		self.buf.push_str(s)
	}
	pub fn escch(&mut self, c: char){
		self.buf.push('\x1b');
		self.buf.push('[');
		self.buf.push(c)
	}
	pub fn clearattr(&mut self){
		self.attr.clear();
		self.escch('m')
	}
	pub fn hasallattr(&self, ta: TextAttr) -> bool{
		self.attr.contains(ta)
	}
	pub fn hasanyattr(&self, ta: TextAttr) -> bool{
		self.attr.intersects(ta)
	}
	pub fn setattr(&mut self, ta: TextAttr){
		if ta == self.attr { return }
		self.buf.push('\x1b');
		self.buf.push('[');
		if ta.contains(self.attr) {
			for &(attr, code) in TA_CHARS.iter() {
				if ta.contains(attr) && !self.attr.contains(attr) {
					self.buf.push(code);
					self.buf.push(';')
				}
			}
		} else {
			self.escch('m');
			for &(attr, code) in TA_CHARS.iter() {
				if ta.contains(attr) {
					self.buf.push(code);
					self.buf.push(';')
				}
			}
		}
		unsafe { *(self.buf.as_mut_vec().last_mut().unwrap()) = b'm' }
		self.attr = ta;
	}
	pub fn setbold(&mut self){
		self.attr.insert(TA_BOLD);
		self.esc("1m")
	}
	pub fn setdim(&mut self){
		self.attr.insert(TA_DIM);
		self.esc("2m")
	}
	pub fn setunder(&mut self){
		self.attr.insert(TA_UNDER);
		self.esc("4m")
	}
	pub fn setblink(&mut self){
		self.attr.insert(TA_BLINK);
		self.esc("5m")
	}
	pub fn setrev(&mut self){
		self.attr.insert(TA_REV);
		self.esc("7m")
	}
	pub fn unsetbold(&mut self){
		self.attr.remove(TA_BOLD);
		self.esc("21m")
	}
	pub fn unsetrev(&mut self){
		self.attr.remove(TA_REV);
		self.esc("27m")
	}
	pub fn wrapon(&mut self){
		self.buf.push_str("\x1b7h")
	}
	pub fn wrapoff(&mut self){
		self.buf.push_str("\x1b7l")
	}
	pub fn up1(&mut self){
		self.y -= 1;
		self.escch('A')
	}
	pub fn down1(&mut self){
		self.y += 1;
		self.escch('B')
	}
	pub fn right1(&mut self){
		self.x -= 1;
		self.escch('C')
	}
	pub fn left1(&mut self){
		self.x += 1;
		self.escch('D')
	}
	pub fn up(&mut self, n: u16){
		self.y -= n;
		self.esc(&format!("{}A", n))
	}
	pub fn down(&mut self, n: u16){
		self.y += n;
		self.esc(&format!("{}B", n))
	}
	pub fn right(&mut self, n: u16){
		self.x -= n;
		self.esc(&format!("{}C", n))
	}
	pub fn left(&mut self, n: u16){
		self.x += n;
		self.esc(&format!("{}D", n))
	}
	pub fn x1down(&mut self, n: u16){
		self.x = 1;
		self.y += n;
		self.esc(&format!("{}E", n))
	}
	pub fn x1up(&mut self, n: u16){
		self.x = 1;
		self.y -= n;
		self.esc(&format!("{}F", n))
	}
	pub fn setx(&mut self, x: u16){
		self.x = x;
		self.esc(&format!("{}G", x))
	}
	pub fn sety(&mut self, y: u16){
		self.y = y;
		self.esc(&format!("{}d", y))
	}
	pub fn resetxy(&mut self){
		self.x = 1;
		self.y = 1;
		self.escch('H')
	}
	pub fn mv(&mut self, x: u16, y: u16){
		self.x = x;
		self.y = y;
		self.esc(&format!("{};{}H",y,x))
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
	pub fn getattr(&self) -> TextAttr{
		self.attr
	}
	pub fn showcur(&mut self){
		self.esc("?23h")
	}
	pub fn hidecur(&mut self){
		self.esc("?25l")
	}
	pub fn spame(&mut self){
		self.buf.push_str("\x1b#8")
	}
	pub fn getxy(&self) -> (u16, u16){
		(self.x, self.y)
	}
	pub fn setfg(&mut self, rgb: TColor) {
		if self.fg != rgb {
			self.fg = rgb;
			self.fg.fg(&mut self.buf);
		}
	}
	pub fn setbg(&mut self, rgb: TColor) {
		if self.bg != rgb {
			self.bg = rgb;
			self.bg.bg(&mut self.buf);
		}
	}
	pub fn prchr(&mut self, c: char){
		self.x += 1;
		self.buf.push(c)
	}
	pub fn print(&mut self, s: &str){
		let mut rsp = s.rsplit('\n');
		let last = rsp.next().unwrap();
		let lines = rsp.count();
		self.x += last.len() as u16;
		self.y += lines as u16;
		self.buf.push_str(s)
	}
	pub fn clear(&mut self) -> io::Result<()> {
		self.buf.clear();
		self.x = 1;
		self.y = 1;
		Cursor::<TColor>::dropclear()
	}
	pub fn dropclear() -> io::Result<()> {
		io::stdout().write_all(b"\x1bc")
	}
	pub fn flush(&mut self) -> io::Result<()> {
		let mut out = io::stdout();
		try!(out.write_all(self.buf.as_bytes()));
		try!(out.flush());
		Ok(self.buf.clear())
	}
}
