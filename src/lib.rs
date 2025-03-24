#[macro_use]
extern crate doc_comment;

doctest!("../README.md");

use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use p256::FieldBytes;
use p256::ecdsa::signature::Signer;
use p256::ecdsa::{Signature, SigningKey};
use progenitor::generate_api;
use std::sync::OnceLock;
use thiserror::Error;

const API_PRIVATE_KEY_VAR: &str = "TURNKEY_API_PRIVATE_KEY";

static API_PRIVATE_KEY: OnceLock<SigningKey> = OnceLock::new();
static API_PUBLIC_KEY: OnceLock<String> = OnceLock::new();

generate_api!(
    spec = "./openapi3-spec.json",
    pre_hook_async = crate::stamp_requests,
);

async fn stamp_requests(
    req: &mut reqwest::Request,
) -> Result<(), reqwest::header::InvalidHeaderValue> {
    // Only POST requests need to be stamped
    if req.method() != reqwest::Method::POST {
        return Ok(());
    }

    let body = req.body().unwrap().as_bytes().unwrap();
    let stamp = create_stamp(body).unwrap();
    req.headers_mut().insert("X-Stamp", stamp.parse().unwrap());

    Ok(())
}

#[derive(Error, Debug, PartialEq)]
pub enum StampError {
    #[error("cannot load private key: invalid bytes")]
    InvalidPrivateKeyBytes,
}

pub fn create_stamp(request_body: &[u8]) -> Result<String, StampError> {
    let private_key = API_PRIVATE_KEY.get_or_init(|| {
        let private_key_hex =
            std::env::var(API_PRIVATE_KEY_VAR).expect("missing env var: TURNKEY_API_PRIVATE_KEY");
        let private_key_bytes =
            hex::decode(&private_key_hex).expect("TURNKEY_API_PRIVATE_KEY env var not valid hex");

        SigningKey::from_bytes(FieldBytes::from_slice(&private_key_bytes))
            .map_err(|_| StampError::InvalidPrivateKeyBytes)
            .unwrap()
    });

    let public_key = API_PUBLIC_KEY.get_or_init(|| {
        let public_key = private_key.verifying_key();
        let compressed_publick_key = public_key.to_encoded_point(true);
        hex::encode(compressed_publick_key.as_bytes())
    });

    let sig: Signature = private_key.sign(request_body);
    let stamp = format!(
        r#"{{"publicKey": "{}","signature": "{}","scheme": "SIGNATURE_SCHEME_TK_API_P256"}}"#,
        public_key,
        hex::encode(sig.to_der())
    );

    Ok(BASE64_URL_SAFE_NO_PAD.encode(stamp.as_bytes()))
}
