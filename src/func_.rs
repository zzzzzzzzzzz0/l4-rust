use zhscript2::{u_::*, *};
use super::*;

pub const NAME_: &str = "函数";

pub struct Item_ {
	super_:set_::Item_,
}

impl Item_ {
	pub fn new(kw: keyword_::RI_, kws:&keyword_::List_) -> Self {
		Self {super_:set_::Item_::new2(&kw, &kws.equ_)}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn kw2__(&self) -> keyword_::ORI_ {self.super_.kw2__()}

	fn add__(&mut self, a:code_::List_) -> Result2_ {self.super_.add__(a)}
	fn add2__(&mut self, a:code_::List_) -> Result2_ {self.super_.add2__(a)}
	fn a__(&self) -> code_::ORL_ {self.super_.a__()}
	fn a2__(&self) -> code_::ORL_ {self.super_.a2__()}

	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, _ret:&mut result_::List_) -> Result2_ {
		let funcs = match as_mut_ref__!(q).obj_mut__(0) {
			Some(o) => o,
			None => return result2_::err2__(dl_::NAME_)
		};
		let mut funcs = as_mut_ref__!(funcs);
		let funcs = funcs.downcast_mut::<funcs_::List_>().unwrap();
		
		let mut names = result_::List_::new();
		let mut vals = result_::List_::new();
		self.super_.hello2_1__(&mut names, &mut vals, gd, q.clone(), w.clone(), wm)?;
		let names = names.to_vec__();
		let vals = vals.to_vec__();
		match funcs.add__(&vals) {
			Ok(i) => {
				qv_::val2__(&names[0], result_::oi__(Box::new(i)), false, false, q.clone(), w);
			}
			Err(s) => return result2_::err__(s)
		};
		ok__()
	}
}
