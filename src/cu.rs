use x1b::*;
use std::mem::{transmute};

#[derive(Copy, Clone, Default, Debug)]
struct TCell{
	ch: u32,
}
impl TCell {
	pub fn gch(&self) -> char {
		unsafe { transmute(self.ch&0x000FFFFF) }
	}
	pub fn gat(&self) -> TextAttr {
		unsafe { transmute((self.ch >> 24) as u8) }
	}
	pub fn sch(&mut self, ch: char) {
		self.ch = (self.ch&0xFF000000)|(ch as u32)
	}
	pub fn sat(&mut self, ta: TextAttr) {
		self.ch = (self.ch&0x00FFFFFF)|((ta.bits() as u32)<<24)
	}
	pub fn new(ch: char, ta: TextAttr) -> Self {
		TCell { ch: (ch as u32)|((ta.bits() as u32)<<24) }
	}
	pub fn from_char(ch: char) -> Self {
		TCell::new(ch, TextAttr::empty())
	}
}

struct Curses {
	c: Cursor,
	w: u16,
	h: u16,
	old: Vec<TCell>,
	new: Vec<TCell>,
}

impl Curses {
	pub fn new(w: u16, h: u16) -> Self {
		let wh = (w*h) as usize;
		Curses {
			c: Cursor::default(),
			w: w,
			h: h,
			old: Vec::with_capacity(wh),
			new: Vec::with_capacity(wh),
		}
	}
	pub fn geth(&self) -> u16 {
		self.old.len() as u16 / self.w
	}
	pub fn setxy(&mut self, x: u16, y: u16, ch: char, ta: TextAttr) {
		self.new[(x+y*self.w) as usize] = TCell::new(ch, ta)
	}
	pub fn refresh(&mut self) {
	}
}
