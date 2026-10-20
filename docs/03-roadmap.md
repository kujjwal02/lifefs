# Roadmap

## Phase 0: repository skeleton

Phase 0 leaves an empty repo that passes every check, locally and in CI. It uses the meeting-capture
toolchain and versions, so capture can merge in later with no tooling changes.

- A Cargo workspace on Rust 1.98.1 and edition 2024, with clippy pedantic and `unsafe_code` forbidden.
- rustfmt, taplo, biome, rumdl and typos for formatting. clippy, cargo-deny and cargo-shear for lints.
  `tsc` type-checks the UI.
- lefthook hooks, a justfile, and a Check workflow on macOS with clippy on Windows.
- `AGENTS.md` for agent rules, and a `CLAUDE.md` that imports it.
- A privacy hook that rejects `.jsonl` outside `fixtures/synthetic/` and files with real email domains.
- A Tauri 2 shell with a `sweep` subcommand, and one empty crate per phase 1 crate.

Done when `just check` passes on the Mac and in CI, `just dev` opens an empty window, and
`lifefs sweep` exits 0. The [checklist](04-checklist.md) tracks each task.

## Phase 1: Slack and meeting notes on the Mac

Each step is one branch, merged before the next starts. Each step is tested live.

| Step | Builds | Exit check |
|---|---|---|
| 1.1 Note format | `todos.base` and 5 hand-made items | A Bases cell edited in Obsidian keeps every field |
| 1.2 Store and vault writer | SQLite store and the note writer | An Obsidian edit survives the next sweep |
| 1.3 Connector contract and meetings | The `Connector` trait and the meetings connector | A ticked meeting item closes its item |
| 1.4 Tauri shell and LaunchAgent | Status list, Sweep now, LaunchAgent install | Sweeps run for 2 days with no manual run |
| 1.5 Slack read, reactions only | Slack connector and the token setup screen | A `:bookmark:` reaction creates an item |
| 1.6 Extraction | `claude -p` provider, extraction, quote check, reconcile | Slack commitments become items with valid quotes |
| 1.7 Jev verification | Jev decider, generative fallback, router | Casual talk drops out. Unsure items go to review |
| 1.8 Closures | `recheck` and reply-outcome proposals | A removed or done reaction closes its item |
| 1.9 Labelling and soak | `lifefs label` | 50 windows labelled. 7 days with no manual sweep |

Phase 1 is done when the sweep runs 7 days unattended, every quote passes the check, Obsidian
edits survive, and a removed reaction closes its item within one sweep.

## Phase 2 and later

1. Work Gmail.
2. More LLM providers: Codex, pi, any agent that speaks the Agent Client Protocol, and any
   OpenAI-compatible endpoint.
3. Personal Gmail.
4. WhatsApp with a local model.
5. Follow-up chase dates and the "Chase today" view.
6. Inferred closures without review, once the labelled set shows they are right.
7. Tests, the Keychain and code signing, once the code settles.
8. The multi-device lease, the Windows build, and the phone over Syncthing. Repeat the Bases
   cell test and the edit-survives test on Obsidian mobile.
9. Research and design the memory bank, including agent chats and agent work.
10. Merge meeting capture in as a module.

Before inferred closures ship, label about 200 message windows from my own history. Track the
precision of new items, closure accuracy and chase precision, with a target of at least 0.80 each.

## Open questions

1. TypeSafe states no retention period. Jev runs on Slack from day one anyway. Ask sales for a term.
2. Keep the Sonnet tie-breaker for the unsure band, or drop it if Jev alone passes the labelled set.
3. Is the WhatsApp Desktop database on Windows readable? If not, WhatsApp items lag while the Mac is
   away.
4. Memory bank: what it stores, how agents query it, and how it differs from the vault.
5. Agent sources: which agents, and where their transcripts live on each machine.
6. When does meeting capture merge in?

## Setup blockers

- Create the internal Slack app with `reactions:read` among its scopes. Needed for step 1.5.
- Turn off "Help improve Claude" on the account before the first sweep.
- Create the Gmail OAuth clients before phase 2. Set the personal one to "In production", because a
  client in Testing mode expires refresh tokens every 7 days.
- Grant Full Disk Access to the app before the WhatsApp connector.
