//! One sweep over every configured source. Phase 0 has no sources, so it only logs that.

pub fn run() {
    tracing::info!("no sources configured");
}
