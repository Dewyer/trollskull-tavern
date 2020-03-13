pub mod hdoc;
pub mod htext;
use web_sys::{Element};

pub fn init_hengine()
{
	let window = web_sys::window().expect("No window");
	hdoc::get_global_document();
}

trait HElement
{
	fn add_children(&self,ot:Element);
}

