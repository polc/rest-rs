use crate::schema::{Schema, TypeMetadata};
use http::Request;
use std::collections::HashMap;

mod parser;

#[derive(Debug, Clone)]
pub enum Selection {
    Link,
    Scalar,
    Object(Vec<ObjectField>),
    List(ListItem, Box<Selection>),
}

#[derive(Debug, Clone)]
pub struct ObjectField {
    pub name: &'static str,
    pub selection: Selection,
}

#[derive(Debug, Clone, Copy)]
pub enum ListItem {
    All,
    Index(u32),
}

impl Selection {
    pub fn resource(type_id: &str, schema: &Schema) -> Self {
        let type_metadata = schema.type_metadata(type_id);

        match type_metadata {
            TypeMetadata::Scalar => Selection::Scalar,
            TypeMetadata::Object { fields } => Selection::Object(
                fields
                    .iter()
                    .map(|field| ObjectField {
                        name: field.name,
                        selection: Selection::resource(&field.type_id, schema),
                    })
                    .collect(),
            ),
            TypeMetadata::List { type_id } => Selection::List(
                ListItem::All,
                Box::new(Selection::resource(type_id, schema)),
            ),
            TypeMetadata::Resource { .. } => Selection::Link,
        }
    }
}

/*
impl Selection {
    pub fn empty() -> Self {
        SelectionSet { fields: vec![] }
    }

    pub fn parse_request<T>(&mut self, req: &Request<T>) -> () {
        let headers = req.headers().get_all("preload");
        let selectors: Vec<parser::Selector> = headers
            .iter()
            .flat_map(|header| header.to_str())
            .flat_map(|header| parser::parse_preload(header))
            .map(|(_, selector)| selector)
            .flatten()
            .collect();

        /**
            /field_1/field_12
            /field_1/field_13
            /field_2/field_21
            /field_2/10
        */
        println!("{:?}", selectors);
    }

    pub fn add_selector(&mut self, selectors: Vec<&[parser::Segment]>) -> SelectionSet {
        let mut map = HashMap::<&str, Vec<&[parser::Segment]>>::new();

        for selector in &selectors {
            if let Some((segment, rest)) = selector.split_first() {
                // map.entry(segment).or_default(vec![])
            }
        }

        SelectionSet { fields: vec![] }
    }
}
*/

/*
pub fn parse_headers(headers: &HeaderMap) -> Vec<Vec<String>> {
    match headers.get("preload") {
        Some(header) => match Parser::parse_list(header.as_bytes()) {
            Ok(list) => list
                .into_iter()
                .filter_map(|list_entry| match list_entry {
                    Item(item) => item.bare_item.as_str().map(|selector| {
                        selector
                            .split("/")
                            .filter(|field| field.len() > 0)
                            .map(|field| field.to_string())
                            .collect()
                    }),
                    _ => None,
                })
                .collect(),
            Err(_) => vec![],
        },
        None => vec![],
    }
}
*/
