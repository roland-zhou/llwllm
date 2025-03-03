use anyhow::Result;
use serde_json::json;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    // 加载环境变量
    dotenv::dotenv().ok();
    
    // 从环境变量获取 API key
    let api_key = std::env::var("OPENAI_API_KEY")
        .expect("OPENAI_API_KEY not found in environment variables");

    println!("欢迎使用中英翻译器！");
    println!("请输入要翻译的文本（输入 'quit' 退出）：");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        
        if input.eq_ignore_ascii_case("quit") {
            println!("感谢使用，再见！");
            break;
        }

        if input.is_empty() {
            continue;
        }

        match translate(input, &api_key).await {
            Ok((chinese, english)) => {
                println!("\n中文：{}", chinese);
                println!("英文：{}\n", english);
            }
            Err(e) => println!("翻译出错：{}", e),
        }
    }

    Ok(())
}

async fn translate(text: &str, api_key: &str) -> Result<(String, String)> {
    let client = reqwest::Client::new();
    
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "gpt-4o",
            "messages": [
                {
                    "role": "system",
                    "content": "你是一个翻译助手。请将用户输入的文本翻译成中文和英文。请使用以下格式返回：\n中文：[中文翻译]\n英文：[英文翻译]"
                },
                {
                    "role": "user",
                    "content": text
                }
            ]
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let content = response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

    // 解析返回的内容
    let mut chinese = String::new();
    let mut english = String::new();

    for line in content.lines() {
        if line.starts_with("中文：") {
            chinese = line.trim_start_matches("中文：").trim().to_string();
        } else if line.starts_with("英文：") {
            english = line.trim_start_matches("英文：").trim().to_string();
        }
    }

    Ok((chinese, english))
}
