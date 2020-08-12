use crate::schema::{Schema, TypeMetadata};

#[derive(Debug, Default, Clone)]
pub struct NodeSelection {
    pub name: &'static str,
    pub nodes: Vec<NodeSelection>,
}

impl NodeSelection {
    pub fn new(name: &'static str, type_metadata: &TypeMetadata, schema: &Schema) -> Self {
        NodeSelection {
            name,
            nodes: match type_metadata.fields(schema) {
                Some(fields) => fields
                    .iter()
                    .map(|field| {
                        let field_type_metadata = schema.type_metadata(&field.field_type);

                        NodeSelection::new(field.name, field_type_metadata, schema)
                    })
                    .collect(),
                None => vec![],
            },
        }
    }
}
