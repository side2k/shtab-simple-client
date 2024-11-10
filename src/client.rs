use crate::user_profile::UserProfile;
use reqwest::header::HeaderMap;
use reqwest::{Client as HTTPClient, RequestBuilder, Response, StatusCode};

pub struct Client {
    client: HTTPClient,
}

type Query = Vec<(String, String)>;

impl Client {
    pub fn new(api_token: String) -> Self {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "Authorization",
            format!("Token {}", api_token).parse().unwrap(),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .unwrap();
        Client { client }
    }

    fn get_request_builder(
        &self,
        method: reqwest::Method,
        path: &str,
        query: Option<Query>,
    ) -> RequestBuilder {
        let base_url = "https://my.shtab.app";
        let url = format!("{base_url}{path}");
        let mut request_builder = self.client.request(method, url);
        if let Some(query) = query {
            request_builder = request_builder.query(&query);
        }
        request_builder
    }

    async fn get(&self, path: &str, query: Option<Query>) -> Result<Response, String> {
        let request = self
            .get_request_builder(reqwest::Method::GET, path, query)
            .build()
            .unwrap();
        println!("GET {}", request.url());
        match self.client.execute(request).await {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response),
                _ => Err(format!("Got status code {}", response.status())),
            },
            Err(error) => Err(format!("{error}")),
        }
    }

    pub async fn get_profile(&self) -> Result<UserProfile, String> {
        match self.get("/en/api/users/profile/", None).await {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }
}
