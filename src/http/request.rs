use super::error::Error;
use crate::http::error::{ReqwestError, SerdeJsonError};
use lazy_static::lazy_static;
#[cfg(not(target_arch = "wasm32"))]
use reqwest::{Client, StatusCode};
#[cfg(target_arch = "wasm32")]
use reqwest_wasm::{Client, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, warn};

lazy_static! {
    pub static ref CLIENT: Client = Client::new();
}

pub async fn post_json_no_serde<S>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    data: &S,
) -> Result<(), Error>
where
    S: Serialize + ?Sized,
{
    let mut request = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&data);
    if let Some(bearer_token) = bearer_token {
        request = request.header("Authorization", format!("Bearer {}", bearer_token));
    }
    let response = match request.send().await {
        Ok(response) => response,
        Err(err) => {
            warn!("{}", err);
            return Err(Error::Send(ReqwestError(err)));
        }
    };
    return match response.status() {
        status if status == StatusCode::OK => Ok(()),
        other => Err(Error::InvalidResponse(
            other,
            response
                .text()
                .await
                .unwrap_or("Response was not text".into()),
        )),
    };
}

///json
pub async fn post_json<S, T>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    data: &S,
    expected_response_code: Option<StatusCode>,
) -> Result<T, Error>
where
    S: Serialize + ?Sized,
    T: DeserializeOwned,
{
    post_json_debug(
        url,
        client,
        bearer_token,
        data,
        expected_response_code,
        false,
    )
    .await
}

///json
///debug_text_output refers specifically to when deserialization fails
pub async fn post_json_debug<S, T>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    data: &S,
    expected_response_code: Option<StatusCode>,
    debug_text_output: bool,
) -> Result<T, Error>
where
    S: Serialize + ?Sized,
    T: DeserializeOwned,
{
    let expected_response_code = expected_response_code.unwrap_or(StatusCode::OK);
    let mut request = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&data);
    if let Some(bearer_token) = bearer_token {
        request = request.header("Authorization", format!("Bearer {}", bearer_token));
    }
    let response = match request.send().await {
        Ok(response) => response,
        Err(err) => {
            warn!("{}", err);
            return Err(Error::Send(ReqwestError(err)));
        }
    };
    return match response.status() {
        status if status == expected_response_code => {
            let response_text: String = match response.text().await {
                Ok(response_text) => response_text,
                Err(err) => {
                    warn!("{}", err);
                    return Err(Error::ConvertResponseToText(ReqwestError(err)));
                }
            };
            if debug_text_output {
                debug!("{}", response_text);
            }
            let data: T = match serde_json::from_str::<T>(&response_text) {
                Ok(data) => data,
                Err(err) => {
                    warn!("{}", err);
                    return Err(Error::DeserializeTypeFromText(SerdeJsonError(err)));
                }
            };
            Ok(data)
        }
        other => Err(Error::InvalidResponse(
            other,
            response
                .text()
                .await
                .unwrap_or("Response was not text".into()),
        )),
    };
}

///x-www-form-urlencoded
pub async fn post_x_form_urlencoded<P, T>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    params: &P,
    expected_response_code: Option<StatusCode>,
) -> Result<T, Error>
where
    P: Serialize + ?Sized,
    T: DeserializeOwned,
{
    post_x_form_urlencoded_debug(
        url,
        client,
        bearer_token,
        params,
        expected_response_code,
        false,
    )
    .await
}

