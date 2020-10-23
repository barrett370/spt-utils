# spt-utils

This is a simple program to wrap functionality implemented by [rspotify](https://github.com/ramsayleung/rspotify).

Currently allows for:

- Checking the currently playing song
- Song skipping forward/backward
- Play/pause toggle.

To build:

with `CLIENT_ID`, `CLIENT_SECRET` and `REDIRECT_URI` set in your path (set redirect URI to `http://localhost:8888/callback` run `cargo build --release`.

Generate your client_id and secret [here](https://developer.spotify.com/dashboard/applications)
