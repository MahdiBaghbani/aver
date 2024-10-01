For example, you might run a file-hosting service where the user has to authenticate to download their files, but the files themselves are served by a separate, stateless "download server". In this case, you might want to have your application server (Server A) issue single-use "download tokens", that the client can then use to download the file from a download server (Server B).

When using JWT in this manner, there are a few specific properties:

The tokens are short-lived. They only need to be valid for a few minutes, to allow a client to initiate the download.
The token is only expected to be used once. The application server would issue a new token for every download, so any one token is just used to request a file once, and then thrown away. There's no persistent state, at all.
The application server still uses sessions. It's just the download server that uses tokens to authorize individual downloads, because it doesn't need persistent state.