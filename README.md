# GProfiles

An objectively better way to manage your Logitech GHUB gaming profiles.

This application was made with the intent of complementing the already existing [Logitech G HUB](https://www.logitechg.com/en-us/innovation/g-hub.html).

# Installation

## Recommended

Download the latest executable from the [release files](https://github.com/Lee-matod/GProfiles/releases).  
No installer. No UAC. Just plug-and-play.

## Do It Yourself!

If you wish to build the application yourself, follow the steps below.

### Requirements

- Windows OS
- Rust: https://www.rust-lang.org/tools/install
- Git: https://git-scm.com/downloads

Clone this repository and set it as your current working directory.

```cmd
git clone https://github.com/Lee-matod/GProfiles
cd GProfiles
```

Build the binary as a release.

```cmd
cargo build --release
```

After Cargo has finished building, run the application with:

```cmd
cargo run --release
```

You can also move or create a shortcut to the `GProfiles.exe` executable file created under `/target/release`.

# Features

- Simple and minimalistic design. One window with all features right in front of your eyes.
- Intuitive running processes detection.
- Easy profile customization (rename profiles, change icons, and more).
- Built-in LGHUB restart button to apply changes.
- Native executable icon transparency.
- Per-application keyboard key remapping.

# FAQ

### Where can I find my profiles/keymaps?

GProfiles stores application keymaps in `settings.db`, the same file where Logitech GHUB stores game profiles. You can create a copy/backup of it, and then replace it in the future. Both GProfiles and Logitech GHUB should work without any additional hassle.  
You can locate this file by searching for `%LOCALAPPDATA%/LGHUB` in the File Explorer or the Run application.

# Disclaimers

This project and its developer(s) are not affiliated in any way with Logitech International S.A.

# Acknowledgements

- Fonts and Material Icons from [Google Fonts](https://fonts.google.com).
- This project uses [slint](https://slint.dev/) as its UI framework.

<a href="https://github.com/slint-ui/slint">
    <img src="https://github.com/slint-ui/slint/blob/master/logo/MadeWithSlint-logo-dark.png?raw=true" alt="https://slint.dev/" width=150 />
</a>
