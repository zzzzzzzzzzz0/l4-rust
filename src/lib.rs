use zhscript2::{u_::*, as_ref__, as_mut_ref__};
use zs2_l4_;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};


#[no_mangle]
pub extern "C" fn i2_free__(s:*mut c_char) {
	if !s.is_null() {
		unsafe {
			CString::from_raw(s);
		}
	}
}

#[no_mangle]
pub extern "C" fn i2__(s:*const c_char, c:u8) -> *mut c_char {
	let mut s2 = String::new();
	unsafe {
		let s = CStr::from_ptr(s);
		if let Ok(s) = s.to_str() {
			s2.push_str(s)
		}
	}
	if !s2.is_empty() {
		let w = zs2_l4_::i_::w__();
		let w2 = as_ref__!(w);
		let mut wm:WorldMut_ = Default::default();
		let  q = Qv_::new2(Some(w2.clone().top_q_));
		let mut ret2 = result_::List_::new();
		let ret3 = eval_::hello__(&s2, &code_::Env_::new(qv_::t__(q), w.clone()), &mut wm, &mut ret2);
		s2.clear();
		if ret3.is_ok() {
			match c {
				b'j' | b'1' => ret2.s2__(c, &mut s2),
				_ => {}
			}
		} else {
			println!("{:?} {:?}", ret3, ret2);
		}
	}
	CString::new(s2).unwrap().into_raw()
}

#[no_mangle]
pub extern fn i__(l4_so:&str, a:&[String], ret:&mut Vec<String>) -> i32 {
	let w = zs2_l4_::i_::w__();
	{
		let w = as_mut_ref__!(w);
		let mut top_q = as_mut_ref__!(w.top_q_);
		top_q.val__("l4.so", l4_so);
	}	

	let mut v = vec![];
	for i in a {
		v.push(i.to_string())
	}
	
	let mut wm:WorldMut_ = Default::default();
	let w2 = as_ref__!(w);
	let mut q = Qv_::new2(Some(w2.clone().top_q_));
	let mut ret2 = result_::List_::new();
	let ret3 = w2.clone().hello3__(&mut v.into_iter(), true, true, false, &mut q, &mut wm, &mut ret2);
	*ret = ret2.to_vec__();
	w2.clone().ret__(ret3)
}

#[no_mangle]
extern fn clpars4_set__(env:&code_::Env_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	zs2_l4_::clpars4_::set__(&as_ref__!(env.q).args_.to_vec__(), ret)
}

#[no_mangle]
extern fn clpars4_par__(env:&code_::Env_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	match zs2_l4_::clpars4_::par__(0, 1, env, wm, ret) {
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
extern fn clpars4_help__(env:&code_::Env_, _wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
	zs2_l4_::clpars4_::help__(0, env.q.clone(), ret);
	ok__()
}
