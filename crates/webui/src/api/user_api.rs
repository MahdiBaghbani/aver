use reqwasm::http;

use super::types::{ErrorResponse, RegisterUserResponseData, User};

pub async fn api_register_user(user_data: &str) -> Result<RegisterUserResponseData, String> {
    let response = match http::Request::post("/api/users/register")
        .header("Content-Type", "application/json")
        .body(user_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        return if let Ok(error_response) = error_response {
            Err(error_response.message)
        } else {
            Err(format!("API error: {}", response.status()))
        };
    }

    let res_json = response.json::<RegisterUserResponseData>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_login_user(credentials: &str) -> Result<(), String> {
    let response = match http::Request::post("/api/auth/login")
        .header("Content-Type", "application/json")
        .credentials(http::RequestCredentials::Include)
        .body(credentials)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        return if let Ok(error_response) = error_response {
            Err(error_response.message)
        } else {
            Err(format!("API error: {}", response.status()))
        };
    }

    Ok(())
}

pub async fn api_user_info() -> Result<User, String> {
    let response = match http::Request::get("/api/users/me")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        return if let Ok(error_response) = error_response {
            Err(error_response.message)
        } else {
            Err(format!("API error: {}", response.status()))
        };
    }

    let res_json = response.json::<User>().await;
    match res_json {
        Ok(data) => Ok(data),
        Err(_) => Err("Failed to parse response".to_string()),
    }
}

pub async fn api_logout_user() -> Result<(), String> {
    let response = match http::Request::get("http://localhost:8000/api/auth/logout")
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
    {
        Ok(res) => res,
        Err(_) => return Err("Failed to make request".to_string()),
    };

    if response.status() != 200 {
        let error_response = response.json::<ErrorResponse>().await;
        if let Ok(error_response) = error_response {
            return Err(error_response.message);
        } else {
            return Err(format!("API error: {}", response.status()));
        }
    }

    Ok(())
}
