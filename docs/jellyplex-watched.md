# JellyPlex-Watched

Syncs watched/played state between Jellyfin and Plex.

- **Runtime:** Docker (`network_mode: host`)
- **Web UI:** none (no port)
- **Config:** `.env` file (Jellyfin + Plex server URLs and tokens)

## Notes

Runs on a schedule to keep watched state in sync between the Jellyfin and Plex
libraries. No web interface — configured via environment / `.env` file and
observed through container logs (`./logs`).

Point it at both media servers via `.env`; it reconciles play state in both
directions on each run. Tokens are secrets — keep them in `.env`, never in the
committed compose.
