pub mod utils;
pub mod hengine;
extern crate web_sys;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::{thread, time};
use crate::hengine::htext::HText;
use js_sys::{Array, Date};
use wasm_bindgen::JsCast;

const SLEEP_MILIS:i32 = 400;

pub fn ticker(cc:i32,hlel:HText)
{
	let ht = hengine::htext::HText::from_selector("body p");
	match ht
	{
		Some(text)=>
		{
			text.set_text(&cc.to_string());
		},
		None => {

		}
	};
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue>
{
	let window = web_sys::window().expect("no global `window` exists");

	let ht = hengine::htext::HText::create();
	ht.set_text("Hello world");
	hengine::hdoc::get_global_document().append_to_body(&ht.element);

	let mut cc = 0;
	let closure = wasm_bindgen::closure::Closure::wrap(Box::new(|| {ticker(cc,ht)}) as Box<dyn Fn()>);
	//let closure = Closure::new(||ticker(cc,ht));

	window.set_interval_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(),SLEEP_MILIS)?;

	Ok(())
}
