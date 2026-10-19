#!/bin/sh
# Rejects files that could leak real messages: .jsonl outside fixtures/synthetic/, and any file
# that names one of my real email domains. The domains live in privacy-domains.txt, one per line,
# which is gitignored so the domains never enter the repo.
# Usage: scripts/privacy-check.sh <file>...
set -eu

domains_file=privacy-domains.txt
if [ ! -s "$domains_file" ]; then
  echo "privacy check: $domains_file is missing or empty. Add your real email domains, one per line." >&2
  exit 1
fi

failed=0
for file in "$@"; do
  [ -f "$file" ] || continue
  case "$file" in
    fixtures/synthetic/*) ;;
    *.jsonl)
      echo "privacy check: $file is .jsonl outside fixtures/synthetic/" >&2
      failed=1
      ;;
  esac
  if grep -I -i -n -F -f "$domains_file" "$file" >/dev/null 2>&1; then
    echo "privacy check: $file names a real email domain" >&2
    failed=1
  fi
done
exit "$failed"
