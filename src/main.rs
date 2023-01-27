use reqwest;
use serde::{Serialize, Deserialize};

const API_KEY: &str = "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";

#[derive(Serialize, Deserialize)]
pub struct TextRequest {
    pub model: String,
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: usize
}

impl TextRequest {
    pub fn new<Model, Prompt>(model: Model, prompt: Prompt, temperature: f32, max_tokens: usize) -> Self
    where Model: AsRef<str>, Prompt: AsRef<str> {
        let model = model.as_ref().to_string();
        let prompt = prompt.as_ref().to_string();
        Self { model, prompt, temperature, max_tokens }
    }
}

pub struct Client {
    client: reqwest::Client,
    api_key: String,
    organization: String
}

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

impl Client {
    pub fn new<APIKey: Into<String>, Organization: Into<String>>(api_key: APIKey, organization: Organization) -> Self {
        let client = reqwest::Client::new();
        let api_key = api_key.into();
        let organization = organization.into();
        Self { client, api_key, organization }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
    text: String
}

#[derive(Serialize, Deserialize)]
pub struct TextResponse {
    id: String,
    object: String,
    created: usize,
    model: String,
    choices: Vec<Choice>
}

#[derive(Serialize, Deserialize)]
pub struct ImageRequest {
    pub prompt: String
}

impl ImageRequest {
    pub fn new<S: AsRef<str>>(prompt: S) -> Self {
        let prompt = prompt.as_ref().to_string();
        Self { prompt }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ImageURL {
    pub url: String
}

#[derive(Serialize, Deserialize)]
pub struct ImageResponse {
    pub data: Vec<ImageURL>
}

impl Client {
    pub async fn ask_with_max_tokens<S: AsRef<str>>(&self, question: S, max_tokens: usize) -> Result<String> {
        let response: TextResponse = self.client.post("https://api.openai.com/v1/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Organization", &self.organization)
            .json(&TextRequest::new("text-davinci-003", question, 1.0, max_tokens))
            .send().await?
            .json().await?;
        Ok(response.choices[0].text.clone())
    }

    pub async fn ask<S: AsRef<str>>(&self, question: S) -> Result<String> {
        self.ask_with_max_tokens(question, 1000).await
    }

    pub async fn generate_image<S: AsRef<str>>(&self, prompt: S) -> Result<String> {
        let response: ImageResponse = self.client.post("https://api.openai.com/v1/images/generations")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Organization", &self.organization)
            .json(&ImageRequest::new(prompt))
            .send().await?
            .json().await?;
        Ok(response.data[0].url.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u8
}

pub struct Conversation {
    client: Client,
    conversation: String
}

impl Conversation {
    pub fn new(client: Client) -> Self {
        let conversation = Default::default();
        Self { client, conversation }
    }

    pub async fn say<S: Into<String>>(&mut self, message: S) -> Result<String> {
        let message = message.into();
        self.conversation.push_str(&format!("I said: {}\n", message));
        self.conversation.push_str(&"You said: ");
        let message = self.client.ask(&self.conversation).await?;
        self.conversation.push_str(&format!("{}\n", message.trim_start()));
        Ok(message)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(API_KEY, "org-ckZwW3joAfeQxBihwHVktUzy");
    println!("{}", client.ask_with_max_tokens(
r#"01/26 - A. My name is Danilo. And today is 01/26.
01/26 - B. Nice to meet you Danilo.
01/27 - A. I am from a small city called Valença. Today is 01/27.
01/27 - B. Interesting.
01/27 - C. Hi, am Marina.
01/27 - B. Nice to meet you Marina.
01/27 - A. What did I say yesterday?
"#
        , 1000).await?);
    Ok(())
    // // println!("{}", client.generate_image("Artistic photography of a beautiful young chinese woman with makeup. Full body.").await?);
    // let mut conversation = Conversation::new(client);
    // conversation.say("Can you write me a Rust code that computes a delay DSP?").await?;
    // println!("{}", conversation.conversation);
    // Ok(())
}
