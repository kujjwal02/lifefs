# Checklist

Every phase and step from the [roadmap](03-roadmap.md), with the tasks inside each one. A ticked box
is done and committed. A step is done when its "Done when" boxes are ticked. Each phase 1 step is
one branch, merged before the next starts.

## Phase 0: repository skeleton

### Workspace and app shell

- [x] Cargo workspace: resolver 3, edition 2024, Rust 1.98.1 pinned in `rust-toolchain.toml`.
- [x] `[workspace.lints]`: clippy `pedantic` and `unwrap_used` warn, `unsafe_code` forbidden.
- [x] One empty library crate per phase 1 crate, each with a doc comment that states its job:
  `model`, `connector`, `connector-slack`, `connector-meetings`, `store`, `pipeline`,
  `llm-provider`, `decider`, `secret-store`, `output-obsidian`, `schedule`.
- [x] `crates/app`: the `lifefs` binary. No subcommand opens the window. `lifefs sweep` logs "no
  sources configured" and exits 0.
- [x] UI in TypeScript and Vite, no framework. tauri-specta types the commands, and debug builds
  write `ui/src/bindings.ts`.
- [x] Placeholder app icon.
- [x] JSON logs from `tracing` on stderr.

### Checks and hooks

- [x] rustfmt, taplo, biome, rumdl and typos, with meeting-capture's settings.
- [x] `cargo-deny`: permissive licences only, RustSec advisories, crates.io only, Mac and Windows
  targets. One ignore: `paste` is unmaintained, and specta 2.0.0-rc.25 depends on it.
- [x] cargo-shear finds no unused dependencies.
- [x] `tsc --noEmit` type-checks the UI.
- [x] `just check` runs every check with `-D warnings`.
- [x] lefthook: the privacy hook and format checks before each commit, clippy and `tsc` before each
  push.
- [x] Privacy hook: rejects `.jsonl` outside `fixtures/synthetic/`, and any file that names a
  domain from the gitignored `privacy-domains.txt`.
- [x] Check workflow: `just check` on macOS, clippy on Windows. Actions pinned by SHA.

### Repo files

- [x] `AGENTS.md` holds the agent rules. `CLAUDE.md` imports it and the gitignored
  `AGENTS.local.md`.
- [x] `.editorconfig` and `.gitattributes` keep LF endings on both machines.
- [x] `.gitignore` covers `fixtures/private/`, SQLite files, `.jsonl` outside the synthetic
  fixtures, and the local-only files.
- [x] `fixtures/synthetic/README.md` states the fake-data rules.
- [x] Licence files: `LICENSE-MIT` and `LICENSE-APACHE`.
- [x] Docs: no private paths, no employer policy lines, no client names.

### Done when

- [x] `just check` passes on the Mac.
- [x] `just check` passes in the Check workflow, and clippy passes on Windows.
- [x] `just dev` opens an empty window that shows the app version.
- [x] `lifefs sweep` exits 0.
- [x] `just privacy` passes over every file before the first push.
- [x] A misformatted file blocks a commit.
- [x] The public GitHub repo exists, and `main` is pushed.

## Phase 1: Slack and meeting notes on the Mac

### Before phase 1

- [x] Create the internal Slack app with `reactions:read` among its scopes. Needed for step 1.5.
- [x] Turn off "Help improve Claude" on the account. Needed for step 1.6.
- [x] Check the `search.messages` rate tier, for thread replies in quiet channels. Needed for step
  1.5.

### Step 1.1: note format by hand

No code. Fixes the note schema before any code writes notes.

- [x] Item note frontmatter: every field, its type and its closed values. `review` names the open
  question, and `same_as` holds a merge target.
- [x] `todos.base` with the open list, the follow-up list and a "Needs review" view.
- [x] Five hand-made item notes that cover a to-do, a follow-up, a closed item, an item in review
  and a dropped item.
- [x] Note names and the folder layout in the vault.

Done when:

- [x] A Bases cell edited in Obsidian on the Mac keeps every frontmatter field.

### Step 1.2: store and vault writer

- [ ] `model`: `Item`, `Kind`, `Resolution`, `Marker` and the other closed enums. Each validates
  once at entry, with an error that names the field and the valid options.
- [ ] `store`: SQLite schema and migrations.
- [ ] `store`: writer lock with `File::try_lock`, so one sweep runs at a time.
- [ ] `output-obsidian`: write item notes with yaml-edit, keeping fields and formatting it does not
  own. Write in Obsidian's style: block lists, and quotes only where YAML needs them.
- [ ] `output-obsidian`: per-field merge. A field I edited in the vault wins over the store.
- [ ] `output-obsidian`: a deleted note leaves a tombstone, and the item is never rewritten.
- [ ] Atomic note write: temp file, `fsync`, rename.
- [ ] `lifefs sweep` syncs SQLite and the notes, with no sources.

Done when:

- [ ] The hand-made items from step 1.1 round-trip through SQLite.
- [ ] A field edited in Obsidian survives the next sweep.
- [ ] A deleted note leaves a tombstone.

