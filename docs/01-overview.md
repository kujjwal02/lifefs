# Overview

## Problem

My commitments and context are spread across personal Gmail, work Gmail, Slack, WhatsApp and
meetings. My agents add more: chats with Claude Code and Codex, and the work they did. No product
reads all of these and keeps one ordered record. About 40 products were screened, and none closes
items from evidence across all channels.

## Goals

- One list of what I owe people (to-dos) and what people owe me (follow-ups), kept current without my
  input.
- Close finished items from evidence, such as a reply, a sent file or a ticked meeting item.
- Track follow-ups by person and by topic, with chase dates set by code.
- Keep everything in the vault, so the Mac, the Windows machine and the phone all see it.
- A memory bank that I and my agents can query: what happened, who said what, what got done.
- Run on my Claude subscription through `claude -p`, with an API key as the fallback.

## Non-goals

- lifefs never sends a message, an email or a reply. It reads and writes notes only.
- No hosted service and no server. Each laptop runs the app locally.
- No WhatsApp bridge (whatsmeow, Baileys). It reads the Desktop app's local database only.
- Personal use first. Choices must not block a public release, but release details wait.

## Modules

| Module | Job | State |
|---|---|---|
| Owed | To-dos and follow-ups from mail, Slack, WhatsApp and meetings | Researched |
| Capture | Meeting recording, transcripts and action items | A separate app, merging in later |
| Memory bank | Stores and answers questions about past context, including agent chats and agent work | Not researched |
| Offline discussions | Conversations outside any app | Much later |
