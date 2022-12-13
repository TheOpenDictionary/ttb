use std::error::Error;

use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, Occur, Query, QueryParser, TermQuery},
    schema::IndexRecordOption,
    DocId, Document, Index, SegmentReader, Term,
};

use crate::core::schema::{FIELD_LANGUAGE, FIELD_TEXT};

use super::{constants::INDEX_DIR, schema::FIELD_TRANSLATIONS};

pub async fn search(
    query: &str,
    language: &Option<String>,
    has_translation: &Option<String>,
    limit: &usize,
) -> Result<Vec<Document>, Box<dyn Error>> {
    let index = Index::open_in_dir(INDEX_DIR.as_path())
        .or(Err("No local Tatoeba store exists or it is corrupted!"))
        .unwrap();

    let reader = index.reader_builder().try_into()?;

    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(
        &index,
        vec![*FIELD_LANGUAGE, *FIELD_TEXT, *FIELD_TRANSLATIONS],
    );

    let query = query_parser.parse_query(
        format!(
            "{} AND language:{} AND translations.{}:*",
            query,
            language.as_ref().unwrap(),
            has_translation.as_ref().unwrap(),
        )
        .as_str(),
    )?;

    let top_docs = searcher.search(
        &*query,
        &TopDocs::with_limit(*limit).custom_score(move |sr: &SegmentReader| {
            // let popularity_reader = sr.().u64(popularity).unwrap();

            move |doc_id: DocId| doc_id
        }),
    )?;

    let docs = top_docs
        .iter()
        .map(|(_, d)| searcher.doc(*d).unwrap())
        .collect();

    Ok(docs)
}
