#![crate_type = "lib"]
#![crate_name = "x1b"]
#![allow(dead_code)]
#[macro_use]
extern crate bitflags;

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
		println!("");
		assert!(query::get_cursor_xy().unwrap().0 == 1);
		let mut rc: x1b::Cursor = Default::default();
		rc.rgb((0x33, 0x66, 0x99));
		rc.print("asdf\n");
		let (rx, ry) = rc.getxy();
		rc.print(&format!("{} {} ?", rx, ry));
		let (rx, ry) = rc.getxy();
		assert!(ry == 2);
		assert!(rc.flush().is_ok());
		assert!(ry == 2);
		assert!(rx == 6);
	}
}
