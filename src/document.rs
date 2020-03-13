
pub struct HDoc
{
	pub doc:web_sys::Document
}

pub fn get_global_document() -> HDoc
{
	let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

	return HDoc{doc:document};
}
