//! Exact current-release boundary shared by Petri's CLI, TUI, and signing code.
//!
//! The live program can be inspected with the historical RC44 reader semantics,
//! while current writes require the SDK's opaque, package-qualified release.
//! An unavailable candidate still stops before network or signer access. This
//! module never manufactures readiness from a CLI flag or historical metadata.

use crate::backend::CliError;

/// Published package recorded by the retained release observation. The actual
/// local candidate source and artifacts are exposed separately by the SDK.
pub const SDK_PACKAGE_COMMIT: &str = "ac30e32bbd151d8819e4a06eafdf33570bab0eab";
pub const READER_SEMANTIC_RELEASE: &str = "v0.1.0-rc.44";
pub const READER_SEMANTIC_SOURCE_COMMIT: &str = "1b2230d96e51f6582155d8284900fbfc11ff1f18";
pub const LIVE_RELEASE_LABEL: &str = "spread-devnet-v3-writer-dlmm-20260909";
pub const LIVE_READ_PROFILE_ID: &str = "ameba-spread-v2-deployed-60d7a856";
pub const REVIEWED_BRIDGE_SOURCE_COMMIT: &str = "60d7a856f627f880a62fb93c951a1252615e1198";
pub const DEPLOYMENT_PROVENANCE: &str = "finalized-rpc-account";
pub const WRITE_COMPATIBILITY: &str = "governance-gate-v1";
pub const WRITE_ERROR_CODE: &str = "CURRENT_PROGRAM_WRITE_ABI_UNAVAILABLE";

pub const PROGRAM_ACCOUNT_BYTES: usize = 36;
pub const PROGRAM_ACCOUNT_SHA256: &str =
    "83be84ac424d864aacf03e489ff8fe3f0a6957e8726c8f9c48f572df6b522e75";
pub const PROGRAM_DATA_ACCOUNT_BYTES: usize = 1351501;
pub const PROGRAM_DATA_ACCOUNT_SHA256: &str =
    "bcb4b3f0782b3e7ea5e741eb2da7a753efbaa8f4e61fe3c3a27e97db99d4d9ca";
pub const PROGRAM_DATA_PAYLOAD_BYTES: usize = 1351456;
pub const PROGRAM_DATA_PAYLOAD_SHA256: &str =
    "6a47719f1c053c6727bf4e8050b1e704362c811b3d28e3a68ec02791644d4b3d";
pub const PROGRAM_DATA_DEPLOYED_SLOT: u64 = 495873438;
pub const PROGRAM_UPGRADE_AUTHORITY: &str = "4hsEKyThDv85YA4HjUaGWn2nWnVbM5V23XcLrEX3UKzn";

pub fn write_unavailable_error() -> CliError {
    CliError::new(format!(
        "Wallet changes are unavailable because packaged Amoeba write evidence could not be verified. Nothing was prepared, signed, or sent. [{WRITE_ERROR_CODE}]"
    ))
}

/// Package-qualified capability gate. This runs before network preparation,
/// RPC access, signer work, or transaction construction in every current
/// mutation flow.
pub fn require_current_write_release() -> Result<(), CliError> {
    ameba_sdk::assert_current_write_release_available().map_err(|_| write_unavailable_error())
}

pub fn selected_write_release() -> Result<Option<ameba_sdk::CurrentGovernedWriteReleaseV1>, CliError>
{
    match ameba_sdk::current_governed_write_release_v1() {
        Ok(release) => Ok(Some(release)),
        Err(ameba_sdk::GovernedOperationError::ReleaseUnavailable) => Ok(None),
        Err(_) => Err(CliError::new(
            "The packaged current release evidence is inconsistent.",
        )),
    }
}

/// Exact byte pins come from the qualified SDK; historical reader pins are
/// used only when the compiled write train explicitly remains unavailable.
pub struct CurrentReadDeployment {
    pub source_commit: Option<String>,
    pub program_sha256: String,
    pub programdata_sha256: String,
    pub payload_sha256: String,
    pub programdata_bytes: usize,
    pub payload_bytes: usize,
    pub artifact_bytes: usize,
    pub artifact_sha256: String,
    pub mandatory_zero_padding_bytes: usize,
    pub deployed_slot: u64,
    pub upgrade_authority: String,
    pub gate: solana_pubkey::Pubkey,
    pub controller: solana_pubkey::Pubkey,
}

pub fn current_read_deployment() -> Result<CurrentReadDeployment, CliError> {
    if let Some(release) = selected_write_release()? {
        return Ok(CurrentReadDeployment {
            // The public DTO names the deployed artifact source, not the tooling commit.
            source_commit: Some(release.artifact_source_commit().to_owned()),
            program_sha256: release.program_account_sha256().to_owned(),
            programdata_sha256: release.programdata_account_sha256().to_owned(),
            payload_sha256: release.payload_sha256().to_owned(),
            programdata_bytes: release.programdata_bytes(),
            payload_bytes: release.payload_bytes(),
            artifact_bytes: release.artifact_bytes(),
            artifact_sha256: release.artifact_sha256().to_owned(),
            mandatory_zero_padding_bytes: release.mandatory_zero_padding_bytes(),
            deployed_slot: release.deployed_slot(),
            upgrade_authority: release.upgrade_authority().to_string(),
            gate: release.gate_address(),
            controller: release.controller_program(),
        });
    }
    Ok(CurrentReadDeployment {
        source_commit: None,
        program_sha256: PROGRAM_ACCOUNT_SHA256.to_owned(),
        programdata_sha256: PROGRAM_DATA_ACCOUNT_SHA256.to_owned(),
        payload_sha256: PROGRAM_DATA_PAYLOAD_SHA256.to_owned(),
        programdata_bytes: PROGRAM_DATA_ACCOUNT_BYTES,
        payload_bytes: PROGRAM_DATA_PAYLOAD_BYTES,
        artifact_bytes: 1351456,
        artifact_sha256: "6a47719f1c053c6727bf4e8050b1e704362c811b3d28e3a68ec02791644d4b3d"
            .to_owned(),
        mandatory_zero_padding_bytes: 0,
        deployed_slot: PROGRAM_DATA_DEPLOYED_SLOT,
        upgrade_authority: PROGRAM_UPGRADE_AUTHORITY.to_owned(),
        gate: ameba_sdk::CURRENT_GOVERNANCE_GATE_V1,
        controller: ameba_sdk::CURRENT_GOVERNANCE_CONTROLLER_V1,
    })
}
