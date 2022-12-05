use once_cell::sync::Lazy;
use tantivy::schema::{Field, Schema, STORED, STRING, TEXT};

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("text", TEXT | STORED);
    schema_builder.add_text_field("language", STRING);
    schema_builder.build()
}

pub const SCHEMA: Lazy<Schema> = Lazy::new(|| create_schema());
pub const FIELD_TEXT: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("text").unwrap());
pub const FIELD_LANGUAGE: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("language").unwrap());
