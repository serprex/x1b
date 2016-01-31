use x1b::*;
use std::io;
use std::io::{Write};
use std::mem::{transmute};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct TCell{
	ch: u32,
}
impl TCell {
	pub fn gch(&self) -> char {
		unsafe { transmute(self.ch&0x000FFFFF) }
	}
	pub fn gta(&self) -> TextAttr {
		unsafe { transmute((self.ch >> 24) as u8) }
	}
	pub fn sch(&mut self, ch: char) {
		self.ch = (self.ch&0xFF000000)|(ch as u32)
	}
	pub fn sta(&mut self, ta: TextAttr) {
		self.ch = (self.ch&0x00FFFFFF)|((ta.bits() as u32)<<24)
	}
	pub fn new(ch: char, ta: TextAttr) -> Self {
		TCell { ch: (ch as u32)|((ta.bits() as u32)<<24) }
	}
	pub fn from_char(ch: char) -> Self {
		TCell::new(ch, TextAttr::empty())
	}
}

pub struct Curse {
	w: u16,
	h: u16,
	old: Vec<TCell>,
	new: Vec<TCell>,
}

impl Curse {
	pub fn new(w: u16, h: u16) -> Self {
		let wh = (w*h) as usize;
		let empty = vec![TCell::from_char(' '); wh];
		Curse {
			w: w,
			h: h,
			old: empty.clone(),
			new: empty,
		}
	}
	pub fn clear(&mut self, tc: TCell) {
		for c in self.new.iter_mut() {
			*c = tc
		}
	}
	pub fn setxy(&mut self, x: u16, y: u16, tc: TCell) {
		if x<self.w && y<self.h {
			*unsafe { self.new.get_unchecked_mut((x+y*self.w) as usize) } = tc
		}
	}
	pub fn printxy(&mut self, x: u16, y: u16, s: &str, ta: TextAttr) {
		let mut xx = 0;
		let mut yy = 0;
		for c in s.chars() {
			if c == '\n' {
				xx = 0;
				yy += 1;
			} else if c == '\r' {
				xx = 0;
			} else if c == '\t' {
				for _ in 0..4-xx&!3 {
					self.setxy(x+xx, y+yy, TCell::new(' ', ta));
					xx += 1;
				}
			} else {
				self.setxy(x+xx, y+yy, TCell::new(c, ta));
				xx += 1;
			}
		}
	}
	pub fn refresh(&mut self) -> io::Result<()> {
		let mut cursor = Cursor::default();
		cursor.escch('H');
		for y in 0..self.h {
			for x in 0..self.w {
				let idx = (x+y*self.w) as usize;
				let (oldtc, newtc) = unsafe {
					(self.old.get_unchecked_mut(idx), self.new.get_unchecked(idx))
				};
				if oldtc != newtc {
					*oldtc = *newtc;
					let ch = newtc.gch();
					let ta = newtc.gta();
					cursor.mv(x+1, y+1);
					cursor.setattr(ta);
					cursor.prchr(ch)
				}
			}
		}
		cursor.flush()
	}
}
