use x1b::*;
use std::collections::btree_map::{BTreeMap, Entry};
use std::io;
use std::cmp::{PartialEq, Eq};
use std::mem::transmute;

bitflags! {
	pub flags TextAttr: u8 {
		const TA_BOLD = 1,
		const TA_DIM = 2,
		const TA_UNDER = 4,
		const TA_BLINK = 8,
		const TA_REV = 16,
	}
}

const TA_CHARS: [(TextAttr, u8); 5] = [
	(TA_BOLD, b'1'),
	(TA_DIM, b'2'),
	(TA_UNDER, b'4'),
	(TA_BLINK, b'5'),
	(TA_REV, b'7')];

impl Default for TextAttr {
	fn default() -> Self {
		TextAttr::empty()
	}
}

impl TextAttr {
	pub fn clear(&mut self) -> bool {
		let ret = self.bits != 0;
		self.bits = 0;
		ret
	}
}

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

#[derive(Default)]
struct CursorState<TColor: RGB> {
	cursor: Cursor,
	fg: TColor,
	bg: TColor,
	x: u16,
	y: u16,
	attr: TextAttr,
}

pub struct Curse<TColor: RGB> {
	old: Vec<Char<TColor>>,
	new: BTreeMap<u32, Char<TColor>>,
	state: CursorState<TColor>,
	w: u16,
	h: u16,
}

impl<TColor: RGB + Default + Clone> Curse<TColor> {
	pub fn new(w: u16, h: u16) -> Self {
		Curse::<TColor> {
			w: w,
			h: h,
			state: Default::default(),
			old: vec![Char::<TColor>::from(' '); (w*h) as usize],
			new: Default::default(),
		}
	}
}

impl<TColor: RGB + Eq + Copy> Curse<TColor> {
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
	fn oldnewtc(state: &mut CursorState<TColor>, w: u32, idx: u32, oldtc: &mut Char<TColor>, newtc: &Char<TColor>) {
		if oldtc != newtc {
			*oldtc = *newtc;
			let ch = newtc.get_char();
			let ta = newtc.get_attr();
			let (x, y) = ((idx%w) as u16, (idx/w) as u16);
			if state.x != x || state.y != y {
				state.cursor.mv(x+1, y+1);
			}
			state.setattr(ta);
			state.setbg(newtc.bg);
			state.setfg(newtc.fg);
			state.cursor.prchr(ch);
			state.x = x + 1;
			state.y = y;
		}
	}
	pub fn refresh(&mut self) -> io::Result<()> {
		for (&idx, newtc) in self.new.iter() {
			let oldtc = unsafe { self.old.get_unchecked_mut(idx as usize) };
			Self::oldnewtc(&mut self.state, idx, self.w as u32, oldtc, newtc);
		}
		self.new.clear();
		self.state.cursor.flush()
	}
	pub fn perframe_refresh_then_clear(&mut self, tc: Char<TColor>) -> io::Result<()> {
		let mut rmxyc: Vec<u32> = Vec::with_capacity(self.new.len());
		for (&idx, newtc) in self.new.iter_mut() {
			let oldtc = unsafe { self.old.get_unchecked_mut(idx as usize) };
			Self::oldnewtc(&mut self.state, idx, self.w as u32, oldtc, newtc);
			if *newtc != tc {
				*newtc = tc
			} else {
				rmxyc.push(idx);
			}
		}
		for idx in rmxyc {
			self.new.remove(&idx);
		}
		self.state.cursor.flush()
	}
}
impl<TColor: RGB + Eq + Copy> CursorState<TColor> {
	fn setbg(&mut self, rgb: TColor) {
		if self.bg != rgb {
			self.bg = rgb;
			self.cursor.setbg(rgb);
		}
	}
	fn setfg(&mut self, rgb: TColor) {
		if self.fg != rgb {
			self.fg = rgb;
			self.cursor.setfg(rgb);
		}
	}
	fn setattr(&mut self, ta: TextAttr){
		if ta == self.attr { return }
		unsafe {
			let mut buffer = &mut self.cursor.0;
			let mut blen = buffer.len();
			buffer.reserve(12);
			*buffer.get_unchecked_mut(blen) = b'\x1b';
			*buffer.get_unchecked_mut(blen) = b'[';
			if ta.contains(self.attr) {
				blen += 2;
				for &(attr, code) in TA_CHARS.iter() {
					if ta.contains(attr) && !self.attr.contains(attr) {
						*buffer.get_unchecked_mut(blen) = code;
						*buffer.get_unchecked_mut(blen+1) = b';';
						blen += 2;
					}
				}
			} else {
				*buffer.get_unchecked_mut(blen+2) = b'm';
				blen += 3;
				for &(attr, code) in TA_CHARS.iter() {
					if ta.contains(attr) {
						*buffer.get_unchecked_mut(blen) = code;
						*buffer.get_unchecked_mut(blen+1) = b';';
						blen += 2;
					}
				}
			}
			*buffer.get_unchecked_mut(blen-1) = b'm';
			buffer.set_len(blen);
		}
		self.attr = ta;
	}
}
