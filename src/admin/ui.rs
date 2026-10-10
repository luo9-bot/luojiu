// Served by the admin HTTP server. Keep this wired to the single-file Vite build,
// rather than committing a second, stale copy of the frontend bundle.
pub(super) const HTML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/frontend/dist/index.html"
));
