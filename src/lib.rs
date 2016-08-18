#![crate_type = "lib"]
#![crate_name = "x1b"]
#![allow(dead_code)]
#[macro_use]
extern crate bitflags;
extern crate fnv;

mod x1b;
mod cu;
mod query;

pub use x1b::*;
pub use cu::*;
pub use query::*;

#[cfg(test)]
mod test {
	use x1b;
	use query;
	#[test]
	fn curspos() {
		let mut rc: x1b::Cursor<(u8, u8, u8)> = Default::default();
		rc.setfg((0x33, 0x66, 0x99));
		rc.print("asdf\n");
		let (rx, ry) = rc.getxy();
		rc.print(&format!("{} {} ?", rx, ry));
		let (rx, ry) = rc.getxy();
		assert_eq!(ry, 2);
		assert!(rc.flush().is_ok());
		assert_eq!(ry, 2);
		assert_eq!(rx, 6);
	}
	#[test]
	fn getcursorxyttywh() {
		print!("\r\n");
		assert_eq!(query::get_cursor_xy().unwrap().0, 1);
		println!("{:?}", query::get_cursor_xy());
		println!("{:?}", query::get_tty_wh());
		println!("{:?}", query::get_cursor_xy());
		println!("{:?}", query::get_tty_wh_dirty());
		println!("{:?}", query::get_cursor_xy());
	}
}