### Step 1.3: connector contract and meetings

- [ ] `connector`: the `Connector` trait, `Message`, `Cursor`, `Marker`, `MessageState`,
  `FetchWindow`, `FetchPage`, `AuthKind`, `AuthStatus` and `ConnectorError`.
- [ ] Upsert on source, account and external ID. An edit that removes the quoted text sends the
  item to review.
- [ ] `connector-meetings`: read action items from the `%% mc:begin %%` blocks in the meeting notes
  folder, with pulldown-cmark.
- [ ] `connector-meetings`: a ticked meeting item maps to `MarkedDone`. `recheck` re-hashes the
  block.
- [ ] The sweep runs every configured connector and records its cursor.

Done when:

- [ ] Meeting action items become items.
- [ ] A ticked meeting item closes its item.

### Step 1.4: Tauri shell and LaunchAgent

- [ ] Window: one row per source with its auth status, last sweep and item count.
- [ ] A "Sweep now" button that runs the same code as `lifefs sweep`.
- [ ] `schedule`: write the LaunchAgent plist with the `plist` crate, then load and unload it.
- [ ] The LaunchAgent runs `lifefs sweep` every 3 hours.
- [ ] Settings file: vault folder, meeting notes folder, sweep interval. Errors name the field.

Done when:

- [ ] The LaunchAgent runs for 2 days with no manual sweep.

### Step 1.5: Slack read, reactions only

- [ ] `secret-store`: read and write `secrets.toml` in the app data folder, mode 0600.
- [ ] `PastedToken` setup screen: show the app manifest, open the Slack app page, take the `xoxp-`
  token.
- [ ] `connector-slack` on slack-morphism: `auth_status` through `auth.test`.
- [ ] `fetch`: DMs, group DMs, and channels I posted in during the last 14 days. One `oldest`
  cursor per conversation.
- [ ] `thread_context` through `conversations.replies`, only for threads with a candidate or an
  open item.
- [ ] `permalink` built locally from the workspace domain, channel ID and `ts`.
- [ ] Rate limits: backon retries that honour `Retry-After`.
- [ ] `:bookmark:` maps to `MarkedTodo` and `:white_check_mark:` to `MarkedDone`. A to-do reaction
  makes an item with no LLM.

Done when:

- [ ] A `:bookmark:` reaction creates an item with a quote and a permalink.
- [ ] Auth, cursors and rate limits hold on my workspace.

### Step 1.6: extraction from Slack text

- [ ] `llm-provider`: the `CompletionProvider` trait, which asks for schema-checked JSON and never
  names a vendor.
- [ ] `claude_cli` provider on claude-codes, with every tool and MCP server off.
- [ ] jsonschema checks every reply. One repair turn on a bad reply.
- [ ] `pipeline`: rule prefilter that drops windows with no candidate.
- [ ] `pipeline`: extraction of to-dos and follow-ups, each with a verbatim quote.
- [ ] `pipeline`: quote check in code with unicode-normalization and triple_accel. A failed quote
  drops the item.
- [ ] `pipeline`: reconcile new items against open ones, so a repeat updates its item.
- [ ] A cap of 400 windows per sweep.

Done when:

- [ ] Commitments in Slack text become items with no reaction.
- [ ] Every quote passes the code check.

### Step 1.7: Jev verification

- [ ] `decider`: the `Decider` trait.
- [ ] `JevDecider` on rig-typesafeai: verify, direction and duplicate questions. Jev sees only the
  excerpt a question needs.
- [ ] `GenerativeDecider` fallback over the provider, used when Jev is down.
- [ ] Router by source privacy class. Jev runs only for sources whose `remote_vendors` names
  `typesafe`.
- [ ] Unsure items go to "Needs review". Jev never closes an item.

Done when:

- [ ] Casual talk drops out.
- [ ] Unsure items land in "Needs review".
- [ ] The fallback works with Jev turned off.

### Step 1.8: closures

- [ ] `recheck` in `connector-slack`: `reactions.get`, and `conversations.history` with `latest`
  and `inclusive`.
- [ ] A removed `:bookmark:` or an added `:white_check_mark:` closes its item.
- [ ] A deleted source message sends its item to review.
- [ ] Jev reply-outcome questions: a reply that looks like delivery becomes a closure proposal, not
  a closure.

Done when:

- [ ] A removed or done reaction closes its item within one sweep.
- [ ] Inferred closures appear as proposals, and nothing closes without explicit evidence.

### Step 1.9: labelling and soak

- [ ] `lifefs label`: show one message window at a time and write my labels as YAML in
  `fixtures/private/`.
- [ ] Label 50 windows.
- [ ] Soak: 7 days of LaunchAgent sweeps with no manual runs.

### Phase 1 done when

- [ ] The LaunchAgent sweeps for 7 days in a row with no manual runs.
- [ ] Every item's quote passes the code check against its source message.
- [ ] A field edited in Obsidian survives the next sweep.
- [ ] A removed Slack reaction closes its item within one sweep.
- [ ] 50 windows are labelled.
