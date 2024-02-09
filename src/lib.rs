use zhscript2::{u_::*, u2_::clpars_, as_ref__, as_mut_ref__};
use zs2_l4_::funcs_::callback_::*;
use std::{os::raw::{c_char, c_int}, ptr};
use lazy_static::*;
//mod test_;

lazy_static! {
	pub static ref W_: world_::T_ = zs2_l4_::i_::w__();
}

fn env__() -> code_::Env_ {
	let q = Qv_::new2(Some(as_ref__!(W_).top_q_.clone()));
	code_::Env_::new(t__(q), W_.clone(), t__(result_::List_::new()))
}

#[no_mangle]
pub extern "C" fn c_i__(errret2:*mut i32, ctl:u8, s:*const c_char, argc:i32, argv:&mut &mut i8) -> *mut c_char {
	cb__(&mut env__() as *mut code_::Env_, ptr::null_mut(), errret2, ctl, s, argc, argv)
}

#[no_mangle]
pub extern "C" fn c_i_free__(s:*mut c_char) {
	free__(s)
}

#[no_mangle]
pub extern "C" fn c_i___() -> u64 {c_i__ as u64}
#[no_mangle]
pub extern "C" fn c_i_free___() -> u64 {c_i_free__ as u64}

#[no_mangle]
pub extern "C" fn c_add__(ret0:*mut result_::List_, s:*const c_char) {
	let ret;
	unsafe {
		ret = &mut *ret0;
	}
	ret.add__(&s__(s));
}

#[no_mangle]
pub extern "C" fn c_dunhao__(ret0:*mut result_::List_) {
	let mut ret;
	unsafe {
		ret = &mut *ret0;
	}
	as_ref__!(W_).dunhao__(&mut ret);
}

#[no_mangle]
pub extern "C" fn c_lpars__(errret2:*mut i32, argc : c_int , argv : &mut &mut c_char) {
	let args = args__(argc, argv).unwrap();
	let mut q = Qv_::new2(None);
	let ret = world_::clpars__(&mut args.into_iter(), true, true, true, false, &mut q, W_.clone());
	if ret.is_ok() {
		let top_q = &as_ref__!(W_).top_q_;
		let mut top_q = as_mut_ref__!(top_q);
		top_q.args_ = q.args_;
		top_q.src_ = q.src_;
	} else {
		ret2__(errret2, ret__(ret, ptr::null_mut()));
	}
}

#[no_mangle]
pub extern "C" fn c_cfg_shl__(s:*const c_char) {
	as_mut_ref__!(W_).cfg_.shl_ = s__(s);
}
#[no_mangle]
pub extern "C" fn c_path__(s:*const c_char) {
	as_mut_ref__!(W_).path2__(s__(s));
}
#[no_mangle]
pub extern "C" fn c_dbg_bp2__(s:*const c_char) -> bool {
	//if cfg!(debug_assertions) {
		as_mut_ref__!(W_).dbg_.bp2_ == s__(s)
	//} else {false}
}

#[no_mangle]
pub extern "C" fn c_load__(src1:*const c_char) -> c_int {
	let mut src1 = s__(src1);
	let w = || as_ref__!(W_);
	if src1.is_empty() {
		let top_q = &w().top_q_;
		let top_q = as_ref__!(top_q);
		if top_q.src_.is_empty() {
			return clpars_::ARG_NE_
		}
		src1.push_str(&top_q.src_);
	}
	let env = env__();
	let q = &env.q;
	let mut src = String::new();
	let mut ret;
	{
		if w().cfg_.src_is_file_ {
			eval_::ok_src__(&src1, q.clone(), W_.clone());
			ret = eval_::src__(&mut src, q.clone(), W_.clone());
		} else {
			src.push_str(&src1);
			ret = ok__()
		}
	}
	if ret.is_ok() {
		{
			let top_q = &w().top_q_;
			let top_q = as_ref__!(top_q);
			let mut q2 = as_mut_ref__!(q);
			q2.args_ = top_q.args_.clone();
			q2.src_ = src1;
		}
		ret = eval_::hello__(&src, &env);
	}
	ret__(ret, ptr::null_mut())
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
	let ret3 = w.clone().hello3__(&mut v.into_iter(), true, true, true, &mut q, ret2.clone());
	*ret = as_ref__!(ret2).to_vec__();
	w.ret__(ret3)
}

