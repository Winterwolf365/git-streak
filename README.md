
# Git Streak

An desktop application for tracking the git commit streak on repositories.
## Installation
⚠️⚠️⚠️ Make sure you [git](https://git-scm.com/download) is installed on your system, before proceeding! ⚠️⚠️⚠️

There are official releases on the [releases page](https://github.com/Winterwolf365/git-streak/releases) for these platoms:

- Windows Installer
- Windows Portable
- Ubuntu/Debian deb
- Fedoara/REHL rpm
- MacOs Portable Script (with debug terminal)
- Linux Portable

If that doesn't work or you want to build it yourself, than install the [tauri prerequisites](https://beta.tauri.app/guides/prerequisites/) and run the following commands.

```bash
git clone https://github.com/Winterwolf365/git-streak.git
cd git-streak

cargo install tauri-cli --version '^2.0.0-beta'
cargo tauri build
```

Than you should have the installers for you system in `/src-tauri/target/release/bundle` and the portable script app should be located in `/src-tauri/target/release`.

NOTE: In windows the MSI installer didn't work, in Linux the AppImage wouldn't open, and in MacOs the .app and .dmg didn't launch when opened so for mac the only thing that worked was the portable script.
## Features

- Add and remove repositories to local sqlite database.
- Live git streak that updates every five seconds.
- Notifications every hour after 12:00(PM) if you haven't committed that day.
- Automatically launch your application at system startup.
- Settings to toggle auto-launch and notifications.
- System tray with buttons for opening, restarting and quitting Git Streak.

The application keeps running in the background if you close it, unless you manually close it trough the systems tray or something else, that's so the app can send notifications in the background.

## Screenshots

![App Screenshot](https://raw.githubusercontent.com/Winterwolf365/git-streak/main/screenshots/lunix-fedora-gtk-repositories-features.png)

More screenshots are in the `/screenshots` folder!

## Roadmap

Just some ideas, that i maybe implement in the future.

- Fix known issues.

- Add auto detect git repositories.

- Add custom icon instead of the default tauri icon.

- Make Appimage on Linux, MSI installer on Windows and on MacOs .app and .dmg work.
## Known issues
Windows:
- Auto-launch on startup gets registered in the OS, but didn't work in my testing.

MacOS:
- Auto-launch on startup didn't work.
- And when building with notifications support, i always got an error even with specific rust packages for MacOS. So they are disabled in the source code for MacOS.
