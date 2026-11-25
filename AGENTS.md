# lib-type

The `type` capability surface + `dependent` (indexed families,
ADR-0120): the `.hnl` interface plus the Rust crate providing native
`typeof.*` ops.

## Layout

| path | role |
| --- | --- |
| `src/index.hnl` | the interface |
| `src/type.hnl`, `src/dependent.hnl` | the type / dependent modules |
| `src/lib.rs`, `src/ops.rs` | the Rust crate (native `typeof.*` ops) |
| `atoms.yaml`, `Cargo.toml`, `xtask/` | atoms / manifest / `cargo xtask` gate |
| `pkg.hnl` | manifest — `modules: ["type", "dependent"]` |
| `scripts/hnl-gate.sh`, `ci.sh` | the hnl gate / bupha entry |

## Contract

- Transitional layout: `.hnl` surface (`type` + `dependent` indexed
  families) + native Rust ops behind it.

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
cargo xtask gate        # the Rust crate side
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