use std::{os::raw::{c_void}, ffi::{CString}};

type Ret_ = extern fn(*const c_char, *const c_void);
type Erret_ = extern fn(i32, *const c_char, *const c_char, i32, *const c_void);
type Ret2_ = *const c_void;

#[no_mangle]
pub extern "C" fn true__(s:*const c_char) -> bool {
	t_::true__(&s__(s))
}

fn hello_ret__(s:&str, ret:Ret_, ret2:Ret2_) {
	let s = CString::new(s).unwrap();
	ret(s.as_ptr(), ret2);
}

#[no_mangle]
pub extern "C" fn hello__(s:*const c_char, src_is_file:bool, argc: u32, argv: *const *const c_char, from: u32,
		env0:*mut code_::Env_, ret:Ret_, ret2:Ret2_, erret:Erret_, erret2:Ret2_) -> bool {
	let s = s__(s);
	let mut arg = vec![];
	{
		let cfg = &mut as_mut_ref__!(W_).cfg_;
		cfg.src_is_file_ = src_is_file;
		cfg.jvhao_dunhao_ = true;
	}
	if from == 0 {
		arg.push(s.to_string());
	}
	for i in 0 .. argc as isize {
		arg.push(s__(unsafe {*argv.offset(i)}));
	}
	if from == 1 {
		let w = as_ref__!(W_);
		let mut q = as_mut_ref__!(w.top_q_);
		q.src_ = s.to_string();
		if arg.len() > 2 {
			let mut a = as_mut_ref__!(q.args_);
			let mut b = 0;
			let i3 = if arg[1] == w.shebang_flag_ {4} else {3};
			for i in &arg {
				b += 1;
				if b < i3 {
					continue;
				} else if b > i3 {
					w.dunhao__(&mut a);
				}
				a.add__(i);
			}
		}
	}
	let mut q = Qv_::new2(Some(as_ref__!(W_).clone().top_q_));
	q.src_ = s.to_string();
	let ret4 = t__(result_::List_::new());
	let ret3 = world_::hello__(&mut arg.into_iter(), from > 0, true, true, &mut q,
		if !env0.is_null() {
			let env;
			unsafe {
				env = &*env0;
			}
			env.w.clone()
		} else {
			W_.clone()
		},
		ret4.clone());
	for i in as_ref__!(ret4).to_vec__() {
		hello_ret__(&i, ret, ret2)
	}
	//as_ref__!(W_).ret__(ret3)
	if let Err((mut i, i2, s, s2)) = ret3 {
		let mut is_quit = 0;
		match i {
			jump_::QUIT_ => if i2 != jump_::NO_ {
				i = i2;
				is_quit = 1;
			}
			jump_::RETURN_ => if s.is_empty() {
				i = 0;
				is_quit = 1;
			}
			jump_::CONTINUE_ => if s.is_empty() {
				i = 0;
				is_quit = 2;
			}
			jump_::BREAK_ => if s.is_empty() {
				i = 0;
				is_quit = 3;
			}
			_ => {}
		}
		if is_quit == 0 {
			i = result2_::exitcode__(i)
		}
		let s = CString::new(s).unwrap();
		let s2 = CString::new(s2).unwrap();
		erret(i, s.as_ptr(), s2.as_ptr(), is_quit, erret2);
		false
	} else {true}
}

