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

<details>
<summary>Install Scripts</summary>

#### Native Steam & Steamdeck

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s GE-Proton-Fusion ~/.steam/root/compatibilitytools.d/GE-Proton-Fusion
```

#### Flatpak Steam

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s GE-Proton-Fusion ~/.var/app/com.valvesoftware.Steam/.steam/root/compatibilitytools.d/GE-Proton-Fusion
```

#### Snap Steam (Unsupported)

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s GE-Proton-Fusion ~/snap/steam/common/.steam/root/compatibilitytools.d/GE-Proton-Fusion
```
</details>

### Proton-Tkg-Valvebe-Fusion

<details>
<summary>Install Scripts</summary>

#### Native Steam & Steamdeck

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s Proton-Tkg-Valvebe-Fusion ~/.steam/root/compatibilitytools.d/Proton-Tkg-Valvebe-Fusion
```

#### Flatpak Steam

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s Proton-Tkg-Valvebe-Fusion ~/.var/app/com.valvesoftware.Steam/.steam/root/compatibilitytools.d/Proton-Tkg-Valvebe-Fusion
```

#### Snap Steam (Unsupported)

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s Proton-Tkg-Valvebe-Fusion ~/snap/steam/common/.steam/root/compatibilitytools.d/Proton-Tkg-Valvebe-Fusion
```

</details>

### Proton-Tkg-Wine-Fusion

<details>
<summary>Install Scripts</summary>

#### Native Steam & Steamdeck

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s Proton-Tkg-Wine-Fusion ~/.steam/root/compatibilitytools.d/Proton-Tkg-Wine-Fusion
```

#### Flatpak Steam

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s Proton-Tkg-Wine-Fusion ~/.var/app/com.valvesoftware.Steam/.steam/root/compatibilitytools.d/Proton-Tkg-Wine-Fusion
```

#### Snap Steam (Unsupported)

```sh
curl -fsSL https://raw.githubusercontent.com/Blooym/proton-fusion/main/install.sh | sh -s Proton-Tkg-Wine-Fusion ~/snap/steam/common/.steam/root/compatibilitytools.d/Proton-Tkg-Wine-Fusion
```

</details>

## Optional Extras

These are additional features or functionality that can be enabled by setting environment variables in a game's launch arguments. They are supported by all custom proton versions offered by this tool.

| Name               | Description                                                                                                                                                                                                                              | Enablement Variable   |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------- |
| Discord RPC Bridge | Automatically run `0e4ef622/wine-discord-ipc-bridge` when launching a game. This allows most Windows games to communicate with Discord and display rich presence data. Note that this may cause launching to be a bit finnicky at times. | `DISCORDRPC_BRIDGE=1` |

## Building from source

If you want to build a specific steam tool from source please view the [build script](./build.sh) for more information.