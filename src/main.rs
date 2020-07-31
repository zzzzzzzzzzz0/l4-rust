use std::{env, path::{Path}, process};

type I_ = fn(l4_so:&str, a:&[String], ret:&mut Vec<String>) -> i32;

fn main() {
	let a: Vec<String> = env::args().collect();
	let mut path;
	{
		let path1 = Path::new(&a[0]);
		path = path1.canonicalize().unwrap();
		path.set_file_name("l4.so");
		if !path.exists() {
			path.set_file_name("libl4.so");
		}
	}
	let lib = libloading::Library::new(&path);
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
	let mut ret2 = vec![];
	let ret = i__(path.to_str().unwrap(), &a, &mut ret2);
	//println!("{} {:?}", ret, ret2);
	process::exit(ret);
}
