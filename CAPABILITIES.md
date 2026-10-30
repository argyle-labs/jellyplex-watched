# jellyplex-watched — ServiceBackend contract

Pure-Rust plugin (**no bash/compose/provision scripts**) driven by the single
generic `service.*` surface — no per-plugin tools. Runtimes: **docker,podman,lxc,vm**.

## Per-plugin code (the only work this repo owns)
- [x] `provider` / `runtimes` / `default_port` / `capabilities` / `data_paths` — declarative descriptor
- [ ] `workload_spec(runtime)` — *what* to run; `deploy_target` renders it to a container / LXC / VM
- [ ] `configure` — rewrite the `.env` file this backend runs on (no REST API to call)
- [ ] `status` — health + rich diagnostics returned in the typed `ServiceStatus.info`

> Declarative descriptor is implemented and the plugin **registers + loads live**
> in orca today (`service.list` shows it). `workload_spec`/`configure`/`status`
> are being filled in per plugin.
>
> No listening port (`default_port` is `0`) and no `data_paths` — the service
> runs `network_mode: host` with no web UI, and its config lives in a
> host-side `.env` file injected via `env_file`, never written into the
> container filesystem.

## Provided generically by orca (NO code here)
- `deploy` — `service.deploy` → `deploy_target.launch(WorkloadSpec)`
- `backup` / `restore` — pluggable `BackupMethod` (tar; **PBS** for Proxmox guests)
- single `service.*` tool surface, exposed over CLI / REST / MCP
