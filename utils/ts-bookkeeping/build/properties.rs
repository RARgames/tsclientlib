//! Access properties of a connection with the propertie structs from events.
use std::default::Default;

use heck::*;
use t4rust_derive::Template;
use tsproto_structs::book::*;
use tsproto_structs::messages_to_book::{self, MessagesToBookDeclarations};

use crate::events::{get_properties, get_property_name, get_rust_type};

#[derive(Template)]
#[TemplatePath = "build/Properties.tt"]
#[derive(Debug)]
pub struct Properties<'a>(
	&'a BookDeclarations,
	&'a MessagesToBookDeclarations<'a>,
);

impl Default for Properties<'static> {
	fn default() -> Self { Properties(&DATA, &messages_to_book::DATA) }
}

fn get_ids(struc: &Struct) -> String {
	let mut res = String::new();
	for i in 0..struc.id.len() {
		//let p = id.find_property(structs);
		if !res.is_empty() {
			res.push_str(", ");
		}
		//res.push_str(&p.get_rust_type());
		res.push_str(&format!("s{}", i));
	}
	res
}

fn get_ids2(structs: &[Struct], struc: &Struct) -> String {
	let mut res = String::new();
	for (i, id) in struc.id.iter().enumerate() {
		let p = id.find_property(structs);
		if !res.is_empty() {
			res.push_str(", ");
		}
		if p.type_s != "str" {
			res.push('*');
		}
		res.push_str(&format!("s{}", i));
	}
	res
}
