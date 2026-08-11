# CI / CD

Everything under `.github/` and how to operate it. Run `make ci` locally to
check the same things CI does before you push.

## Workflows

| Workflow | Trigger | What it does |
| --- | --- | --- |
| `ci.yml` | push to `main`, every PR, manual | fmt, clippy, tests, wasm build, dependency audit |
| `release.yml` | push of a `v*` tag, manual | re-verifies, builds the optimized wasm, publishes a GitHub Release with checksums |
| `deploy.yml` | manual only | deploys a build to testnet / futurenet / mainnet and verifies the instance |
| `security.yml` | weekly + lockfile changes | `cargo audit` against the RustSec advisory DB, opens an issue on a new advisory |

`.github/actions/setup-stellar-cli` installs a **pinned** prebuilt `stellar`
binary (currently 23.1.4) and caches it. Dependabot does not track that version;
bump `inputs.version` in the composite action by hand when you move the local
toolchain.

### CI details

- Every cargo command runs with `--locked`. `Cargo.lock` is committed on
  purpose, and `--locked` fails the build if it drifts from `Cargo.toml`.
- The toolchain comes from `rust-toolchain.toml` (`rustup show` installs the
  channel, the `wasm32v1-none` target, and the components), so CI and local
  builds never diverge.
- The `build` job runs `stellar contract build`, optimizes, and uploads the
  wasm, its sha256, its exported interface, and its contract meta as an
  artifact. The interface is also printed to the run summary, so an accidental
  entry-point change is visible in the PR without downloading anything.
- Warnings are denied through clippy's `-D warnings`, not through a global
  `RUSTFLAGS`, so a noisy dependency can't fail our build.
- A final `ci` job aggregates the rest. Point branch protection at that one
  check so adding a job never means editing repo settings.

## Releasing

```bash
# version in contracts/payroll/Cargo.toml must already match
git tag -a v0.1.0 -m "payroll v0.1.0"
git push origin v0.1.0
```

The workflow refuses to publish if the tag doesn't match the crate version. The
release carries `disburs_payroll.wasm`, its interface and meta dumps, and
`SHA256SUMS`. Anyone can rebuild from the tag with the pinned toolchain and
compare hashes.

## Deploying

Actions → **Deploy** → *Run workflow*. Inputs: network, admin address, token
address, and optionally a release tag to deploy a published wasm instead of
building from the ref. After deploying it calls `get_admin` and
`treasury_balance` on the new instance and fails if the admin isn't what you
asked for. The contract id lands in the run summary.

The payroll contract has **no upgrade entry point**: a deploy creates a new
instance, it never replaces one. Migrating means deploying fresh and moving
funds and salary config over.

### Required one-time setup

Create a GitHub Environment per network (`testnet`, `futurenet`, `mainnet`) with:

- Secret `STELLAR_SECRET_KEY` — the deployer's secret key (`S...`), funded on
  that network. Use a different key per environment; never a repo-level secret.
- On `mainnet`: required reviewers, and restrict deployment branches to `main`.
  The key is only exposed to the job after a human approves.

```bash
gh api -X PUT repos/:owner/:repo/environments/testnet
gh api -X PUT repos/:owner/:repo/environments/mainnet \
  -f 'reviewers[][type]=User' -F 'reviewers[][id]=<your-user-id>'
gh secret set STELLAR_SECRET_KEY --env testnet
gh secret set STELLAR_SECRET_KEY --env mainnet
```

### Recommended repo settings

- Branch protection on `main`: require the `CI` status check in addition to the
  existing review requirement.
- Actions → General → Workflow permissions: **read-only** by default. The two
  jobs that need more (`release` writes releases, `security` opens issues) ask
  for it per job.
- Enable Dependabot alerts and security updates so `.github/dependabot.yml`
  produces PRs.

```bash
gh api -X PATCH repos/:owner/:repo/branches/main/protection/required_status_checks \
  -f strict=true -f 'checks[][context]=CI'
```

## Notes on the workflow code

Workflow inputs are passed to shell steps through `env:`, never interpolated
into the script body with `${{ }}`. That matters most in `deploy.yml`, which
holds a signing key.
