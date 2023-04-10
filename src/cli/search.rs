use std::error::Error;

use crate::core::schema::{FIELD_TEXT, FIELD_TRANSLATIONS};

pub async fn search(
    query: &String,
    language: &Option<String>,
    translation: &Option<String>,
    limit: &usize,
) -> Result<(), Box<dyn Error>> {
    let results = crate::core::search::search(query.as_str(), language, translation, limit).await?;

    for (idx, result) in results.iter().enumerate() {
        let text = result.get_first(*FIELD_TEXT).unwrap().as_text().unwrap();
        let json = result
            .get_first(*FIELD_TRANSLATIONS)
            .unwrap()
            .as_json()
            .unwrap();

        let trans = json.get("fra").map(|r| r.as_u64());

        println!("{:>2}. {} {:?}", idx + 1, text, trans);
    }

    Ok(())
}
