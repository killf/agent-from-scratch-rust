use serde::Serialize;

#[derive(Serialize)]
struct LLMRequest<'a> {
    model: &'a str,
    messages: Vec<LLMMessage>,
}

#[derive(Serialize)]
struct LLMMessage {}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[tokio::test]
    async fn test_glm() -> Result<(), Box<dyn std::error::Error>> {
        let base_url = "https://open.bigmodel.cn/api/paas/v4/chat/completions";
        let api_key = std::env::var("API_KEY")?;

        let resp = reqwest::Client::new()
            .post(base_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": "glm-4.7-flash",
                "messages":[
                    {
                        "role": "user",
                        "content": "请用一句话介绍一下Agent"
                    }
                ]
            }))
            .send()
            .await?;

        println!("{}", resp.text().await?);

        Ok(())
    }
}
