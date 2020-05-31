use std::{env, path::{Path}, process};

type I_ = fn(a:&[String], ret:&mut Vec<String>) -> i32;

fn main() {
	let a: Vec<String> = env::args().collect();
	let path = &a[0];
	let path = Path::new(&path);
	let mut path = path.canonicalize().unwrap();
	path.set_file_name("l4.so");
	if !path.exists() {
		path.set_file_name("libl4.so");
	}
	let lib = libloading::Library::new(path);
	let lib = match lib {
		Ok(lib) => lib,
		Err(e) => {
			eprintln!("{}", e);
			return
		}
	};
	let i__:I_;
	unsafe {
		i__ = match lib.get::<I_>(b"i__") {
			Ok(i) => *i.into_raw(),
			Err(e) => {
				eprintln!("{}", e);
				return
			}
		}
	}
	//println!("{:?}", a);
	let mut ret2 = vec![];
	let ret = i__(&a, &mut ret2);
	//println!("{} {:?}", ret, ret2);
	process::exit(ret);
}
