# nasa-wallpaper-dl

![Rust](https://img.shields.io/badge/built_with-Rust-dea584?logo=rust)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)

A lightweight and fast command-line tool written in Rust that scrapes the **NASA Astronomy Picture of the Day (APOD)** calendar and automatically downloads all available images for the current month.

## Features

- **Automatic Monthly Sync:** Fetches all APOD entries for the current month using `reqwest` and regex link matching.
- **Organized Storage:** Automatically creates subdirectories per month (`./images/<current_month>/`).
- **Custom Output Path:** Override the default download destination via CLI flag.

## Installation

### From Source (via Cargo)

Ensure you have the [Rust toolchain](https://rustup.rs/) installed, then clone and install the binary:

```bash
git clone [https://github.com/](https://github.com/)realbustinjieber/nasa-wallpaper-dl.git
cd nasa-wallpaper-dl
cargo install --path .
```

## Usage

### Default Download

Downloads all images for the current month into `./images/<current_month>/`:

```bash
nasa-wallpaper-dl
```

### Custom Output Directory

Use the `-p` flag to specify a custom target directory:

```bash
nasa-wallpaper-dl -p /path_to_my_folder/here/
```

## Roadmap

- [ ] Add `-m` / `--month` and `-y` / `--year` flags to download past APOD archives
- [ ] Skip already downloaded files to avoid redundant network requests
- [ ] Optional flag to automatically set the latest image as desktop wallpaper

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

> **Note:** All downloaded images are courtesy of NASA's Astronomy Picture of the Day (APOD) and remain subject to the copyright terms of their respective creators.
