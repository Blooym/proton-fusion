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
sh -c "$(curl -fsSL https://raw.githubusercontent.com/Blooym/use-proton-latest/main/setup/install-ge-proton.sh)" ~/.steam/root/compatibilitytools.d/GE-Proton-Latest
```

#### Flatpak Steam

```sh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/Blooym/use-proton-latest/main/setup/install-ge-proton.sh)" ~/.var/app/com.valvesoftware.Steam/.steam/root/compatibilitytools.d/GE-Proton-Latest
```

#### Snap Steam (Unsupported)

```sh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/Blooym/use-proton-latest/main/setup/install-ge-proton.sh)" ~/snap/steam/common/.steam/root/compatibilitytools.d/
```