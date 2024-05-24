#[doc = "Token claims supported for the JWT"]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
    pub role: String,
}

#[derive(Debug)]
pub struct UserClaims {
    pub user_id: i64,
    pub role: String,
}
