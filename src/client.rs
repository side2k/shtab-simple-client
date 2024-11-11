use crate::user_profile::{AuthCredentials, AuthResponse, UserProfile};
use reqwest::header::HeaderMap;
use reqwest::{Client as HTTPClient, RequestBuilder, Response, StatusCode};
use serde::ser::Serialize;

pub struct Client {
    client: HTTPClient,
}

type Query = Vec<(String, String)>;

impl Client {
    pub fn new(api_token: Option<String>) -> Self {
        let mut default_headers = HeaderMap::new();
        if let Some(api_token) = api_token {
            default_headers.insert(
                "Authorization",
                format!("Token {}", api_token).parse().unwrap(),
            );
        }

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

    async fn post<T: Serialize>(&self, path: &str, data: T) -> Result<Response, String> {
        let request = self
            .get_request_builder(reqwest::Method::POST, path, None)
            .header("Content-Type", "application/json")
            .json::<T>(&data)
            .build()
            .unwrap();
        println!("POST {}", request.url());
        match self.client.execute(request).await {
            Ok(response) => Ok(response),
            Err(error) => Err(format!("{error}")),
        }
    }

    pub async fn get_profile(&self) -> Result<UserProfile, String> {
        match self.get("/en/api/users/profile/", None).await {
            Ok(response) => Ok(response.json().await.unwrap()),
            Err(error) => Err(error),
        }
    }

    pub async fn login(&self, username: String, password: String) -> Result<AuthResponse, String> {
        let credentials = AuthCredentials { username, password };
        print!("{:?}", credentials);
        let response = self
            .post("/en/api/users/user/login/", credentials)
            .await
            .unwrap();
        match response.status() {
            StatusCode::OK => Ok(response.json().await.unwrap()),
            _ => Err(format!("Got status code {}", response.status())),
        }
    }
}
