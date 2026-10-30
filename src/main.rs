//! Dynamic (subprocess) entrypoint for the jellyplex-watched plugin.
//!
//! Service-backend only — deploy/backup/restore of the JellyPlex-Watched
//! instance itself. No `#[orca_tool]` surface to force-link.
plugin_toolkit::instrument::bootstrap!();
use plugin_toolkit::plugin::Plugin;
fn main() -> plugin_toolkit::anyhow::Result<()> {
    Plugin::named("jellyplex-watched")
        .version(env!("CARGO_PKG_VERSION"))
        .service(jellyplex_watched::JellyplexWatchedServiceBackend::new(
            "jellyplex-watched",
        ))
        .serve()
}
