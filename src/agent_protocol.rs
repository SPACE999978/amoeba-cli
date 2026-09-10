use serde_json::Value;

const MCP_MANIFEST_JSON: &str = r#"{
  "action": "mcp_manifest",
  "clientConfig": {
    "claudeCode": {
      "configPath": "~/.claude.json",
      "lifecycle": "Claude Code starts the Petri stdio server when Claude Code opens.",
      "scope": "user",
      "serverName": "petri",
      "setup": "Use Home > Connect your AI agent in the Petri TUI."
    },
    "codex": {
      "configPath": "~/.codex/config.toml",
      "lifecycle": "Codex starts the Petri stdio server when Codex opens.",
      "scope": "user",
      "serverName": "petri",
      "setup": "Use Home > Connect your AI agent in the Petri TUI."
    }
  },
  "currentRelease": {
    "packageWriteCapable": true,
    "governanceGeneration": 3,
    "governanceGate": "Cdym9p7FvtxEAjF8XuCqSrishB7LmBDXaZGDMMgWczu",
    "governanceStatus": "Active",
    "governanceEpoch": "9",
    "observationOnly": true,
    "observedAt": "2026-09-06T23:17:03.694Z",
    "runtimePermission": "requires-finalized-verification",
    "historicalReaderSemanticRelease": "v0.1.0-rc.44",
    "programId": "2jVQSPny9eFoaG1ZWoJVAezQ5VgqJtF8rQCQXMktuBVw",
    "sdkCommit": "ac30e32bbd151d8819e4a06eafdf33570bab0eab",
    "walletChangesAvailable": false,
    "writeCompatibility": "governance-gate-v1",
    "writeErrorCode": "CURRENT_PROGRAM_WRITE_ABI_UNAVAILABLE"
  },
  "humanWorkflows": {
    "writerClose": {
      "cancelAvailability": "Writer-close cancellation is unavailable in the current action-mask boundary; the preserved CLI grammar is not an executable fallback.",
      "capabilityTool": "writers.capabilities",
      "executionBoundary": "Inspection only. MCP only inspects. Direct CLI/TUI requires finalized V3 permission and initialized business state, even if stale capability or action-mask data appears enabled.",
      "availabilityBoundary": "Capability, hot/cold custody, exact owner+sleeve action masks, previews, and status remain read-only evidence. The finalized runtime gate overrides every dynamic enabled value.",
      "actionMaskTool": "writers.available_actions",
      "releaseContract": {
        "governanceGeneration": 3,
        "governanceStatus": "Active",
        "sdkCommit": "ac30e32bbd151d8819e4a06eafdf33570bab0eab",
        "walletChangesAvailable": false,
        "writeCompatibility": "governance-gate-v1",
        "writeErrorCode": "CURRENT_PROGRAM_WRITE_ABI_UNAVAILABLE"
      },
      "mutationStatus": "requires_finalized_runtime_permission",
      "previewTool": "writers.close_preview",
      "statusTool": "writers.close_status",
      "workflow": "Inspect capability, exact owner+sleeve availability, preview, and status only. Explain that frozen, uninitialized or paused state prevents execution; MCP cannot start, advance, cancel, claim or transfer."
    }
  },
  "nativeMcpServer": {
    "crateOrPackage": "petri-mcp",
    "status": "planned",
    "v0Implementation": "use scripts/petri-mcp-server.mjs as the stdio wrapper",
    "v1Implementation": "call the same protocol core functions used by CLI/TUI"
  },
  "ok": true,
  "protocol": {
    "commandStarter": "petri",
    "mcpSpecVersion": "2025-06-18",
    "name": "petri-agent-protocol",
    "transport": "CLI JSON wrapper first; shared core extraction before a native MCP server",
    "version": "v0"
  },
  "safety": {
    "defaultAgentMode": "inspect_draft_validate_preview",
    "managedSigningBoundary": "Managed hosted execution has not been activated. Local V3 capability does not authorize managed signing.",
    "secretPolicy": "Never expose keypair JSON, .env values, Helius keys, signer tokens, or wallet credentials.",
    "signerAccess": "Petri MCP never resolves the configured wallet, opens a local keypair, or contacts a hardware wallet. Every wallet-scoped read requires an explicit owner public key. Direct Petri mutations require finalized gate and business readiness before signer access.",
    "signing": "Petri MCP never signs or submits transactions; direct CLI/TUI uses typed governed operations after finalized revalidation.",
    "transactionControls": "Petri MCP exposes no approval, confirmation, transaction-signature authorization, broadcast, or execution tool. Direct CLI/TUI supports only typed SDK governed operations after finalized revalidation.",
    "walletActionAvailability": "Capability and writers.available_actions are read-only evidence. The finalized runtime gate overrides every dynamic enabled value and neither surface can authorize preparation, signing, or submission.",
    "unreleasedWriterActions": "Writer liquidity and exact collective-long claim integration are pre-testing source; current executable support requires qualified packages and fresh native admission. MCP exposes no wallet mutation."
  },
  "stdioMcpServer": {
    "command": [
      "node",
      "scripts/petri-mcp-server.mjs"
    ],
    "implementation": "JSON-RPC stdio wrapper that exposes these tools and spawns petri --json commands",
    "status": "wired_v0"
  },
  "tools": [
    {
      "cli": [
        "petri",
        "--json",
        "writers",
        "capabilities"
      ],
      "description": "Read historical writer capability and separate hot/cold Light-account evidence. Finalized runtime permission and action admission are required for every wallet change.",
      "name": "writers.capabilities",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "writers",
        "available-actions",
        "--owner",
        "<owner_pubkey>",
        "--sleeve",
        "<sleeve_pubkey>"
      ],
      "description": "Read the exact current owner+sleeve 23-action availability mask as inspection evidence only. New writer-liquidity source is pre-testing; every wallet change needs fresh native and SDK admission.",
      "name": "writers.available_actions",
      "status": "wired_read_only"
    },
    {
      "cli": ["petri", "--json", "writers", "liquidity", "--owner", "<owner_pubkey>", "--sleeve", "<sleeve_pubkey>", "--series-index", "<0..19>"],
      "description": "Read exact writer liquidity, custody, reserve, inventory and frozen budgets for an explicit actor and series. No wallet resolution, preparation or signing.",
      "name": "writers.liquidity",
      "status": "wired_read_only"
    },
    {
      "cli": ["petri", "--json", "writers", "refunds", "--owner", "<owner_pubkey>", "--limit", "<1..32>"],
      "description": "Discover historical auction refund rows for an explicit owner; optional cursor resumes the bounded inventory. This tool never prepares or submits refunds.",
      "name": "writers.refunds",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "mcp",
        "manifest"
      ],
      "description": "Read Petri's machine-readable MCP/agent manifest.",
      "name": "petri.mcp_manifest",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "markets"
      ],
      "description": "List user-facing Petri markets.",
      "name": "market.list",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "markets",
        "show",
        "<market_id>"
      ],
      "description": "Open one market summary.",
      "name": "market.show",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "markets",
        "status",
        "<market_id>"
      ],
      "description": "Read the compact live status for one market.",
      "name": "market.status",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "markets",
        "print",
        "<market_id>"
      ],
      "description": "Read the latest oracle print for one market.",
      "name": "market.print",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "markets",
        "chart",
        "<market_id>",
        "--static"
      ],
      "description": "Read non-interactive market or expiry chart history.",
      "name": "market.chart",
      "status": "wired_static"
    },
    {
      "cli": [
        "petri",
        "--json",
        "contracts",
        "--market",
        "<market_id>"
      ],
      "description": "Read a bounded option chain with prices, depth, availability, risk, freshness, and no-live-contract issues.",
      "name": "contracts.chain",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "contracts",
        "--market",
        "<market_id>",
        "--all"
      ],
      "description": "Find factual call or put candidates by quote, depth, and on-chain availability without opening an order.",
      "name": "contracts.find",
      "status": "wired_mcp_read"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "recipe",
        "ramx",
        "--query",
        "<optional_query>"
      ],
      "description": "Read the RAMX-MOD source tree, row weights, and source pins from the active backend oracle recipe.",
      "name": "oracle.read_recipe",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "state"
      ],
      "description": "Read spread-owned oracle totals and market summaries from the Amoeba API.",
      "name": "oracle.read_state",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "markets",
        "<optional_market_id>"
      ],
      "description": "List spread-owned oracle markets, or inspect one market when a market id is provided.",
      "name": "oracle.list_markets",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "latest",
        "<optional_market_id>"
      ],
      "description": "Read the latest spread-owned oracle month and settlement.",
      "name": "oracle.read_latest",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "history",
        "<optional_market_id>"
      ],
      "description": "Read spread-owned oracle settlement history.",
      "name": "oracle.read_history",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "settlements",
        "show",
        "<market_id>",
        "<expiry_id>"
      ],
      "description": "Read one spread settlement record through the Amoeba API.",
      "name": "settlement.show",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "settlements",
        "check",
        "<market_id>",
        "<expiry_id>"
      ],
      "description": "Read spread settlement oracle preflight for one market expiry.",
      "name": "settlement.check",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "settlements",
        "oracle",
        "<market_id>",
        "<expiry_id>"
      ],
      "description": "Read the spread oracle package/signing state for one settlement.",
      "name": "settlement.read_oracle",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "wallet",
        "balance",
        "<owner_pubkey>"
      ],
      "description": "Read SOL, USDC, and AMBA balances for a wallet.",
      "name": "wallet.balance",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "wallet",
        "collateral",
        "--owner",
        "<owner_pubkey>"
      ],
      "description": "Read current trading-collateral state and the next safe action for a wallet.",
      "name": "wallet.collateral",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "staking",
        "status",
        "--owner",
        "<owner_pubkey>"
      ],
      "description": "Read verified AMBA/sAMBA staking, activation, reward, and unstaking status without changing it.",
      "name": "staking.status",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "writers",
        "list"
      ],
      "description": "Read the current global collective-writer sleeve catalog. This is not a wallet-owned position view.",
      "name": "writers.list",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "writers",
        "show",
        "--sleeve",
        "<sleeve_pubkey>"
      ],
      "description": "Read one current collective-writer sleeve and its staged state.",
      "name": "writers.show",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "writers",
        "policy-audit",
        "--sleeve",
        "<sleeve_pubkey>"
      ],
      "description": "Read the active writer policy and immutable audit hashes for one sleeve.",
      "name": "writers.policy_audit",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "writers",
        "close-preview",
        "--sleeve",
        "<sleeve_pubkey>",
        "--amount",
        "<flat_par_atoms>",
        "--minimum-withdrawal",
        "<minimum_usdc_atoms>"
      ],
      "description": "Inspect a staged-close preview and exact basket/withdrawal bounds. This cannot authorize or lead to current-release signing or submission.",
      "name": "writers.close_preview",
      "status": "wired_unsigned_preview"
    },
    {
      "cli": [
        "petri",
        "--json",
        "writers",
        "close-status",
        "--close-request",
        "<close_request_pubkey>"
      ],
      "description": "Read one staged writer-close request and its historical next-stage projection. Current mutation execution remains unavailable regardless of that value.",
      "name": "writers.close_status",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "liquidity",
        "--owner",
        "<owner_pubkey>"
      ],
      "description": "Read owner-bound manager-liquidity positions; the current hosted route fails closed when it cannot prove a non-empty row's canonical Market.",
      "name": "liquidity.positions",
      "status": "wired_read_fail_closed_upstream_limited"
    },
    {
      "cli": [
        "petri",
        "--json",
        "history",
        "<owner_pubkey>"
      ],
      "description": "Read authoritative untyped wallet history and recent wallet activity; category filters are unavailable.",
      "name": "history.read",
      "status": "wired_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "sources",
        "propose",
        "<sku_or_row>",
        "--category",
        "<public_source_type>",
        "--locator",
        "<url>",
        "--definition",
        "<definition>",
        "--stake",
        "<amount>"
      ],
      "description": "Create a local source proposal draft without submitting it.",
      "name": "source.draft_submission",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "sources",
        "support",
        "<sku_or_row>",
        "--stake",
        "<amount>"
      ],
      "description": "Create a local source support/backing draft without submitting it.",
      "name": "source.draft_support",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "sources",
        "challenge",
        "<sku>",
        "--reason",
        "<reason>",
        "--archive-url",
        "<url>",
        "--stake",
        "<amount>",
        "[--comparison-source",
        "<source_id>]"
      ],
      "description": "Create a local source/update challenge draft; source differentiation challenges use --comparison-source.",
      "name": "source.draft_challenge",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "prints",
        "opening",
        "<sku>",
        "--raw-value",
        "<value>",
        "--timestamp",
        "<iso8601>",
        "--archive-url",
        "<https://web.archive.org/web/...>",
        "--canonical-locator",
        "<url_or_locator>",
        "--source-definition",
        "<definition>",
        "--stake",
        "<amount>"
      ],
      "description": "Create a pending opening claim with https://web.archive.org/web/<14-digit UTC>/<exact canonical URL> (max 384 UTF-8 bytes); capture time must equal source time and the spread program derives the evidence hash.",
      "name": "opening.draft_print",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "prints",
        "challenge",
        "<sku>",
        "--reason",
        "<reason>",
        "--corrected-value",
        "<value>",
        "--timestamp",
        "<iso8601_or_unix>",
        "--archive-url",
        "<https://web.archive.org/web/...>",
        "--canonical-locator",
        "<url_or_locator>",
        "--source-definition",
        "<definition>",
        "--stake",
        "<amount>"
      ],
      "description": "Challenge one pending opening claim with https://web.archive.org/web/<14-digit UTC>/<exact canonical URL> (max 384 UTF-8 bytes); no caller evidence hash is accepted.",
      "name": "opening.draft_challenge",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "updates",
        "commit",
        "--source",
        "<source_id>",
        "--claim-id",
        "<hex_or_draft_id>",
        "--commit-hash",
        "<hex32>",
        "--stake",
        "<amount>"
      ],
      "description": "Create a local hidden source-local update commitment draft.",
      "name": "update.commit_claim",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "updates",
        "reveal",
        "--source",
        "<source_id>",
        "--claim-id",
        "<hex_or_draft_id>",
        "--prior-state",
        "<current_state>",
        "--raw-value",
        "<value>",
        "--timestamp",
        "<source_time>",
        "--archive-url",
        "<wayback_url>",
        "--secret-salt",
        "<hex32>"
      ],
      "description": "Create a local reveal draft for a hidden source-local update commitment using archive evidence.",
      "name": "update.reveal_claim",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "updates",
        "expire",
        "--source",
        "<source_id>",
        "--claim-id",
        "<claim_id>",
        "--claimant",
        "<claimant_address>"
      ],
      "description": "Settle the bond of a claimant-scoped update commitment that missed its reveal deadline.",
      "name": "update.expire_commitment",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "updates",
        "challenge",
        "<sku>",
        "--reason",
        "<reason>",
        "--archive-url",
        "<url>",
        "--claim-id",
        "<hex_or_draft_id>",
        "--claimant",
        "<claimant_address>",
        "--corrected-value",
        "<positive_state>"
      ],
      "description": "Challenge one revealed claimant-scoped update claim using its exact claim id, original claimant address, backend-safe positive integer corrected state, and archive evidence.",
      "name": "update.challenge_claim",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "emergency",
        "commit",
        "--dispute-id",
        "<hex32>",
        "--commit-hash",
        "<hex32>",
        "--lock-amount",
        "<amount>"
      ],
      "description": "Create a local hidden emergency vote commitment draft.",
      "name": "emergency.commit_vote",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "emergency",
        "reveal",
        "--dispute-id",
        "<hex32>",
        "--choice",
        "<choice>",
        "--salt",
        "<hex32>"
      ],
      "description": "Create a local emergency vote reveal draft.",
      "name": "emergency.reveal_vote",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "rewards",
        "claim",
        "--kind",
        "<source_proposer|source_support|opening|update>"
      ],
      "description": "Create a local current USDC oracle reward claim draft without submitting it.",
      "name": "oracle.reward.claim",
      "status": "wired_local_draft"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "stakes",
        "settle",
        "--kind",
        "<listing-bond|support-stake|source-challenge|opening-claim|opening-challenge|update-claim|update-challenge|emergency-vote>",
        "[--subject-pda",
        "<canonical_record_from_oracle_latest>]",
        "[--source-id",
        "<listing_bond_source_only>]"
      ],
      "description": "Create a permissionless terminal stake/bond settlement draft; the spread program derives refund or slash and the caller cannot choose the disposition.",
      "name": "oracle.stake.settle",
      "status": "wired"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "amba",
        "deposit",
        "--amount",
        "<amount>"
      ],
      "description": "Create a local AMBA deposit draft for oracle voting custody.",
      "name": "amba.deposit",
      "status": "wired_local_draft"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "amba",
        "withdraw",
        "--amount",
        "<amount>"
      ],
      "description": "Create a local AMBA withdraw draft for available oracle voting custody.",
      "name": "amba.withdraw",
      "status": "wired_local_draft"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "drafts",
        "list",
        "--limit",
        "<optional_limit>"
      ],
      "description": "List locally queued semantic Oracle drafts without preparing, signing, or sending a transaction.",
      "name": "oracle.drafts.list",
      "status": "wired_semantic_read_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "drafts",
        "validate",
        "<selector>"
      ],
      "description": "Validate selected semantic oracle drafts without preparing, signing, or sending a transaction.",
      "name": "oracle.drafts.validate",
      "status": "wired_semantic_validation_only"
    },
    {
      "cli": [
        "petri",
        "--json",
        "oracle",
        "drafts",
        "show",
        "latest"
      ],
      "description": "Show one semantic oracle draft without preparing, signing, or sending a transaction.",
      "name": "oracle.drafts.show",
      "status": "wired_semantic_read_only"
    }
  ]
}"#;

pub(crate) fn mcp_manifest() -> Value {
    let mut manifest: Value =
        serde_json::from_str(MCP_MANIFEST_JSON).expect("bundled MCP manifest JSON must be valid");
    manifest["currentRelease"]["packageBuildIdentity"] =
        ameba_sdk::current_sdk_package_build_identity_v1().unwrap_or(Value::Null);
    manifest["currentRelease"]["packageWriteCapable"] =
        Value::Bool(crate::current_release::require_current_write_release().is_ok());
    manifest
}

pub(crate) fn render_mcp_manifest(payload: &Value) -> String {
    let tool_count = payload
        .get("tools")
        .and_then(Value::as_array)
        .map(Vec::len)
        .unwrap_or_default();
    [
        "Petri MCP manifest".to_string(),
        "protocol=petri-agent-protocol.v0".to_string(),
        format!("tools={tool_count}"),
        "v0: wrap petri --json commands; v1: call shared protocol core.".to_string(),
        "Petri MCP only inspects or drafts; it never prepares, signs, or submits transactions."
            .to_string(),
    ]
    .join("\n")
}
