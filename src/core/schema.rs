use once_cell::sync::Lazy;
use tantivy::schema::{Field, Schema, FAST, INDEXED, STORED, STRING, TEXT};

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_u64_field("id", FAST | INDEXED);
    schema_builder.add_text_field("text", TEXT | STORED);
    schema_builder.add_text_field("language", STRING);
    schema_builder.add_u64_field("length", FAST | INDEXED);
    schema_builder.add_json_field("translations", STRING);

    schema_builder.build()
}

pub const SCHEMA: Lazy<Schema> = Lazy::new(|| create_schema());
pub const FIELD_ID: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("id").unwrap());
pub const FIELD_TEXT: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("text").unwrap());
pub const FIELD_LANGUAGE: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("language").unwrap());
pub const FIELD_LENGTH: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("length").unwrap());
pub const FIELD_TRANSLATIONS: Lazy<Field> = Lazy::new(|| SCHEMA.get_field("translations").unwrap());
