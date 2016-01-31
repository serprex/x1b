use x1b::*;
use std::collections::HashSet;
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
	new: Vec<TCell>,
	xycache : HashSet<u32>,
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
			xycache: HashSet::new(),
		}
	}
	pub fn clear(&mut self, tc: TCell) {
		for (idx, c) in self.new.iter_mut().enumerate() {
			if tc != *unsafe { self.old.get_unchecked(idx) } {
				*c = tc;
				self.xycache.insert(idx as u32);
			}
		}
	}
	pub fn set(&mut self, x: u16, y: u16, tc: TCell) {
		if x<self.w && y<self.h {
			*unsafe { self.new.get_unchecked_mut((x+y*self.w) as usize) } = tc;
			self.xycache.insert((x+y*self.w) as u32);
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
		for &idxu32 in self.xycache.iter() {
			let idx = idxu32 as usize;
			let (oldtc, newtc) = unsafe {
				(self.old.get_unchecked_mut(idx), self.new.get_unchecked(idx))
			};
			if oldtc != newtc {
				*oldtc = *newtc;
				let ch = newtc.gch();
				let ta = newtc.gta();
				let (x, y) = ((idx%self.w as usize) as u16, (idx/self.w as usize) as u16);
				cursor.mv(x+1, y+1);
				cursor.setattr(ta);
				cursor.prchr(ch)
			}
		}
		cursor.flush()
	}
	pub fn perframe_refresh_then_clear(&mut self, tc: TCell) -> io::Result<()> {
		let mut cursor = Cursor::default();
		let mut rmxyc: Vec<u32> = Vec::new();
		for &idxu32 in self.xycache.iter() {
			let idx = idxu32 as usize;
			let (oldtc, newtc) = unsafe {
				(self.old.get_unchecked_mut(idx), self.new.get_unchecked_mut(idx))
			};
			if oldtc != newtc {
				*oldtc = *newtc;
				let ch = newtc.gch();
				let ta = newtc.gta();
				let (x, y) = ((idx%self.w as usize) as u16, (idx/self.w as usize) as u16);
				cursor.mv(x+1, y+1);
				cursor.setattr(ta);
				cursor.prchr(ch);
				*newtc = tc;
			} else if *newtc == tc {
				rmxyc.push(idxu32);
			} else { *newtc = tc; }
		}
		for idxu32 in rmxyc {
			self.xycache.remove(&idxu32);
		}
		cursor.flush()
	}
}
