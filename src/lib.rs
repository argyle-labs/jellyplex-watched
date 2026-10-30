//! JellyPlex-Watched's `ServiceBackend` — deploy/backup/restore of the
//! JellyPlex-Watched instance itself. JellyPlex-Watched has no REST API and
//! no web UI (a scheduled sync daemon run with `network_mode: host`,
//! configured via a host-side `.env` file), so this plugin carries no typed
//! API client and no `#[orca_tool]` surface — service-backend only.

use plugin_toolkit::service::{
    BoxFuture, Routes, Runtime, ServiceBackend, ServiceCapability, ServiceError, ServiceStatus,
    WorkloadSpec,
};

#[derive(Debug, Clone)]
pub struct JellyplexWatchedServiceBackend {
    provider: &'static str,
}

impl JellyplexWatchedServiceBackend {
    pub fn new(provider: &'static str) -> Self {
        Self { provider }
    }
}

impl ServiceBackend for JellyplexWatchedServiceBackend {
    fn provider(&self) -> &str {
        self.provider
    }

    fn runtimes(&self) -> Vec<Runtime> {
        vec![Runtime::Docker, Runtime::Podman, Runtime::Lxc, Runtime::Vm]
    }

    fn capabilities(&self) -> Vec<ServiceCapability> {
        vec![
            ServiceCapability::Deploy,
            ServiceCapability::Backup,
            ServiceCapability::Restore,
            ServiceCapability::Configure,
            ServiceCapability::Status,
        ]
    }

    /// No listening port: this service runs `network_mode: host` and exposes
    /// no web UI or API to reach. `0` signals "no port" — nothing in the
    /// `ServiceBackend` contract reads `default_port` outside its own
    /// `endpoint()`, which this backend leaves at its empty default.
    fn default_port(&self) -> u16 {
        0
    }

    /// No in-container config state to tar: the compose mount is
    /// `./logs:/app/logs` (transient operational output, not restore-
    /// sufficient), and the actual config — Jellyfin/Plex URLs and tokens —
    /// lives in a host-side `.env` file injected via `env_file` at container
    /// creation, never written into the container filesystem. Empty means
    /// the generic tar-based backup is unavailable; `backup`/`restore`
    /// would need overriding to capture the `.env` file if that's wanted.
    fn data_paths(&self) -> Vec<String> {
        Vec::new()
    }

    fn workload_spec<'a>(
        &'a self,
        _runtime: Runtime,
        _instance: &'a str,
        _routes: &'a Routes,
    ) -> BoxFuture<'a, Result<WorkloadSpec, ServiceError>> {
        Box::pin(async move {
            Err(ServiceError::unimplemented(
                "jellyplex-watched.workload_spec",
            ))
        })
    }

    /// Configuring this backend means rewriting its `.env` file, not calling
    /// a REST API — there is none. Declared as a capability anyway since
    /// env-file config is still a config surface this backend will own.
    fn configure<'a>(
        &'a self,
        _instance: &'a str,
        _routes: &'a Routes,
        _config: &'a str,
    ) -> BoxFuture<'a, Result<(), ServiceError>> {
        Box::pin(async move { Err(ServiceError::unimplemented("jellyplex-watched.configure")) })
    }

    fn status<'a>(
        &'a self,
        _instance: &'a str,
        _routes: &'a Routes,
    ) -> BoxFuture<'a, Result<ServiceStatus, ServiceError>> {
        Box::pin(async move { Err(ServiceError::unimplemented("jellyplex-watched.status")) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_backend_declares_no_port_no_data_paths() {
        let b = JellyplexWatchedServiceBackend::new("jellyplex-watched");
        assert_eq!(b.provider(), "jellyplex-watched");
        assert_eq!(b.default_port(), 0);
        assert_eq!(b.data_paths(), Vec::<String>::new());
    }
}
