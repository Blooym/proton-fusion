# proton-fusion

Automatically updating custom proton builds with a little extra on top.

## Disclaimers

- **When Proton is being installed or automatically updated your game will take longer than usual to launch.** This is normal and depends entirely on your internet speed.
- **Automatically updating a proton prefix is a potentially breaking action.** While upgrading is safer than downgrading versions, it may still break things depending on the proton version and the changes made between them, be aware of this when using proton-fusion.
- **Using the provided [optional extras](#optional-extras) may break things.** Always be careful with what you choose to enable.
- **Do not report bugs about proton versions here to upstreams** unless you know, for certain, that the bug lies there.

## Setup

Install scripts have been created to automatically handle installation for you; Below you can pick between different proton versions to install. **After installing any of the following, you should:**
- Restart your Steam/Steamdeck to refresh available Steam compatibility tools
- Go to the game of your choice -> Properties -> Compatibility
- Select the proton version you just downloaded from the dropdown menu.

### GE-Proton-Fusion

#### Native Steam & Steamdeck

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install-ge-proton.sh | sh -s ~/.steam/root/compatibilitytools.d/GE-Proton-Fusion
```

#### Flatpak Steam

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install-ge-proton.sh | sh -s ~/.var/app/com.valvesoftware.Steam/.steam/root/compatibilitytools.d/GE-Proton-Latest
```

#### Snap Steam (Unsupported)

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install-ge-proton.sh | sh -s ~/snap/steam/common/.steam/root/compatibilitytools.d/GE-Proton-Latest
```

## Optional Extras

These are additional features or functionality that can be enabled by setting environment variables in a game's launch arguments. They are supported by all custom proton versions offered by this tool.

| Name               | Description                                                                                                                                                            | Enablement Variable   | Supported from |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- | -------------- |
| Discord RPC Bridge | Automatically run `0e4ef622/wine-discord-ipc-bridge` when launching a game. This allows most Windows games to communicate with Discord and display rich presence data. | `DISCORDRPC_BRIDGE=1` | v0.1.0         |
