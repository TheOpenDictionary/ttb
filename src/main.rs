#[macro_use]
extern crate tantivy;

mod download;
mod indexer;
mod tatoeba;
mod utils;

use indexer::{build_index, create_schema};

use tantivy::{
    collector::{FilterCollector, TopDocs},
    query::{BooleanQuery, Occur, Query, QueryParser, TermQuery},
    schema::IndexRecordOption,
    Index, ReloadPolicy, Term,
};
use tatoeba::get_sentences;
use tokio::fs::create_dir_all;

pub static TEMP_DIR: &str = ".tmp";

#[tokio::main]
async fn main() {
    // create_dir_all(TEMP_DIR)
    //     .await
    //     .unwrap_or_else(|err| println!("Couldn't create temp directory: {}", err.to_string()));

    // let sentences = get_sentences().await.unwrap();

    // build_index(sentences).unwrap();

    let index = Index::open_in_dir(".tmp/idx").unwrap();
    let reader = index.reader_builder().try_into().unwrap();

    let schema = create_schema();
    let searcher = reader.searcher();
    let text = schema.get_field("text").unwrap();
    let language = schema.get_field("language").unwrap();

    let text_query: Box<dyn Query> = Box::new(TermQuery::new(
        Term::from_field_text(text, "bonjour"),
        IndexRecordOption::Basic,
    ));

    let lang_query: Box<dyn Query> = Box::new(TermQuery::new(
        Term::from_field_text(language, "fra"),
        IndexRecordOption::Basic,
    ));

    let queries = BooleanQuery::from(vec![(Occur::Must, text_query), (Occur::Must, lang_query)]);

    let top_docs = searcher
        .search(&queries, &TopDocs::with_limit(100))
        .unwrap();

    println!("{}", top_docs.len());
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        println!(
            "{}",
            retrieved_doc
                .get_first(schema.get_field("text").unwrap())
                .unwrap()
                .as_text()
                .unwrap()
        );
    }
}