///x-www-form-urlencoded
///debug_text_output refers specifically to when deserialization fails
pub async fn post_x_form_urlencoded_debug<P, T>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    params: &P,
    expected_response_code: Option<StatusCode>,
    debug_text_output: bool,
) -> Result<T, Error>
where
    P: Serialize + ?Sized,
    T: DeserializeOwned,
{
    let expected_response_code = expected_response_code.unwrap_or(StatusCode::OK);
    let mut request = client
        .post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params);
    if let Some(bearer_token) = bearer_token {
        request = request.header("Authorization", format!("Bearer {}", bearer_token));
    }
    let response = match request.send().await {
        Ok(response) => response,
        Err(err) => {
            warn!("{}", err);
            return Err(Error::Send(ReqwestError(err)));
        }
    };
    return match response.status() {
        status if status == expected_response_code => {
            let response_text: String = match response.text().await {
                Ok(response_text) => response_text,
                Err(err) => {
                    warn!("{}", err);
                    return Err(Error::ConvertResponseToText(ReqwestError(err)));
                }
            };
            if debug_text_output {
                debug!("{}", response_text);
            }
            let data: T = match serde_json::from_str::<T>(&response_text) {
                Ok(data) => data,
                Err(err) => {
                    warn!("{}", err);
                    return Err(Error::DeserializeTypeFromText(SerdeJsonError(err)));
                }
            };
            Ok(data)
        }
        other => Err(Error::InvalidResponse(
            other,
            response
                .text()
                .await
                .unwrap_or("Response was not text".into()),
        )),
    };
}

pub async fn get<T>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    origin: Option<&str>,
) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    get_debug(url, client, bearer_token, origin, false).await
}

///debug_text_output refers specifically to when deserialization fails
pub async fn get_debug<T>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    origin: Option<&str>,
    debug_text_output: bool,
) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let mut request = client.get(url).header("accept", "application/json");
    if let Some(origin) = origin {
        request = request.header("Origin", origin);
    }
    if let Some(bearer_token) = bearer_token {
        request = request.header("Authorization", format!("Bearer {}", bearer_token));
    }

    let response = match request.send().await {
        Ok(response) => response,
        Err(err) => {
            warn!("{}", err);
            return Err(Error::Send(ReqwestError(err)));
        }
    };
    return match response.status() {
        StatusCode::OK => {
            let response_text: String = match response.text().await {
                Ok(response_text) => response_text,
                Err(err) => {
                    warn!("{}", err);
                    return Err(Error::ConvertResponseToText(ReqwestError(err)));
                }
            };
            if debug_text_output {
                debug!("{}", response_text);
            }
            let data: T = match serde_json::from_str::<T>(&response_text) {
                Ok(data) => data,
                Err(err) => {
                    warn!("{}", err);
                    return Err(Error::DeserializeTypeFromText(SerdeJsonError(err)));
                }
            };
            Ok(data)
        }
        other => Err(Error::InvalidResponse(
            other,
            response
                .text()
                .await
                .unwrap_or("Response was not text".into()),
        )),
    };
}

pub async fn get_query_params<T, P>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    params: &P,
) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: Serialize + ?Sized,
{
    get_query_params_debug(url, client, bearer_token, params, false).await
}

///debug_text_output refers specifically to when deserialization fails
pub async fn get_query_params_debug<T, P>(
    url: &str,
    client: Client,
    bearer_token: Option<&str>,
    params: &P,
    debug_text_output: bool,
) -> Result<T, Error>
where
    T: DeserializeOwned,
    P: Serialize + ?Sized,
{
    let mut request = client
        .get(url)
        .query(params)
        .header("accept", "application/json");
    if let Some(bearer_token) = bearer_token {
        request = request.header("Authorization", format!("Bearer {}", bearer_token));
    }

    let response = match request.send().await {
        Ok(response) => response,
        Err(err) => {
            warn!("{}", err);
            return Err(Error::Send(ReqwestError(err)));
        }
    };
    return match response.status() {
        StatusCode::OK => {
            let response_text: String = match response.text().await {
                Ok(response_text) => response_text,
                Err(err) => {
                    warn!("{}", err);
                    return Err(Error::ConvertResponseToText(ReqwestError(err)));
                }
            };
            if debug_text_output {
                debug!("{}", response_text);
            }
            let data: T = match serde_json::from_str::<T>(&response_text) {
                Ok(data) => data,
                Err(err) => {
                    warn!("{}", err);
                    return Err(Error::DeserializeTypeFromText(SerdeJsonError(err)));
                }
            };
            Ok(data)
        }
        other => Err(Error::InvalidResponse(
            other,
            response
                .text()
                .await
                .unwrap_or("Response was not text".into()),
        )),
    };
}
