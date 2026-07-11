# jellyplex-watched

[JellyPlex-Watched](https://github.com/luigi311/JellyPlex-Watched) syncs
watched/played state between Jellyfin and Plex libraries.

First-party [orca](https://github.com/argyle-labs/orca) plugin (service-backend).

> **Status:** deploy config + docs only. The orca service-backend implementation
> is planned; today this repo carries the self-contained deploy config so it can
> be stood up by hand or via orca.

| App | Web UI | Role |
|-----|--------|------|
| **JellyPlex-Watched** | none | Sync watched state Jellyfin ↔ Plex (scheduled) |

## Deploy

```bash
docker compose up -d
```

Runs on a schedule, no web interface — configured via a `.env` file (Jellyfin +
Plex server URLs and tokens) and observed through container logs. See
[`compose.yml`](compose.yml) and [`docs/jellyplex-watched.md`](docs/jellyplex-watched.md).
Never commit real tokens — keep them in `.env`.
