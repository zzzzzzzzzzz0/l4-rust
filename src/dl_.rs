use zhscript2::u_::*;
use super::*;

pub const NAME_: &str = "函数集";

pub struct Item_ {
	super_:code_::Item1_,
	a_:code_::OL_,
}

impl Item_ {
	pub fn new(kw: keyword_::RI_) -> Self {
		Self {super_:code_::Item1_::new(&kw), a_:None}
	}
}

impl code_::Item_ for Item_ {
	fn kw__(&self) -> &keyword_::Item_ {self.super_.kw__()}
	fn add__(&mut self, a:code_::List_) -> Result2_ {
		if a.is_empty() {
			return result2_::qve__();
		}
		self.a_ = Some(a);
		ok__()
	}
	fn a__(&self) -> code_::ORL_ {t_::some__(&self.a_)}
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, _ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		t_::o__(&self.a_).hello__(gd, q.clone(), w, wm, &mut ret2)?;

		match funcs_::List_::new(&ret2.to_vec__()) {
			Ok(funcs) => {
				as_mut_ref__!(q).add_obj_mut__(Box::new(funcs));
				ok__()
			}
			Err(e) => result2_::err__(e)
		}
	}
}
