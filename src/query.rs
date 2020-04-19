use crate::schema::{Schema, TypeMetadata};
use http::{Request, Uri};

#[derive(Debug, Default, Clone)]
pub struct SelectionSet<'a> {
    pub nodes: Vec<Selection<'a>>,
}

#[derive(Debug, Clone)]
pub struct Selection<'a> {
    pub name: &'a str,
    pub path_segments: Vec<&'a str>,
    pub selection_set: SelectionSet<'a>,
}

impl<'a> SelectionSet<'a> {
    fn new(
        type_metadata: &'a TypeMetadata,
        schema: &'a Schema<'a>,
        path_segments: Vec<&'a str>,
    ) -> Self {
        SelectionSet {
            nodes: match type_metadata.fields(schema) {
                Some(fields) => fields
                    .into_iter()
                    .map(|field| {
                        let mut path_segments = path_segments.clone();
                        let field_type_metadata = schema.type_metadata(&field.field_type);

                        if let TypeMetadata::Resource { .. } = field_type_metadata {
                            path_segments.push(&field.name);
                        }

                        Selection {
                            name: &field.name,
                            path_segments: path_segments.clone(),
                            selection_set: SelectionSet::new(
                                field_type_metadata,
                                schema,
                                path_segments.clone(),
                            ),
                        }
                    })
                    .collect(),
                None => vec![],
            },
        }
    }

    fn first_level(
        type_metadata: &'a TypeMetadata,
        schema: &'a Schema<'a>,
        path_segments: Vec<&'a str>,
    ) -> Self {
        SelectionSet {
            nodes: match type_metadata.fields(schema) {
                Some(fields) => fields
                    .into_iter()
                    .map(|field| {
                        let mut path_segments = path_segments.clone();
                        let field_type_metadata = schema.type_metadata(&field.field_type);

                        if let TypeMetadata::Resource { .. } = field_type_metadata {
                            path_segments.push(&field.name);
                        }

                        Selection {
                            name: &field.name,
                            path_segments: path_segments.clone(),
                            selection_set: SelectionSet::empty(),
                        }
                    })
                    .collect(),
                None => vec![],
            },
        }
    }

    fn empty() -> Self {
        SelectionSet { nodes: vec![] }
    }
}

pub fn parse<'a, T>(request: Request<T>, schema: &'a Schema<'a>) -> Result<Selection<'a>, String> {
    let root_selection_set = SelectionSet::new(schema.type_metadata("Root"), schema, vec![]);

    Ok(Selection {
        name: "root",
        path_segments: vec![],
        selection_set: root_selection_set,
    })
}

/*
for header in request.headers().get_all("Preload") {
    match header.to_str() {
        Ok(pointer) => {
            let tokens = parse_json_pointer_extended(pointer).unwrap();

            // selection_set.add_fields(&mut tokens, root_type, schema);
        }
        Err(error) => return Err(format!("Error parsing Preload header : '{}'", error)),
    };
}

fn parse_json_pointer_extended(pointer: &str) -> Result<Vec<String>, String> {
    if pointer == "" {
        return Ok(vec![]);
    }

    if !pointer.starts_with('/') {
        return Err(format!(
            "Error parsing Preload header '{}' : pointer does't start with '/'",
            pointer
        ));
    }

    Ok(pointer
        .split('/')
        .skip(1)
        .map(|escaped_token| {
            escaped_token
                .replace("~1", "/")
                .replace("~0", "~")
                .replace("~2", "*")
        })
        .collect::<Vec<String>>())
}
*/
