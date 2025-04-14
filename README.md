# use-proton-latest

Automatically update custom proton installations at launch time via a Steam compatibility tool.

## Disclaimers

- **When Proton is installed or being automatically updated your game will take longer than usual to launch.** This is normal and depends entirely on your internet speed.
- **Updating a proton prefix is a potentially breaking action.** While upgrading is safer than downgrading versions, it may still break things depending on the proton version and the changes made between them, be cautious.

## Setup

Install scripts have been created to automatically handle installation for you. Below you can pick between different proton versions to install.

After installing any of the following, you should:
- Restart your Steam/Steamdeck to refresh available Steam compatibility tools
- Go to the game of your choice -> Properties -> Compatibility
- Select "ProtonName-Latest" from the dropdown menu.

### GE-Proton

#### Native Steam & Steamdeck

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/use-proton-latest/main/setup/install-ge-proton.sh | sh -s ~/.steam/root/compatibilitytools.d/GE-Proton-Latest
```

#### Flatpak Steam

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/use-proton-latest/main/setup/install-ge-proton.sh | sh -s ~/.var/app/com.valvesoftware.Steam/.steam/root/compatibilitytools.d/GE-Proton-Latest
```

#### Snap Steam (Unsupported)

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/use-proton-latest/main/setup/install-ge-proton.sh | sh -s ~/snap/steam/common/.steam/root/compatibilitytools.d/
```

## Optional Extras

These are additional features or functionality that can be enabled by setting environment variables in a game's launch arguments. They are supported by all custom proton versions offered by this tool.

| Name               | Description                                                                                                                                                            | Enablement Variable   | Supported from   |
|--------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------|----------------- |
| Discord RPC Bridge | Automatically run `0e4ef622/wine-discord-ipc-bridge` when launching a game. This allows most Windows games to communicate with Discord and display rich presence data. | `DISCORDRPC_BRIDGE=1` | v0.2.0           |


