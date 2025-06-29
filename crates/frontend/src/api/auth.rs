use space_cms_shared::{AuthResponse, LoginRequest, RegisterRequest};

pub async fn login(data: LoginRequest) -> Result<AuthResponse, String> {
    let request = gloo_net::http::Request::post("/api/auth/login")
        .json(&data)
        .map_err(|e| format!("Failed to create request: {}", e))?;
    
    let response = request.send().await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let auth_response: AuthResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(auth_response)
    } else {
        Err("Login failed".to_string())
    }
}

pub async fn register(data: RegisterRequest) -> Result<AuthResponse, String> {
    let request = gloo_net::http::Request::post("/api/auth/register")
        .json(&data)
        .map_err(|e| format!("Failed to create request: {}", e))?;
    
    let response = request.send().await
        .map_err(|e| format!("Request failed: {}", e))?;

    if response.ok() {
        let auth_response: AuthResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(auth_response)
    } else {
        Err("Registration failed".to_string())
    }
}