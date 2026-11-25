# lib-type

The `type` capability surface + `dependent` (indexed families, ADR-0120).

## Layout

```text
src/        the .hnl sources (+ lib.rs, ops.rs)
pkg.hnl     package manifest — modules, deps, description, license
Cargo.toml  the crate manifest (native `typeof.*` ops)
xtask/      cargo xtask gate (fmt → clippy → test → doc)
scripts/hnl-gate.sh   the gate
workflows/gate.yaml       bupha entry (handl-gates)
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

The Rust crate gates via `cargo xtask gate`.

## License

MIT.
