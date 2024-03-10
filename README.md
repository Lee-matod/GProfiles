# GProfiles

An objectively better way to manage custom Logitech GHUB gaming profiles.

# Installation

Compile this application from its source code by following these steps:

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

# Features

- Running processes detection.
- Create a new profile from an executable.
- Rename profiles.
- Change the image of profiles.
- Change the top-level executable path of profiles.
- Delete profiles.
- Built-in LGHUB restart button to apply changes.

# To-do

While I personally consider this project as finished, there are a couple of additional feature I would like to include in the future.
These are, in order of priority:

1. Write an install script so GProfiles is easier to install on the user-end.
2. Remove `processes` as a dependency and just use `sysinfo`.
3. When creating a new profile, use its icon image as the default poster, rather than a preset image.
4. Convert a custom application to an emulated installed application.
   - This would allow a banner to be used instead of a `.bmp` image for the profile.
   - An identifier should be placed in the JSON blob to ensure editing is allowed for future instances.
5. Automatically scan for applications.
   - Logitech GHUB already includes this feature. However, it has a reputation for not properly detecting all installed games.

# Disclaimers

This project and its developer(s) are not affiliated in any way with Logitech International S.A.

# Acknowledgements

- Fonts and Material Icons from [Google Font](https://fonts.google.com).
- This project uses [slint](https://slint.dev/) as its UI framework.

<a href="https://github.com/slint-ui/slint">
    <img src="https://github.com/slint-ui/slint/blob/master/logo/MadeWithSlint-logo-dark.png?raw=true" alt="https://slint.dev/" width=150 />
</a>
