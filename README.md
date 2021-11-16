# spt-utils

This is a simple program to wrap functionality implemented by [rspotify](https://github.com/ramsayleung/rspotify).

Currently allows for:

- Checking the currently playing song
- Song skipping forward/backward
- Play/pause toggle.

To build:

Run `cargo build --release`.

Generate your client_id and secret [here](https://developer.spotify.com/dashboard/applications) and store them in a file `~/.config/spt-utils/client.yml` of the form:

```yaml
---
client_id: xxxxx
client_secret: xxxxx
```

**Note: If compilation is taking too long/ is too resource intensive, remove the line `lto=true` from `Cargo.toml`. This disables dependency optimisation on compilation**

