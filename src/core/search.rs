use std::error::Error;

use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, Occur, Query, TermQuery},
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

    let text_query: Box<dyn Query> = Box::new(TermQuery::new(
        Term::from_field_text(*FIELD_TEXT, query),
        IndexRecordOption::Basic,
    ));

    let mut queries_set = vec![(Occur::Must, text_query)];

    if let Some(lang) = language {
        let lang_query: Box<dyn Query> = Box::new(TermQuery::new(
            Term::from_field_text(*FIELD_LANGUAGE, lang.as_str()),
            IndexRecordOption::Basic,
        ));

        queries_set.push((Occur::Must, lang_query));
    }

    if let Some(translation) = has_translation {
        let trans_query: Box<dyn Query> = Box::new(TermQuery::new(
            Term::from_field_text(*FIELD_TRANSLATIONS, translation.as_str()),
            IndexRecordOption::Basic,
        ));

        queries_set.push((Occur::Must, trans_query));
    }

    let queries = BooleanQuery::from(queries_set);

    let top_docs = searcher.search(
        &queries,
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
