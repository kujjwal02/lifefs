set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

# Every check. The check workflow runs the same recipe on each push.
check: fmt-check lint typecheck deny shear

# Rewrite files in place.
fmt:
    cargo fmt --all
    taplo fmt
    biome check --write
    rumdl fmt

fmt-check:
    cargo fmt --all -- --check
    taplo fmt --check
    biome check
    rumdl check

lint:
    typos
    cargo clippy --workspace --all-targets --locked -- -D warnings

typecheck:
    bun --bun run typecheck

deny:
    cargo deny --locked check

shear:
    cargo shear

# Scan every file git would commit with the privacy hook. Needs privacy-domains.txt (gitignored).
privacy:
    git ls-files -z --cached --others --exclude-standard | xargs -0 scripts/privacy-check.sh

# Install the git hooks and the UI packages once per clone.
setup:
    lefthook install
    bun install

# Open the window in dev mode. Debug builds rewrite crates/app/ui/src/bindings.ts.
dev:
    cd crates/app && cargo tauri dev

# Run one sweep from a debug build.
sweep:
    cargo run -p lifefs -- sweep
