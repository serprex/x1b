#![crate_type = "lib"]
#![crate_name = "x1b"]
#![allow(dead_code)]
#[macro_use]
extern crate bitflags;

mod cu;
mod query;
mod x1b;

pub use cu::*;
pub use query::*;
pub use x1b::*;

#[cfg(test)]
mod test {
	use query;
	use x1b;
	#[test]
	fn curspos() {
		let mut rc: x1b::Cursor = Default::default();
		rc.setfg((0x33, 0x66, 0x99));
		rc.print("asdf\n");
		assert!(rc.flush().is_ok());
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
