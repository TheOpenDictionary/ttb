use std::{collections::HashMap, error::Error, fs};

use indicatif::{ProgressBar, ProgressStyle};

use rusqlite::Connection;

use super::tatoeba::Sentence;

pub fn build_index(
    sentences: HashMap<u64, Sentence>,
    links: HashMap<u64, Vec<u64>>,
) -> Result<(), Box<dyn Error>> {
    if fs::metadata("tatoeba.db").is_ok() {
        fs::remove_file("tatoeba.db")?;
    }

    let conn = Connection::open("tatoeba.db")?;
    // FOREIGN KEY (sentence_id) REFERENCES sentences(id),
    // FOREIGN KEY (translation_id) REFERENCES sentences(id)
    conn.execute_batch(
        "CREATE TABLE sentences (
          id        INTEGER PRIMARY KEY,
          language  TEXT NOT NULL,
          text      TEXT NOT NULL
        ); 

        CREATE TABLE translations (
          sentence_id INTEGER,
          translation_id INTEGER
        );
        
        CREATE INDEX language_idx ON sentences(language);
        
        CREATE VIRTUAL TABLE sentences_fts USING fts5(text, language, content=sentences, content_rowid=id);",
    )?;

    let progress = ProgressBar::new_spinner();

    progress.set_style(
        ProgressStyle::default_bar().template("{spinner} {human_pos} sentences indexed")?,
    );

    for s in sentences {
        conn.execute(
            "INSERT INTO sentences (id, language, text) VALUES ($1, $2, $3);",
            (s.0, s.1.language, s.1.text),
        )?;

        for l in links.get(&(s.0)) {
            for t in l {
                conn.execute(
                    "INSERT INTO translations (sentence_id, translation_id) VALUES ($1, $2);",
                    (s.0, t),
                )?;
            }
        }

        progress.inc(1);
    }

    progress.finish();

    Ok(())
}
