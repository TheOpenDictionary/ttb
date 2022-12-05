use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, Occur, Query, TermQuery},
    schema::IndexRecordOption,
    Index, Term,
};

use crate::core::indexer::create_schema;

pub fn search() {
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
