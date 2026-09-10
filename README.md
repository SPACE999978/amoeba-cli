# Petri

<p align="center">
  <img src="assets/petri-icon.png" alt="Petri ghost logo" width="128" height="128">
</p>

Petri is Amoeba's open-source command-line and terminal interface for bounded
hardware markets. It helps users discover markets, inspect fixed-risk option
contracts, understand maximum loss and payout, inspect collective writer sleeves and settlements,
follow oracle evidence, stake AMBA, and connect supported workflows to agents.

The executable is `petri`; Amoeba remains the company and product name.

> [!IMPORTANT]
> Petri v0.1 is a pre-production Devnet source preview. The final business audit
> records completed replay with approved oracle differences. It does not claim
> exact economic or future-payoff equivalence. Package capability does not grant runtime permission.

## Install from source

The September 9 writer-liquidity revision includes local candidate package
connections and build preparation. Tests, qualification, and deployment remain
deferred; see [local build status](docs/local-candidate-build-20260909.md). The
source exposes `writers liquidity`, `liquidity-initialize`, `liquidity-add`,
`liquidity-remove`, and `liquidity-sweep`, together with `writers withdraw`,
`refunds`, and `refund`. The Writers TUI uses the same operations. See
[writer liquidity source interface](docs/writer-liquidity-source.md) for exact
inputs and the limited buyback boundary. Existing installation pins below do
not attest these new operations.

The selected deployment is `spread-devnet-v3-writer-terminal-lifecycle-20260906`. Supported direct CLI/TUI
operations use the pinned Rust SDK's governed bytes and finalized revalidation.
Frozen, missing, paused, mismatched, or stale state prevents signing and submission.
Staking submission and unsupported operator actions remain `not_wired`; MCP stays
inspection and semantic-draft only.

The generated public source destination is `SPACE999978/amoeba-cli`. The exact
SDK and Spread Git dependencies currently require authorized access to private
repositories; anonymous source installation is not available. From a supplied
source checkout with access to those exact dependencies, use:

```bash
cargo install --locked --path . --bin petri
petri --version
```

Official macOS and Windows downloads are not published yet. Source installation
creates the terminal command; it does not install a signed native app.

## Start here

```bash
petri
petri markets
petri markets show ramx
petri contracts --market ramx
petri writers list
petri staking status
petri oracle latest ramx
petri tui
```

Use `petri help <command>` or `petri <command> --help` for the command reference
shipped by your build. Add `--json` when integrating Petri with scripts.
The TUI keeps related work in existing screens: Market Detail includes a
settlement-evidence tab, while Wallet Ledger includes Account, manager
Liquidity, collective Writers, and History tabs. Writer and staking reads use
the same byte-qualified identity boundary. Mutation grammar remains visible for
parity. Supported expert swaps, writer deposits/bids, staged close continuation,
Flat claims and transfers require a fresh active gate and initialized business
state before wallet access or preparation. Close cancellation and long claims
remain restricted by the action mask.
Manager-liquidity execution,
semantic trade routing without an authoritative route selector, and Oracle
transaction preparation remain visibly unavailable instead of being inferred.
When the TUI reports that an update is available, press `U` to close the TUI
and run the same user-safe `petri update` command. Source-checkout updates are
fail-closed: verify the full `origin/HEAD` commit through the trusted release
channel and set `PETRI_TRUSTED_UPDATE_COMMIT` to that exact 40-character commit
before installing fetched source.

| Question | Command |
| --- | --- |
| What markets are available? | `petri markets` |
| What can I trade? | `petri contracts --market ramx` |
| Are wallet changes available? | `petri config show` reports V3 package metadata; runtime permission requires fresh finalized verification |
| What collective writer sleeves are available? | `petri writers list` |
| Which writer-close custody modes are live? | Open `petri`, then choose Wallet ledger → Writers; agents can call read-only MCP tool `writers.capabilities` |
| Which writer actions does the backend describe for this wallet and sleeve? | Agents can inspect read-only MCP tool `writers.available_actions`; finalized runtime permission overrides every enabled value |
| How do I inspect a writer close? | `petri writers close-status --close-request <REQUEST>`; advancing requires current runtime permission |
| What can I redeem from staking? | `petri staking status` |
| How did a market settle? | `petri settlements show ramx <CURRENT_EXPIRY_ID>` |
| What evidence supports the oracle? | `petri oracle recipe ramx` |

## Safety model

Petri treats every backend and RPC response as untrusted input. Historical RC44
reader semantics remain separate from the exact V3 deployment identity recorded
in `release/current-governance-status.json`. Program/ProgramData bytes, the V3
controller and gate, business state, owner, action, transaction bytes and epoch
must agree with the pinned SDK. Petri never appends its own governance tail or
broadcasts arbitrary transactions. It retains the SDK message and revalidates
finalized deployment and business accounts after user approval, before typed relay.

