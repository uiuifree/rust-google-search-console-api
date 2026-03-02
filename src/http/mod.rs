use crate::error::GoogleApiError;
use serde_json::json;
use std::fmt::Debug;

#[derive(Default, Debug)]
pub(crate) struct HttpClient {}

impl HttpClient {
    pub async fn get<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
    where
        T: for<'de> serde::Deserialize<'de>,
        U: serde::Serialize + std::fmt::Debug,
    {
        let mut request = reqwest::Client::new()
            .get(url)
            .header("Authorization", format!("Bearer {}", token));

        let q = json!(params).to_string();
        if !q.is_empty() && q != "{}" {
            request = request.json(&q);
        } else {
            request = request.header("Accept", "application/json");
        }

        let response = request
            .send()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;

        let status = response.status().as_u16();
        let value = response
            .text()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;

        if !(200..300).contains(&status) {
            return Err(GoogleApiError::Api {
                status,
                message: value,
            });
        }

        serde_json::from_str(&value).map_err(|_| GoogleApiError::JsonParse(value))
    }

    pub async fn post<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
    where
        T: for<'de> serde::Deserialize<'de>,
        U: serde::Serialize + std::fmt::Debug,
    {
        let mut request = reqwest::Client::new().post(url);

        if !token.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request
            .json(&json!(params))
            .send()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;

        let status = response.status().as_u16();
        let value = response
            .text()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;

        if !(200..300).contains(&status) {
            return Err(GoogleApiError::Api {
                status,
                message: value,
            });
        }

        serde_json::from_str(&value).map_err(|_| GoogleApiError::JsonParse(value))
    }

    pub async fn put<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
    where
        T: for<'de> serde::Deserialize<'de>,
        U: serde::Serialize + std::fmt::Debug,
    {
        let response = reqwest::Client::new()
            .put(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&json!(params))
            .send()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;

        let status = response.status().as_u16();
        let value = response
            .text()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;

        if !(200..300).contains(&status) {
            return Err(GoogleApiError::Api {
                status,
                message: value,
            });
        }

        let value = if value.is_empty() {
            "{}".to_string()
        } else {
            value
        };

        serde_json::from_str(&value).map_err(|_| GoogleApiError::JsonParse(value))
    }

    pub async fn delete<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
    where
        T: for<'de> serde::Deserialize<'de>,
        U: serde::Serialize + std::fmt::Debug,
    {
        let response = reqwest::Client::new()
            .delete(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&json!(params))
            .send()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;

        let status = response.status().as_u16();
        let value = response
            .text()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;

        if !(200..300).contains(&status) {
            return Err(GoogleApiError::Api {
                status,
                message: value,
            });
        }

        let value = if value.is_empty() {
            "{}".to_string()
        } else {
            value
        };

        serde_json::from_str(&value).map_err(|_| GoogleApiError::JsonParse(value))
    }
}
