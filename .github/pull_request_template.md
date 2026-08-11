## What this changes

<!-- One or two sentences. Link the issue if there is one. -->

## Contract impact

- [ ] No change to the public entry points (`stellar contract info interface` output is unchanged)
- [ ] Error codes unchanged (codes in `errors.rs` are stable and must never be renumbered)
- [ ] No change to storage keys or their durability (instance vs persistent)

If any box is unchecked, say what changed and why it is safe for already-deployed instances.

## Checks

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --locked --all-targets -- -D warnings`
- [ ] `cargo test --locked`
- [ ] Tests cover the new behaviour, including the failure paths and `require_auth`

## Notes for the reviewer

<!-- Anything worth a second look: auth, arithmetic, token transfer ordering. -->
