# lifefs

One local app that gathers my context from mail, Slack, WhatsApp, meetings and my agent chats, and
puts it in order. Its first module, Owed, keeps one list of to-dos and follow-ups and closes the
finished ones. Meeting capture merges in as a module later.

Status: phase 0 skeleton in place. Phase 1 step 1.1, the note format, is next.

- [Overview](docs/01-overview.md): problem, goals, non-goals, modules
- [Architecture](docs/02-architecture.md): components, design rules, decisions
- [Roadmap](docs/03-roadmap.md): phase 0, the nine steps of phase 1, later phases, open questions
- [Checklist](docs/04-checklist.md): every task in phase 0 and phase 1, ticked when done

## Development

Needs Rust (pinned by `rust-toolchain.toml`), Bun, `just`, `cargo tauri`, lefthook,
taplo, typos, biome, rumdl, cargo-deny and cargo-shear.

```sh
just setup        # git hooks and UI packages, once per clone
just dev          # the window, with the Vite dev server
just sweep        # one sweep from a debug build
just check        # every format, lint, type and dependency check
```

The research behind the design lives in my private notes. The docs here state the current design.

## Licence

MIT OR Apache-2.0, at your option. See `LICENSE-MIT` and `LICENSE-APACHE`.
