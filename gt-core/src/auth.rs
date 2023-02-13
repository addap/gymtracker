use anyhow::{anyhow, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;
use std::{collections::HashMap, num::ParseIntError};

use crate::models::{AuthToken, UserAuth};

pub fn verify_token(secret: &str, token: &AuthToken) -> Result<i32> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).map_err(|e| anyhow!(e))?;

    let claims: HashMap<String, String> = token.verify_with_key(&key).map_err(|e| anyhow!(e))?;

    let user_id = claims.get("sub").ok_or(anyhow!("Malformed JWT"))?;
    let user_id: i32 = user_id.parse().map_err(|e: ParseIntError| anyhow!(e))?;

    Ok(user_id)
}

pub fn get_claims_unverified(token: &AuthToken) -> Result<HashMap<String, String>> {
    let parsed: Token<jwt::Header, HashMap<String, String>, _> =
        Token::parse_unverified(&token).map_err(|e| anyhow!(e))?;

    let claims: &HashMap<String, String> = parsed.claims();
    Ok(claims.clone())
}

pub fn create_token(secret: &str, user: UserAuth) -> Result<AuthToken> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).map_err(|e| anyhow!(e))?;

    let mut claims = HashMap::new();
    claims.insert("sub", user.id.to_string());
    claims.insert("name", user.username);
    claims.insert("adm", user.is_superuser.to_string());
    claims.insert("iat", Utc::now().naive_utc().to_string());

    let token = claims.sign_with_key(&key).map_err(|e| anyhow!(e))?;

    Ok(token.into())
}
