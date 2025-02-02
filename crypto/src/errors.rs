// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

use fvm_ipld_encoding::Error as EncodingError;
use libsecp256k1::Error as SecpError;
use thiserror::Error;

/// Crypto error
#[derive(Debug, PartialEq, Eq, Error)]
pub enum Error {
    /// Failed to produce a signature
    #[error("Failed to sign data {0}")]
    SigningError(String),
    /// Unable to perform `ecrecover` with the given parameters
    #[error("Could not recover public key from signature: {0}")]
    InvalidRecovery(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Error {
        // Pass error encountered in signer trait as module error type
        Error::SigningError(err.to_string())
    }
}

impl From<SecpError> for Error {
    fn from(err: SecpError) -> Error {
        match err {
            SecpError::InvalidRecoveryId => Error::InvalidRecovery(format!("{:?}", err)),
            _ => Error::SigningError(format!("{:?}", err)),
        }
    }
}

impl From<EncodingError> for Error {
    fn from(err: EncodingError) -> Error {
        // Pass error encountered in signer trait as module error type
        Error::SigningError(err.to_string())
    }
}
