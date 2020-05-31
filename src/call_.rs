use zhscript2::u_::*;
use super::*;

pub const NAME_: &str = "调用";

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
	fn hello__(&self, gd:code_::Opt_, q:qv_::T_, w:world_::T_, wm:&mut WorldMut_, ret:&mut result_::List_) -> Result2_ {
		let mut ret2 = result_::List_::new();
		let mut err = String::new();
		t_::o__(&self.a_).hello__(gd, q.clone(), w.clone(), wm, &mut ret2)?;
		if ret2.obj__(0, |fi:&funcs_::RI_| {
			let mut argv2 = vec![];
			let mut start = 1;
			while start < ret2.len() {
				let i = as_ref__!(ret2[start]);
				if i.dunhao__() {
					start += 1;
					break
				}
				let mut s = String::new();
				i.s__(&mut s);
				if let Err(e) = funcs_::Item_::pargs2__(&s, &mut argv2) {
					err = e;
					return false
				}
				start += 1
			}
			let args = ret2.to_vec5__(start);
			if let Err(e) = fi.call__(&argv2, &args, ret) {
				err = e;
				false
			} else {true}
		}) {return ok__()} else if err.is_empty() {
			err.push_str("第一个非");
			err.push_str(super::func_::NAME_);
			err.push_str("变量");
		}
		result2_::err__(err)
	}
}
