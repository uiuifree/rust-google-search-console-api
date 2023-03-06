use std::fmt::{Debug};
use serde_json::{json};
use crate::error::GoogleApiError;


#[derive(Default, Debug)]
pub(crate) struct HttpClient {}


impl HttpClient {
    pub async fn get<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
        where
            T: for<'de> serde::Deserialize<'de>,
            U: serde::Serialize + std::fmt::Debug
    {
        let mut response = reqwest::Client::new()
            .get(format!("{}", url))
            .header("Authorization", format!("Bearer {}", token));

        let q = json!(params).to_string();
        if !q.is_empty() && q != "{}" {
            response = response.json(&q);
        } else {
            response = response.header("Accept", "application/json");
        }

        let response = response.send().await;


        if response.is_err() {
            return Err(GoogleApiError::Connection(response.err().unwrap().to_string()));
        }
        let response = response.unwrap();
        let status = response.status();
        let value = response.text().await;
        if !(200 <= status.as_u16() && status.as_u16() < 300) {
            return Err(GoogleApiError::JsonParse(value.unwrap().to_string()));
        }
        if value.is_err() {
            return Err(GoogleApiError::JsonParse(value.unwrap().to_string()));
        }
        let value = value.unwrap();
        let parse = serde_json::from_str(value.as_str());
        if parse.is_err() {
            return Err(GoogleApiError::JsonParse(value));
        }

        Ok(parse.unwrap())
    }
    pub async fn post<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
        where
            T: for<'de> serde::Deserialize<'de>,
            U: serde::Serialize + std::fmt::Debug
    {
        let mut response = reqwest::Client::new()
            .post(format!("{}", url));
        if !token.is_empty() {
            response = response.header("Authorization", format!("Bearer {}", token))
        }
        let response = response
            .json(&json!(params))
            .send()
            .await;

        println!("{}", url);
        println!("{}", &json!(params).to_string());

        if response.is_err() {
            return Err(GoogleApiError::Connection(response.err().unwrap().to_string()));
        }
        let response = response.unwrap();
        let status = response.status();
        let value = response.text().await;
        if status != 200 {
            println!("b");
            return Err(GoogleApiError::JsonParse(value.unwrap().to_string()));
        }
        if value.is_err() {
            println!("c");
            return Err(GoogleApiError::JsonParse(value.unwrap().to_string()));
        }
        let value = value.unwrap();
        let parse = serde_json::from_str(value.as_str());
        if parse.is_err() {
            return Err(GoogleApiError::JsonParse(value));
        }

        Ok(parse.unwrap())
    }

    pub async fn put<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
        where
            T: for<'de> serde::Deserialize<'de>,
            U: serde::Serialize + std::fmt::Debug
    {
        let response = reqwest::Client::new()
            .put(format!("{}", url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&json!(params))
            .send()
            .await;


        if response.is_err() {
            return Err(GoogleApiError::Connection(response.err().unwrap().to_string()));
        }
        let response = response.unwrap();
        let status = response.status().as_u16();
        let value = response.text().await;
        if !(200 <= status && status < 300) {
            return Err(GoogleApiError::JsonParse(value.unwrap().to_string()));
        }
        if value.is_err() {
            return Err(GoogleApiError::JsonParse(value.unwrap().to_string()));
        }
        let mut value = value.unwrap();
        if value == "" {
            value = "{}".to_string();
        }
        let parse = serde_json::from_str(value.as_str());
        if parse.is_err() {
            return Err(GoogleApiError::JsonParse(value));
        }

        Ok(parse.unwrap())
    }
    pub async fn delete<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
        where
            T: for<'de> serde::Deserialize<'de>,
            U: serde::Serialize + std::fmt::Debug
    {
        let response = reqwest::Client::new()
            .delete(format!("{}", url))
            .header("Authorization", format!("Bearer {}", token))
            .json(&json!(params))
            .send()
            .await;


        if response.is_err() {
            return Err(GoogleApiError::Connection(response.err().unwrap().to_string()));
        }
        let response = response.unwrap();
        let status = response.status().as_u16();
        let value = response.text().await;

        if !(200 <= status && status < 300) {
            return Err(GoogleApiError::JsonParse(value.unwrap().to_string()));
        }
        if value.is_err() {
            return Err(GoogleApiError::JsonParse(value.unwrap().to_string()));
        }
        let mut value = value.unwrap();
        if value == "" {
            value = "{}".to_string();
        }
        let parse = serde_json::from_str(value.as_str());
        if parse.is_err() {
            return Err(GoogleApiError::JsonParse(value));
        }

        Ok(parse.unwrap())
    }
}