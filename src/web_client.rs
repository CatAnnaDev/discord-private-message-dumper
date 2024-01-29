use reqwest::{Client, header};

pub struct Token {
    pub token: String,
}

impl Token {

    pub async fn http(client: &Client, url: String) -> Option<String> {
        let x = client.get(url).send().await.unwrap().text().await.unwrap();
        Some(x)
    }

    pub fn new_web_client(self) -> Client {
        let webclient = reqwest::ClientBuilder::new();
        let mut headers = header::HeaderMap::new();
        headers.insert("authorization", header::HeaderValue::from_str(self.token.as_str()).unwrap());
        webclient.default_headers(headers).build().unwrap()
    }
}