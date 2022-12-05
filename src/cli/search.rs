use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, Occur, Query, TermQuery},
    schema::IndexRecordOption,
    Index, Term,
};

use crate::core::schema::{FIELD_LANGUAGE, FIELD_TEXT};

pub fn search() {
    let index = Index::open_in_dir(".tmp/idx").unwrap();
    let reader = index.reader_builder().try_into().unwrap();
    let searcher = reader.searcher();

    let text_query: Box<dyn Query> = Box::new(TermQuery::new(
        Term::from_field_text(*FIELD_TEXT, "bonjour"),
        IndexRecordOption::Basic,
    ));

    let lang_query: Box<dyn Query> = Box::new(TermQuery::new(
        Term::from_field_text(*FIELD_LANGUAGE, "fra"),
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
                .get_first(*FIELD_TEXT)
                .unwrap()
                .as_text()
                .unwrap()
        );
    }
}
