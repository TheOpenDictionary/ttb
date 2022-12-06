use std::error::Error;

use crate::core::schema::FIELD_TEXT;

pub async fn search(
    query: &String,
    language: &Option<String>,
    translation: &Option<String>,
    limit: &usize,
) -> Result<(), Box<dyn Error>> {
    let results = crate::core::search::search(query.as_str(), language, limit).await?;

    for (idx, result) in results.iter().enumerate() {
        let text = result.get_first(*FIELD_TEXT).unwrap().as_text().unwrap();

        println!("{:>2}. {}", idx + 1, text);
    }

    Ok(())
}
