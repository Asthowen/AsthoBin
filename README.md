<!--suppress HtmlDeprecatedAttribute -->
<h1 align="center">
  AsthoBin
</h1>
<p align="center">
    <a href="https://www.rust-lang.org/">
        <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust">
    </a>
    <a href="https://github.com/Asthowen/AsthoBin">
        <img src="https://img.shields.io/badge/Git-F05032?style=for-the-badge&logo=git&logoColor=white" alt="Use git">
    </a>
    <br>
    <a href="https://github.com/Asthowen/AsthoBin/blob/main/LICENSE">
        <img src="https://img.shields.io/github/license/Asthowen/AsthoBin?style=for-the-badge" alt="License">
    </a>
    <a href="https://github.com/Asthowen/AsthoBin/stargazers">
        <img src="https://img.shields.io/github/stars/Asthowen/AsthoBin?style=for-the-badge" alt="Stars">
    </a>
</p>
<h3 align="center">
    <strong>AsthoBin is a simple website to share pieces of code with a URL, written in Rust.</strong>
</h3>

## Features
* Lightweight < 5MB
* Low RAM consumption
* Responsive UI
* Code coloring with [highlight.js](https://highlightjs.org/)
* Automatic deletion after custom time

## TODO
- [ ] GitHub actions
- [ ] Unit tests
- [ ] Docker image
- [ ] Support of PostgreSQL

## Installation
### Docker
Start by cloning the repo:
```bash
git clone https://github.com/Asthowen/AsthoBin.git
```

Now switch to project folder and build the container with docker-compose:
```bash
cd AsthoBin && docker-compose -f ./docker/docker-compose.yml up -d --build
```

### Manually
Start by cloning the repo:
```bash
git clone https://github.com/Asthowen/AsthoBin.git
```
**For the next step you need to have Rust and cargo installed on your PC, for that follow the [official documentation](https://www.rust-lang.org/tools/install).**

Now switch to project folder and compile a release:
```bash
cd AsthoBin && cargo build --release
```

Your executable will be in the `target/release/` folder, it is named `asthonbin`.

## Contributors
[<img width="45" src="https://avatars.githubusercontent.com/u/59535754?v=4" alt="Asthowen">](https://github.com/Asthowen)

## License
**[AsthoBin](https://github.com/Asthowen/AsthoBin) | [GNU General Public License v3.0](https://github.com/Asthowen/AsthoBin/blob/main/LICENSE)**