Petri does not calculate reserves, auction results, close liabilities, settlement
values or security metrics. Missing runtime evidence fails closed. The package's
last recorded gate observation is not a permanent runtime constant.

MCP supplies inspection, semantic drafts and explicitly unsigned previews. It
cannot approve, sign or submit transactions, resolve the configured signer, or
select arbitrary local draft paths. Hosted deployment and activation are separate
publication dependencies.

Review the displayed market, expiry, premium, maximum loss, maximum payout,
settlement source, network, and wallet. Petri does not
substitute another network or fabricate live data when a required source is
unavailable.

## Configuration

Petri connects to one Amoeba service, defaulting to
`https://api.amoeba.farm`. Chain reads are derived from that base as `/rpc`;
Petri does not accept a Solana, Helius, or Photon endpoint and never receives a
private provider credential.

```bash
export AMEBA_BACKEND_URL=https://api.amoeba.farm
export SOLANA_KEYPAIR=/path/to/id.json
export AMEBA_OUTPUT=plain
```

Use `petri config show` to inspect effective non-secret configuration.
`petri config set backend-url <ORIGIN>` can save only the hosted Amoeba API
origin or an explicit loopback development origin; `petri config reset
backend-url` restores the hosted default. This is an Amoeba service setting,
not a chain-provider selector. Petri also follows the standard Solana CLI
configuration for keypair path and commitment, while deliberately ignoring its
RPC URL. It never prints keypair contents.

## MCP and connected agents

Open **Connect your AI agent** in Petri, or run `petri mcp enable`, to connect
Petri with supported AI agents. Petri can work with tools such as Claude Code,
Codex, Gemini, and other supported agents; one-click setup currently manages
the compatible local Codex and Claude Code registrations. MCP never resolves
the configured wallet, opens a local keypair, contacts a hardware wallet,
requests a transaction signature, or submits a transaction. Wallet-scoped MCP
reads require an explicit owner public key.

If Petri detects a broken managed connection, select **Repair Petri MCP** or run
`petri mcp repair`. Repair diagnoses the connection and rebuilds it in place
without requiring a disconnect first.

MCP setup requires Node.js on `PATH`. Petri writes only its owned user-level
registration and does not install a startup daemon.

## Build the release binary

```bash
cargo build --release --locked --bin petri
./target/release/petri --version
```

The public CI workflow is the reference clean-source build. It runs the locked
public Rust suite, the current-governance boundary checker, compilation, release
builds, script parsing, and command smokes. Commit-bound private
`scripts/devloop.sh` results and standalone boundary checks remain additional
release evidence. Release packages also carry the
project license, third-party inventory, and complete third-party license
corpus.

## Package for macOS

This is a release-maintainer path, not a currently published download. It
requires an Apple Developer ID signing identity and a configured notarization
profile:

```bash
export PETRI_MACOS_SIGNING_IDENTITY="Developer ID Application: ..."
export PETRI_MACOS_NOTARY_PROFILE="petri-notary"
./scripts/package-petri-macos.sh
```

The packager embeds the installer inside `Petri.app` before signing, submits the
whole bundle for notarization, staples the result, verifies it, and emits the
archive and SHA-256 file. It fails closed without the required credentials.

## Package for Windows

This is also a release-maintainer path. Build the exact release files, sign the
executable and installer helpers with a publicly trusted Authenticode identity,
then package without rebuilding:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/package-petri-windows.ps1 -BuildOnly
# Sign target\release\petri.exe, scripts\install-petri.ps1, and
# scripts\finish-petri-windows-update.ps1.
powershell -ExecutionPolicy Bypass -File scripts/package-petri-windows.ps1 -SkipBuild
```

The Windows packager and installer reject unsigned or altered release files and
require the executable, installer, and updater helper to share one publisher
certificate.

## Documentation and security

- Start with the bundled [Amoeba documentation](assets/gitbook/README.md).
- Read [SECURITY.md](SECURITY.md) before reporting a vulnerability.
- Third-party components are listed in
  [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md), with full license text in
  [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).

Please do not open a public issue containing a vulnerability, wallet material,
seed phrase, API key, keypair JSON, or private endpoint credential.

## License

Petri is licensed under the [Apache License, Version 2.0](LICENSE). Amoeba and
Petri names and artwork are covered by [TRADEMARKS.md](TRADEMARKS.md).

September 7 release note: the final business audit accepts explicitly documented oracle differences. It does not claim exact oracle economic equality or future payoff equality. The raw identity capture and business audit are dated evidence; every mutation still requires fresh runtime admission. Final integration checks were skipped at user request.
