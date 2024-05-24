use std::{
    collections::HashSet,
    future::Ready,
    marker::PhantomData,
    sync::{Arc, RwLock},
};

use actix_web::{dev::Transform, FromRequest};
use jsonwebtoken::{DecodingKey, Validation};

/// A wrapper around JWTs
#[derive(Hash, PartialEq, Eq, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct JWT(pub String);

#[derive(Eq, PartialEq, Debug)]
struct InvalidatedJWTsState(HashSet<JWT>);

#[derive(Clone)]
struct AuthenticateMiddlewareFactory<ClaimsType> {
    invalidated_jwts_state: Arc<RwLock<InvalidatedJWTsState>>,
    jwt_decoding_key: Arc<DecodingKey>,
    #[cfg(feature = "session")]
    jwt_session_key: Option<Arc<JWTSessionKey>>,
    jwt_authorization_header_prefixes: Option<Arc<Vec<String>>>,
    jwt_validator: Arc<Validation>,
    _claims_type_marker: PhantomData<ClaimsType>,
}
