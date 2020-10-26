# spt-utils

This is a simple program to wrap functionality implemented by [rspotify](https://github.com/ramsayleung/rspotify).

Currently allows for:

- Checking the currently playing song
- Song skipping forward/backward
- Play/pause toggle.

To build:

with `CLIENT_ID` and `CLIENT_SECRET` set in your path (redirect URI set to `http://localhost:8888/callback` by default) run `cargo build --release`.

Generate your client_id and secret [here](https://developer.spotify.com/dashboard/applications)

**Note: If compilation is taking too long/ is too resource intensive, remove the line `lto=true` from `Cargo.toml`. This disables dependency optimisation on compilation**
