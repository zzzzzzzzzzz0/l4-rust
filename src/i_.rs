use zhscript2::{u_::*, as_ref__, as_mut_ref__};
use super::*;

struct U11_ {}
impl pars_::U11_ for U11_ {
	fn u11__(&self, kw:keyword_::RI_, kws:&keyword_::List_, _codes:& code_::List_) -> code_::OI_ {
		if kw.s_ == dl_::NAME_ {
			return code_::oi__(dl_::Item_::new(kw))
		}
		if kw.s_ == func_::NAME_ {
			return code_::oi__(func_::Item_::new(kw, kws))
		}
		if kw.s_ == call_::NAME_ {
			return code_::oi__(call_::Item_::new(kw))
		}
		None
	}
}

#[no_mangle]
pub extern fn i__(a:&[String], ret:&mut Vec<String>) -> i32 {
	let w = world_::t__(World_::new());
	{
		let kws = &mut as_mut_ref__!(w).kws_;
		kws.add3__(  dl_::NAME_, keyword_::Id_::U11, keyword_::Grp_ {print_:true, ..Default::default()});
		kws.add3__(func_::NAME_, keyword_::Id_::U11, keyword_::Grp_ {print_:true, set_:true, ..Default::default()});
		kws.add3__(call_::NAME_, keyword_::Id_::U11, keyword_::Grp_ {print_:true, ..Default::default()});
	}
	as_mut_ref__!(w).pars_.u11_ = Some(Rc_::new(U11_ {}));
	
	let mut wm:WorldMut_ = Default::default();
	let w2 = as_ref__!(w);
	let mut q = Qv_::new2(Some(w2.clone().top_q_));
	let mut ret2 = result_::List_::new();
	let mut v = vec![];
	for i in a {
		v.push(i.to_string())
	}
	let ret3 = w2.clone().hello3__(&mut v.into_iter(), true, true, false, &mut q, &mut wm, &mut ret2);
	*ret = ret2.to_vec__();
	w2.clone().ret__(ret3)
}
