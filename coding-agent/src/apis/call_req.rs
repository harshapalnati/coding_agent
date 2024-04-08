use crate::models::generals::llm::{Message,ChatCompletion,APIResponse};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap,HeaderValue};
//call llm 
pub async fn call_gpt(messages:Vec<Message>) -> Result<String,Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    //Extract api key info
    let api_key = env::var("OPEN_AI_KEY").expect("key not found");
    let org_id = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG key not found");

    //endpoint
    let url:&str ="https://api.openai.com/v1/chat/completions";

    //creater header

    let mut header =HeaderMap::new();
    header.insert("authorization", HeaderValue::from_str(&format!("Bearer {}",api_key))
    .map_err(|e| -> Box<dyn std::error::Error +Send> {Box::new(e)})?
);

    //create api key header
    header.insert("authorization", HeaderValue::from_str(&format!("Bearer {}",api_key))
    .map_err(|e|->Box<dyn std::error::Error +Send> {Box::new(e)})?
);
    //create open AI org header
    header.insert("OpenAI-Organization", HeaderValue::from_str(org_id.as_str())
    .map_err(|e|->Box<dyn std::error::Error +Send>{Box::new(e)})?
);



    //create client
    let client=Client::builder()
    .default_headers(header)
    .build().unwrap();

    //create chatcompletion

    let chat_completion:ChatCompletion =ChatCompletion { 
        model: "gpt-4".to_string(),
        messages, 
        temperature:0.1
        };

    //trouble shooting
    let res_raw =client.post(url).json(&chat_completion).send().await.unwrap();

    dbg!(res_raw.text().await.unwrap());

    //extract API resposne
    let res:APIResponse = client
    .post(url)
    .json(&chat_completion)
    .send()
    .await
    .map_err(|e|->Box<dyn std::error::Error +Send> {Box::new(e)})?
    .json()
    .await
    .map_err(|e|->Box<dyn std::error::Error +Send> {Box::new(e)})?; 

    Ok(res.choices[0].message.content.clone())

}


#[cfg(test)]
mod tests{
    use super::*;

    #[tokio::test]
    async fn test_call_to_openai(){
        let message = Message{
            role:"user".to_string(),
            content:"Hi there , give me some repsonse this is for test".to_string()
        };

        let messages:Vec<Message> = vec![message];

        let res = call_gpt(messages).await;
        if let Ok(res_str) = res{
            dbg!(res_str);
            assert!(true)
        }else {
            assert!(false)
        }
    }
}