#[no_mangle]
extern fn for_kws__(env:&code_::Env_) -> Result2_ {
	let mut kws2 = vec![];
	{
		let kws = &as_ref__!(env.w).kws_;
		for kw in &kws.a_ {
			//println!("{:?}",kw);
			kws2.push(kw.s_.clone());
		}
	}
	{
		use zhscript2::{var_::*, def_, set_};
		kws2.push(qv_::rem4_::TOP_.to_string());
		kws2.push(qv_::rem4_::UP_.to_string());
		kws2.push(qv_::rem4_::IN_.to_string());
		kws2.push(set_::PRIV_.to_string());
		kws2.push(set_::VAL_.to_string());
		kws2.push(def_::NO_ARG_.to_string());
		kws2.push(def_::BACK_ARG_.to_string());
		kws2.push(zhscript2::u_::def_::argname_::SP_.to_string());
		kws2.push(arg_::ARGS_.to_string());
		kws2.push(arg_::ARG_.to_string());
		kws2.push(arg_::ARGC_.to_string());
		kws2.push(code_::attr_::ATTR_.to_string());
		kws2.push(world_::CR_.to_string());
		kws2.push(world_::LF_.to_string());
		kws2.push(world_::TAB_.to_string());
		kws2.push(world_::ESC_.to_string());
	}

	let q = env.q.clone();
	let q = as_ref__!(q);
	let args = &as_ref__!(q.args_);

	let mut src = String::new();
	as_ref__!(args[0]).s__(&mut src);

	let mut ret3 = ok__();
	for kw in &kws2 {
		let q2 = Qv_::new2(q.up_.clone());
		{
			let args = &mut as_mut_ref__!(q2.args_);
			args.add__(&kw);
		}
		ret3 = eval_::hello__(&src, &mut code_::Env_::new2(t__(q2), env));
		if ret3.is_err() {break;}
	}
	ret3
}

#[no_mangle]
extern fn clpars4_set__(env:&code_::Env_) -> Result2_ {
	zs2_l4_::clpars4_::set__(&as_ref__!(as_ref__!(env.q).args_).to_vec__(), &mut as_mut_ref__!(env.ret))
}

#[no_mangle]
extern fn clpars4_par__(env:&code_::Env_) -> Result2_ {
	match zs2_l4_::clpars4_::par__(0, 1, 1, env) {
		Ok(()) => ok__(),
		Err((i, ret2)) => {
			if i >= clpars_::ARG_NE_ {
				if let Err((_, _, s, s2)) = ret2 {
					let s3 = if !s.ends_with("\n") && !s2.starts_with("\n") {"\n"} else {""};
					let s = s + s3 + &s2;
					return result2_::err4__(jump_::QUIT_, i, s, "".to_string())
				}
				return result2_::n2__(jump_::QUIT_, i)
			}
			ret2
		}
	}
}

#[no_mangle]
extern fn clpars4_help__(env:&code_::Env_) -> Result2_ {
	zs2_l4_::clpars4_::help__(0, env.q.clone(), &mut as_mut_ref__!(env.ret));
	ok__()
}
#[no_mangle]
extern fn clpars4_help2__(env:&code_::Env_) -> Result2_ {
	zs2_l4_::clpars4_::help2__(env)
}

#[no_mangle]
extern fn thread4__(env:&code_::Env_) -> Result2_ {
	zs2_l4_::thread4_::start__(&vec![], env)
}

#[no_mangle]
extern fn thread4_stop__(env:&code_::Env_) -> Result2_ {
	zs2_l4_::thread4_::stop__(env)
}

#[no_mangle]
extern fn regexpr4_for__(env:&code_::Env_) -> Result2_ {
	let mut args = as_ref__!(as_ref__!(env.q).args_).to_vec__();
	let get = args.remove(0).parse::<u8>().unwrap();
	zs2_l4_::regexpr4_::for__(&args, get, env)
}

#[no_mangle]
extern fn regexpr4_test__(env:&code_::Env_) -> Result2_ {
	match zs2_l4_::regexpr4_::test__(&as_ref__!(as_ref__!(env.q).args_).to_vec__()) {
		Ok(b) => {
			if b {
				as_mut_ref__!(env.ret).add__("1")
			}
			ok__()
		}
		Err(e) => e
	}
}

#[no_mangle]
extern fn forqv__(env:&code_::Env_) -> Result2_ {
	zs2_l4_::forqv_::z__(env, 0)
}

#[test]
fn test1__() {
	use zhscript2::u_::{code_};
	use std::{ptr};
	let env0:*mut code_::Env_ = ptr::null_mut();
	assert_eq!(!env0.is_null(), false);
	let w =
		if !env0.is_null() {
			let env;
			unsafe {
				env = &*env0;
			}
			env.w.clone()
		} else {
			W_.clone()
		};
}
