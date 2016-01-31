use x1b::*;
use std::collections::HashMap;
use std::io;
use std::io::{Write};
use std::mem::{transmute};

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct TCell{
	ch: u32,
}
impl TCell {
	pub fn gch(&self) -> char {
		unsafe { transmute(self.ch&0x00ffffff) }
	}
	pub fn gta(&self) -> TextAttr {
		unsafe { transmute((self.ch >> 24) as u8) }
	}
	pub fn sch(&mut self, ch: char) {
		self.ch = (self.ch&0xff000000)|(ch as u32)
	}
	pub fn sta(&mut self, ta: TextAttr) {
		self.ch = (self.ch&0x00ffffff)|((ta.bits() as u32)<<24)
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
	new: HashMap<u32, TCell>,
}

impl Curse {
	pub fn new(w: u16, h: u16) -> Self {
		Curse {
			w: w,
			h: h,
			old: vec![TCell::from_char(' '); (w*h) as usize],
			new: HashMap::new(),
		}
	}
	pub fn clear(&mut self, tc: TCell) {
		let len = self.old.len() as u32;
		for idx in 0..len {
			unsafe { self.setidx(idx, tc) }
		}
	}
	pub unsafe fn setidx(&mut self, idx: u32, tc: TCell) {
		if self.new.contains_key(&idx) || *self.old.get_unchecked(idx as usize) != tc {
			self.new.insert(idx as u32, tc);
		}
	}
	pub fn set(&mut self, x: u16, y: u16, tc: TCell) {
		let w = self.w;
		if x<w && y<self.h {
			unsafe { self.setidx((x+y*w) as u32, tc) }
		}
	}
	pub fn printnows(&mut self, x: u16, y: u16, s: &str, ta: TextAttr) {
		let mut xx = 0;
		for c in s.chars() {
			self.set(x+xx, y, TCell::new(c, ta));
			xx += 1;
		}
	}
	pub fn print(&mut self, x: u16, y: u16, s: &str, ta: TextAttr) {
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
					self.set(x+xx, y+yy, TCell::new(' ', ta));
					xx += 1;
				}
			} else {
				self.set(x+xx, y+yy, TCell::new(c, ta));
				xx += 1;
			}
		}
	}
	pub fn refresh(&mut self) -> io::Result<()> {
		let mut cursor = Cursor::default();
		for (&idx, newtc) in self.new.iter() {
			let oldtc = unsafe { self.old.get_unchecked_mut(idx as usize) };
			if oldtc != newtc {
				*oldtc = *newtc;
				let ch = newtc.gch();
				let ta = newtc.gta();
				let (x, y) = ((idx%self.w as u32) as u16, (idx/self.w as u32) as u16);
				cursor.mv(x+1, y+1);
				cursor.setattr(ta);
				cursor.prchr(ch)
			}
		}
		self.new.clear();
		cursor.flush()
	}
	pub fn perframe_refresh_then_clear(&mut self, tc: TCell) -> io::Result<()> {
		let mut cursor = Cursor::default();
		let mut rmxyc: Vec<u32> = Vec::with_capacity(self.new.len());
		for (&idx, newtc) in self.new.iter_mut() {
			let oldtc = unsafe { self.old.get_unchecked_mut(idx as usize) };
			if oldtc != newtc {
				*oldtc = *newtc;
				let ch = newtc.gch();
				let ta = newtc.gta();
				let (x, y) = ((idx%self.w as u32) as u16, (idx/self.w as u32) as u16);
				cursor.mv(x+1, y+1);
				cursor.setattr(ta);
				cursor.prchr(ch);
			}
			if *newtc != tc {
				*newtc = tc
			} else {
				rmxyc.push(idx);
			}
		}
		for idx in rmxyc {
			self.new.remove(&idx);
		}
		cursor.flush()
	}
}
