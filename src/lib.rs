use std::io::Error;

/**
 * Translates text using a google translate endpoint
 * text - the text to be translated
 * from - the language of the text
 * to - the language to translate to
 */
pub async fn translate(text: &str, from: &str, to: &str) -> Result<String, std::io::Error> {
    let url = format!("https://translate.googleapis.com/translate_a/single?client=gtx&sl={}&tl={}&dt=t&q={}", from, to, text);
    let resp = reqwest::get(url).await;
    if resp.is_err() {return Err(Error::new(std::io::ErrorKind::Other, "translate error"));}
    let resp = resp.unwrap().text().await;
    if resp.is_err() {return Err(Error::new(std::io::ErrorKind::Other, "translate error"));}
    let resp = resp.unwrap();
    let resp = resp.split("\"").nth(1);
    if resp.is_none(){return Err(Error::new(std::io::ErrorKind::Other, "translate error"));}
    Ok(resp.unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_translate() {
        let text = "Hello, world!";
        let from = "en";
        let to = "zh-CN";
        let resp = translate(text, from, to).await.unwrap();
        println!("{}", resp);
    }
}
