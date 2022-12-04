#[macro_use]
extern crate tantivy;

mod download;
mod indexer;
mod tatoeba;
mod utils;

use indexer::build_index;

use tatoeba::get_sentences;
use tokio::fs::create_dir_all;

pub static TEMP_DIR: &str = ".tmp";

#[tokio::main]
async fn main() {
    create_dir_all(TEMP_DIR)
        .await
        .unwrap_or_else(|err| println!("Couldn't create temp directory: {}", err.to_string()));

    let sentences = get_sentences().await.unwrap();

    build_index(sentences).unwrap();

    // let reader = index
    //     .reader_builder()
    //     .reload_policy(ReloadPolicy::OnCommit)
    //     .try_into()
    //     .unwrap();

    // let searcher = reader.searcher();
    // let query_parser = QueryParser::for_index(&index, vec![schema.get_field("sentence").unwrap()]);
    // let query = query_parser.parse_query("sea whale").unwrap();
    // let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

    // for (_score, doc_address) in top_docs {
    //     let retrieved_doc = searcher.doc(doc_address).unwrap();
    //     println!("{}", schema.to_json(&retrieved_doc));
    // }
}
