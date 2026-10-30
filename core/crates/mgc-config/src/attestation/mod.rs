//! Core identity attestation facade / Facade chứng thực identity core.
mod anchor_mode;
mod attestation;
mod enforcement;
mod keyring;
mod message_digest;
mod project_slot;
mod rotation;
mod signing;
mod types;
mod verification;

pub use anchor_mode::{ANCHOR_MODE_ENV, AnchorMode};
pub use attestation::{attest, keyring_public_key, pin_key_in_manifest};
pub use enforcement::{enforce_live_attestation, enforce_missing_marker_attestation};
pub use keyring::load_or_create_keyring;
pub use message_digest::{
    ATTESTATION_MESSAGE_DOMAIN, CoreIdentity, blake3_hex, canonical_message, signed_message,
};
pub use project_slot::{ATTESTATION_FILE, attestation_dir, read_config_bytes};
pub use signing::attester_version;
pub use types::{Attestation, AttestationStatus, PreviousKey};
pub use verification::verify;

use rotation::previous_key_for;
use signing::sign_and_store;
