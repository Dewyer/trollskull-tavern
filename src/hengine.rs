use crate::document;

pub fn init_hengine()
{
	let window = web_sys::window().expect("No window");
	document::get_global_document();
}

