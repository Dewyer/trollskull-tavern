use web_sys::{Element,HtmlElement};
use wasm_bindgen::prelude::*;

pub struct HDoc
{
	pub doc:web_sys::Document
}

impl HDoc
{
	pub fn create_element(&self,name:&str) -> Result<Element,JsValue>
	{
		return self.doc.create_element(name);
	}

	pub fn query_element(&self, selector:&str) -> Result<Option<Element>, JsValue>
	{
		return self.doc.query_selector(selector);
	}

	pub fn get_body(&self) -> Option<HtmlElement>
	{
		return self.doc.body();
	}

	pub fn append_to_body(&self,ot:&Element)
	{
		self.get_body().unwrap().append_child(ot);
	}
}

pub fn get_global_document() -> HDoc
{
	let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

	return HDoc{doc:document};
}
