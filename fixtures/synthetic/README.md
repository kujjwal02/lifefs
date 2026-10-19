# Synthetic fixtures

Made-up message windows for live checks and, later, tests. Real messages and the labelled set
live in `fixtures/private/`, which is gitignored.

- Every person, company, email address and channel name is invented.
- Email addresses use `example.com`, `example.org` or `example.net`.
- No text is copied from a real message, even with names changed.
- `.jsonl` files are allowed only in this folder. The pre-commit privacy hook rejects them
  anywhere else.
