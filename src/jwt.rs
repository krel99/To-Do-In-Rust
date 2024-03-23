use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::Config;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwToken {
    pub user_id: i32,
    pub exp: usize,
}

impl JwToken {
    pub fn get_key() -> String {
        let config = Config::new();
        let key_str = config.map.get("SECRET_KEY").unwrap().as_str().unwrap();
        return key_str.to_owned();
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwToken::get_key().as_ref());
        let token = encode(&Header::default(), &self, &key).unwrap();
        return token;
    }

    pub fn new(user_id: i32) -> Self {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES").unwrap().as_i64().unwrap();
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::minutes(minutes))
            .expect("valid timestamp")
            .timestamp();
        return JwToken {
            user_id,
            exp: expiration as usize,
        };
    }

    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwToken::get_key().as_ref());
        let token_result = decode::<JwToken>(&token.as_str(), &key, &Validation::default());
        match token_result {
            Ok(data) => return Ok(data.claims),
            Err(error) => {
                let message = format!("{}", error);
                return Err(message);
            }
        }
    }
}

impl FromRequest for JwToken {
    type Error = Error;
    type Future = Ready<Result<JwToken, Error>>;

    /// This gets fired when the JwToken is attached to a request. It fires before the request hits the view.
    /// # Arguments
    /// The arguments are needed in order for the impl of FromRequest to work.
    ///
    /// * req (&HttpRequest): the request that the token is going to be extracted from
    /// * _ (Payload): the payload stream (not used in this function but is needed)
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        match req.headers().get("token") {
            Some(data) => {
                let raw_token = data.to_str().unwrap().to_string();
                let token_result = JwToken::from_token(raw_token);

                match token_result {
                    Ok(token) => return ok(token),
                    Err(message) => {
                        if message == "ExpiredSignature".to_owned() {
                            return err(ErrorUnauthorized("token expired"));
                        }
                        return err(ErrorUnauthorized("token can't be decoded"));
                    }
                }
            }
            None => return err(ErrorUnauthorized("token not in header under key 'token'")),
        }
    }
}

#[cfg(test)]
mod jwt_tests {
    use std::str::FromStr;

    use super::{Config, JwToken};
    use actix_web;
    use actix_web::http::header::{ContentType, HeaderName, HeaderValue};
    use actix_web::test::{call_service, init_service};
    use actix_web::{test::TestRequest, web, App, HttpRequest, HttpResponse};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ResponseFromTest {
        pub user_id: i32,
        pub exp_minutes: i32,
    }
    #[test]
    fn get_key() {
        assert_eq!(String::from("secret"), JwToken::get_key());
    }
    #[test]
    fn get_exp() {
        let config = Config::new();
        let minutes = config.map.get("EXPIRE_MINUTES").unwrap().as_i64().unwrap();
        assert_eq!(120, minutes);
    }
    #[test]
    fn decode_incorrect_token() {
        let encoded_token: String = String::from("invalid_token");
        match JwToken::from_token(encoded_token) {
            Err(message) => assert_eq!("InvalidToken", message),
            _ => panic!("Incorrect token should not be able to be encoded"),
        }
    }
    #[test]
    fn encode_decode() {
        let test_token = JwToken::new(5);
        let encoded_token = test_token.encode();
        let new_token = JwToken::from_token(encoded_token).unwrap();
        assert_eq!(5, new_token.user_id);
    }
    async fn test_handler(token: JwToken, _: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok().json(json!({"user_id": token.user_id, "exp_minutes": 60}));
    }
    #[actix_web::test]
    async fn test_no_token_request() {
        let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
        let req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = call_service(&app, req).await;
        assert_eq!("401", resp.status().as_str());
    }
    #[actix_web::test]
    async fn test_passing_token_request() {
        let test_token = JwToken::new(5);
        let encoded_token = test_token.encode();

        let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
        let mut req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let header_name = HeaderName::from_str("token").unwrap();
        let header_value = HeaderValue::from_str(encoded_token.as_str()).unwrap();
        req.headers_mut().insert(header_name, header_value);

        let resp: ResponseFromTest = actix_web::test::call_and_read_body_json(&app, req).await;
        assert_eq!(5, resp.user_id);
    }

    #[actix_web::test]
    async fn test_false_token_request() {
        let app = init_service(App::new().route("/", web::get().to(test_handler))).await;
        let mut req = TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let header_name = HeaderName::from_str("token").unwrap();
        let header_value = HeaderValue::from_str("test").unwrap();
        req.headers_mut().insert(header_name, header_value);
        let resp = call_service(&app, req).await;
        assert_eq!("401", resp.status().as_str());
    }
}
