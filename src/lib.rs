use zhscript2::{u_::*, as_ref__, as_mut_ref__};
use zs2_l4_;
use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use lazy_static::*;

lazy_static! {
	pub static ref W_: world_::T_ = zs2_l4_::i_::w__();
}

fn env__() -> code_::Env_ {
	let q = Qv_::new2(Some(as_ref__!(W_).top_q_.clone()));
	code_::Env_::new(t__(q), W_.clone(), t__(result_::List_::new()))
}

fn c_i_2__(s:*const c_char, c:u8, env:&code_::Env_) -> *mut c_char {
	let mut s2 = String::new();
	unsafe {
		let s = CStr::from_ptr(s);
		if let Ok(s) = s.to_str() {
			s2.push_str(s)
		}
	}

	if !s2.is_empty() {
		let ret2 = eval_::hello__(&s2, env);
		s2.clear();
		let ret = as_ref__!(env.ret);
		if ret2.is_ok() {
			match c {
				b'j' | b'1' => ret.s2__(c, &mut s2),
				_ => {}
			}
		} else {
			println!("{:?} {:?}", ret2, ret);
		}
	}
	CString::new(s2).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn c_i2__(s:*const c_char, c:u8, env0:*const c_void) -> *mut c_char {
	let env;
	unsafe {
		let env0 = env0 as *const code_::Env_;
		env = &*env0;
	}
	c_i_2__(s, c, env)
}

#[no_mangle]
pub extern "C" fn c_env__() -> *const c_void {
	let env = env__();
	&env as *const _ as *const c_void
}

#[no_mangle]
pub extern "C" fn c_i__(s:*const c_char, c:u8) -> *mut c_char {
	let env = env__();
	c_i_2__(s, c, &env)
}

#[no_mangle]
pub extern "C" fn c_i_free__(s:*mut c_char) {
	if !s.is_null() {
		unsafe {
			CString::from_raw(s);
		}
	}
}

#[no_mangle]
pub extern fn i__(l4_so:&str, a:&[String], ret:&mut Vec<String>) -> i32 {
	{
		let w = as_mut_ref__!(W_);
		let mut top_q = as_mut_ref__!(w.top_q_);
		top_q.val__("l4.so", l4_so);
	}	

	let mut v = vec![];
	for i in a {
		v.push(i.to_string())
	}
	
	let w = as_ref__!(W_);
	let mut q = Qv_::new2(Some(w.clone().top_q_));
	let ret2 = t__(result_::List_::new());
	let ret3 = w.clone().hello3__(&mut v.into_iter(), true, true, false, &mut q, ret2.clone());
	*ret = as_ref__!(ret2).to_vec__();
	w.ret__(ret3)
}

#[no_mangle]
extern fn clpars4_set__(env:&code_::Env_) -> Result2_ {
	zs2_l4_::clpars4_::set__(&as_ref__!(as_ref__!(env.q).args_).to_vec__(), &mut as_mut_ref__!(env.ret))
}

#[no_mangle]
extern fn clpars4_par__(env:&code_::Env_) -> Result2_ {
	match zs2_l4_::clpars4_::par__(0, 1, env) {
		Ok(()) => ok__(),
		Err((i, ret2)) => {
			match i {
				251 => result2_::n__(jump_::QUIT_),
				_ => ret2
			}
		}
	}
}

#[no_mangle]
extern fn clpars4_help__(env:&code_::Env_) -> Result2_ {
	zs2_l4_::clpars4_::help__(0, env.q.clone(), &mut as_mut_ref__!(env.ret));
	ok__()
}
