use crate::hengine;
use web_sys::{Element};
use wasm_bindgen::prelude::*;
use crate::hengine::HElement;
use crate::hengine::hdoc::HDoc;

pub struct HText
{
	pub element:web_sys::Element
}

impl HText
{
	pub fn create() -> HText
	{
		let doc = hengine::hdoc::get_global_document();
		let native = doc.create_element("p").expect("Could not create element.");

		return HText{element:native};
	}

	pub fn set_text(&self,html:&str)
	{
		self.element.set_inner_html(html);
	}

	pub fn from_selector(selector:&str) -> Option<HText>
	{
		let doc = hengine::hdoc::get_global_document();
		let selectResult = doc.query_element(selector);
		match selectResult
		{
			Ok(val)=> match val { Some(v2)=>Option::Some(HText{element:v2}),None=>None},
			Err(_)=> None
		}
	}
}

impl HElement for HText
{
	fn add_children(&self,ot:Element)
	{
		self.element.append_child(&ot);
	}
}

impl From<HText> for Element
{
	fn from(ll: HText) -> Self
	{
		return ll.element;
	}
}
