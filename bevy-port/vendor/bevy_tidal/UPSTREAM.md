# Vendored bevy-tidal source

This directory is a source-only snapshot of
`HumanBeanGames/bevy-tidal` from commit
`f9decb648f26d9fb7ff797707e06c0370676252a` on branch
`codex/native-rust-pattern-engine`.

The upstream repository is private. Vendoring the runtime crate keeps clean
Stream Town checkouts and GitHub Actions builds reproducible without sharing a
cross-repository credential. Large demonstration sample banks and
repository-only metadata are intentionally excluded; the small upstream parser
fixtures and coverage audit are retained. The runtime falls back to its built-in
native Rust synthesizers when an authored sample bank is absent.

The vendored files retain the upstream package declaration (`MIT OR
Apache-2.0`). Stream Town remains distributed under GPL-3.0-only.
