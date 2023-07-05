use zhscript2::{u_::*, as_ref__, };
use super::*;

#[repr(C)]
pub struct T1_ {
	i_:i32,
	i2_:i32,
	c_:u8,
	env_:code_::Env_,
}

#[no_mangle]
pub extern "C" fn c_test_1__() {
	println!("z4")
}

#[no_mangle]
pub extern "C" fn c_test_2__() -> *mut T1_ {
	Box::into_raw(Box::new(T1_ {i_:1, i2_:22, c_:b'c', env_:env__()}))
}

#[no_mangle]
pub extern "C" fn c_test_3__(t0:*mut T1_) {
	let t;
	unsafe {
		t = &*t0;
		//t = Box::from_raw(t0);
	}
	println!("{},{},{},",t.i_,t.i2_,t.c_ as char);
	println!("{:?}",as_ref__!(t.env_.ret).a_);
}

#[no_mangle]
pub extern "C" fn c_test_4__(c:char, argc:usize, argv:&[*const c_char], ) {
	println!("c_test_4__{}", c);
	let mut i = 0;
	while i < argc {
		print!(" {:?}", &s__(argv[i]));
		i += 1;
	}
	println!();
}

#[no_mangle]
pub extern "C" fn c_test_5__(t0:*mut T1_, argc:usize, argv:&[*const c_char], ) {
	c_test_3__(t0);
	c_test_4__('五', argc, argv);
}

#[no_mangle]
pub extern "C" fn c_test_2a__() -> *mut code_::Env_ {
	Box::into_raw(Box::new(env__()))
}

#[no_mangle]
pub extern "C" fn c_test_5a__(env0:*mut code_::Env_, argc:usize, argv:&[*const c_char], ) {
	let env;
	unsafe {
		env = &*env0;
	}
	c_i_1__(&env, argc, argv);
	let _ = eval_::hello__("显示c_test_5a__‘参数’换行", env);
	println!("{:?}",as_ref__!(env.ret).a_);
}

#[no_mangle]
pub extern "C" fn c_test_5b__(env0:*mut code_::Env_, errret2:*mut i32, s:*const c_char, argc:usize, argv:&[*const c_char], ) -> *mut c_char {
	//let errret2:*mut i32 = 0 as *mut i32;
	/*let ctl:u8 = b'0';
	let env;
	unsafe {
		env = &*env0;
	}
	c_i_1__(&env, argc, argv);
	c_i_2__(&env, errret2, ctl, s)*/
	c_i__(/*env0,*/ errret2, b'0', s, argc, argv)
}

#[no_mangle]
pub extern "C" fn c_test_6__(env0:*mut code_::Env_, errret2:*mut i32, errret3:*mut i64) {
	let env;
	unsafe {
		env = &*env0;
	}
	unsafe {
	print!("c_test_6__{},{}",*errret2,*errret3);
	*errret2 += -10;
	*errret3 += -100;
	}
	println!(" [{}]",as_ref__!(env.ret).len());
}
