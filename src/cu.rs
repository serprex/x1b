use x1b::*;
use std::collections::hash_map::{HashMap, Entry};
use std::hash::BuildHasherDefault;
use std::io;
use std::cmp::{PartialEq, Eq};
use std::mem::transmute;
use fnv::FnvHasher;

pub struct Char<TColor: RGB> {
	pub fg: TColor,
	pub bg: TColor,
	ch: u32,
}

impl<TColor: RGB + Copy> Copy for Char<TColor> { }
impl<TColor: RGB + Eq> Eq for Char<TColor> { }

impl<TColor: RGB + Clone> Clone for Char<TColor> {
	fn clone(&self) -> Self {
		Char::<TColor> {
			ch: self.ch,
			fg: self.fg.clone(),
			bg: self.bg.clone(),
		}
	}
}

impl<TColor: RGB + PartialEq> PartialEq for Char<TColor> {
	fn eq(&self, other: &Self) -> bool {
		self.ch == other.ch && self.fg == other.fg && self.bg == other.bg
	}
}

impl<TColor: RGB + Default> Default for Char<TColor> {
	fn default() -> Self {
		Char::<TColor> {
			ch: Default::default(),
			fg: Default::default(),
			bg: Default::default(),
		}
	}
}

impl<TColor: RGB + Default> From<char> for Char<TColor> {
	fn from(ch: char) -> Self {
		Char::<TColor> {
			ch: ch as u32,
			fg: Default::default(),
			bg: Default::default()
		}
	}
}

impl<TColor: RGB + Default> Char<TColor> {
	pub fn new_with_attr(ch: char, ta: TextAttr) -> Self {
		Char::<TColor> {
			ch: (ch as u32)|((ta.bits() as u32)<<24),
			fg: Default::default(),
			bg: Default::default(),
		}
	}
}

impl<TColor: RGB> Char<TColor> {
	pub fn new_with_color(ch: char, fg: TColor, bg: TColor) -> Self {
		Char::<TColor> { ch: ch as u32, fg: fg, bg: bg }
	}
	pub fn new(ch: char, ta: TextAttr, fg: TColor, bg: TColor) -> Self {
		Char::<TColor> { ch: (ch as u32)|((ta.bits() as u32)<<24), fg: fg, bg: bg }
	}
	pub fn get_char(&self) -> char {
		unsafe { transmute(self.ch&0x00ffffff) }
	}
	pub fn get_attr(&self) -> TextAttr {
		unsafe { transmute((self.ch >> 24) as u8) }
	}
	pub fn set_char(&mut self, ch: char) {
		self.ch = (self.ch&0xff000000)|(ch as u32)
	}
	pub fn set_attr(&mut self, ta: TextAttr) {
		self.ch = (self.ch&0x00ffffff)|((ta.bits() as u32)<<24)
	}
}

pub struct Curse<TColor: RGB> {
	old: Vec<Char<TColor>>,
	new: HashMap<u32, Char<TColor>, BuildHasherDefault<FnvHasher>>,
	cursor: Cursor<TColor>,
	w: u16,
	h: u16,
}

impl<TColor: RGB + Default + Clone> Curse<TColor> {
	pub fn new(w: u16, h: u16) -> Self {
		Curse::<TColor> {
			w: w,
			h: h,
			old: vec![Char::<TColor>::from(' '); (w*h) as usize],
			new: Default::default(),
			cursor: Cursor::default(),
		}
	}
}

impl<TColor: RGB + Eq + Copy> Curse<TColor> {
	pub fn new_with_cursor(cursor: Cursor<TColor>, w: u16, h: u16) -> Curse<TColor> {
		Curse::<TColor> {
			w: w,
			h: h,
			old: vec![Char::<TColor>::new_with_color(' ', cursor.fg, cursor.bg); (w*h) as usize],
			new: Default::default(),
			cursor: cursor,
		}
	}
	pub fn clear(&mut self, tc: Char<TColor>) {
		let len = self.old.len() as u32;
		for idx in 0..len {
			unsafe { self.setidx(idx, tc) }
		}
	}
	pub unsafe fn setidx(&mut self, idx: u32, tc: Char<TColor>) {
		match self.new.entry(idx) {
			Entry::Occupied(mut entry) => {entry.insert(tc);},
			Entry::Vacant(entry) => {
				if *self.old.get_unchecked(idx as usize) != tc {
					entry.insert(tc);
				}
			},
		}
	}
	pub fn set(&mut self, x: u16, y: u16, tc: Char<TColor>) {
		let w = self.w;
		if x<w && y<self.h {
			unsafe { self.setidx(x as u32 + y as u32 * w as u32, tc) }
		}
	}
	pub fn printnows(&mut self, x: u16, y: u16, s: &str, ta: TextAttr, fg: TColor, bg: TColor) {
		for (xx, c) in s.chars().enumerate() {
			self.set(x+(xx as u16), y, Char::new(c, ta, fg, bg));
		}
	}
	pub fn print(&mut self, x: u16, y: u16, s: &str, ta: TextAttr, fg: TColor, bg: TColor) {
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
					self.set(x+xx, y+yy, Char::new(' ', ta, fg, bg));
					xx += 1;
				}
			} else {
				self.set(x+xx, y+yy, Char::new(c, ta, fg, bg));
				xx += 1;
			}
		}
	}
	pub fn refresh(&mut self) -> io::Result<()> {
		for (&idx, newtc) in self.new.iter() {
			let oldtc = unsafe { self.old.get_unchecked_mut(idx as usize) };
			if oldtc != newtc {
				*oldtc = *newtc;
				let ch = newtc.get_char();
				let ta = newtc.get_attr();
				let (x, y) = ((idx%self.w as u32) as u16, (idx/self.w as u32) as u16);
				self.cursor.mv(x+1, y+1);
				self.cursor.setattr(ta);
				self.cursor.prchr(ch)
			}
		}
		self.new.clear();
		self.cursor.flush()
	}
	pub fn perframe_refresh_then_clear(&mut self, tc: Char<TColor>) -> io::Result<()> {
		let mut rmxyc: Vec<u32> = Vec::with_capacity(self.new.len());
		for (&idx, newtc) in self.new.iter_mut() {
			let oldtc = unsafe { self.old.get_unchecked_mut(idx as usize) };
			if oldtc != newtc {
				*oldtc = *newtc;
				let ch = newtc.get_char();
				let ta = newtc.get_attr();
				let (x, y) = ((idx%self.w as u32) as u16, (idx/self.w as u32) as u16);
				self.cursor.mv(x+1, y+1);
				self.cursor.setattr(ta);
				self.cursor.setbg(newtc.bg);
				self.cursor.setfg(newtc.fg);
				self.cursor.prchr(ch);
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
		self.cursor.flush()
	}
}
