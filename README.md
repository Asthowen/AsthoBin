<!--suppress HtmlDeprecatedAttribute -->
<div align="center">
    <h1>AsthoBin</h1>
    <p>
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
    <h3>
        <strong>AsthoBin is a simple website to share pieces of code with a URL, written in Rust.</strong>
    </h3>
</div>

## Features
* Lightweight < 5MB
* Low RAM consumption
* Responsive UI
* Code coloring with [highlight.js](https://highlightjs.org/)
* Automatic deletion after custom time
* Recovery of files raws

## TODO
- [x] GitHub actions
- [ ] Add code documentation
- [ ] Unit tests
- [x] Docker image

## Installation
### Docker
Start by cloning the repo:
```bash
git clone https://github.com/Asthowen/AsthoBin.git && cd AsthoBin
```

After that, create a database with the name you want, then edit `.env` config file, for this please refer to [configuration](#configuration).


And finally, run Docker container (**do not forget to change the two ports, one for AsthoBin and the other for your SQL database**):
```bash
docker run -d \
  --name=asthobin \
  -p 8080:8080 \
  -p 3306:3306 \
  --restart unless-stopped \
  --env-file .env \
  asthowen/asthobin:latest
```
You can also use docker-compose with the [`docker/docker-compose.yml`](https://github.com/Asthowen/AsthoBin/blob/main/docker/docker-compose.yml) file.

### Manually
Start by cloning the repo:
```bash
git clone https://github.com/Asthowen/AsthoBin.git && cd AsthoBin
```
**For the next step you need to have Rust and Cargo installed on your PC, for that follow the [official documentation](https://www.rust-lang.org/tools/install).**

Install diesel-cli:
```bash
cargo install diesel_cli --no-default-features --features mysql
```

Now compile a release:
```bash
cargo build --release
```

Your executable will be in the `target/release/` folder, it is named `asthobin`.

Compile CSS (with pnpm):
```bash
pnpm run prod-pnpm
```
or with npm:
```bash
npm run prod
```

## Configuration
To configure **AsthoBin**, just use the example configuration: [`.env.example`](https://github.com/Asthowen/AsthoBin/blob/main/.env.example), you just have to rename it to `.env` and complete it.

### List of variables

| Key                             | Default                | Description                                                         |
|:--------------------------------|:-----------------------|:--------------------------------------------------------------------|
| **HOST**                        | 127.0.0.1              | The desired hostname to launch AsthoBin.                            |
| **PORT**                        | 8080                   | The desired port to launch AsthoBin.                                |
| **DATABASE_URL**                | **Nothing (required)** | The URL of your database.                                           |
| **BASE_URL**                    | **Nothing (required)** | The URL at which your AsthoBin instance is accessible.              |
| **CORS_ORIGIN**                 | *                      | CORS parameters.                                                    |
| **LOG_ON_ACCESS**               | false                  | Display a log when a user access to a file.                         |
| **LOG_ON_SAVE**                 | false                  | Display a log when a user creates a file.                           |
| **RATELIMIT_BETWEEN_SAVE**      | 2                      | Number of seconds between each file save.                           |
| **RATELIMIT_ALLOWED_BEFORE**    | 4                      | Number of requests before blocking.                                 |
| **ACTIX_WORKER_THREADS_NUMBER** | 8                      | The number of threads used by Actix.                                |
| **TZ**                          | System value           | The time zone of the logger, e.g: `Europe/Paris`.                   |
| **HTTP_PRIVATE_KEY**            | **Nothing (optional)** | The file path of SSL certificate private key.                       |
| **HTTP_CERTIFICATE_CHAIN**      | **Nothing (optional)** | The file path of SSL certificate chain.                             |
| **SSL_FILE_TYPE**               | PEM                    | The private key algorithm(PEM/ASN1).                                |
| **SSL_CA_FILE**                 | **Nothing (optional)** | The file path of SSL certificate authority.                         |
| **SSL_PROTOCOL_MIN_VERSION**    | **Nothing (optional)** | The minimum value of SSL protocol (ssl3/tls1/tls1.1/tls1.2/tls1.3). |
| **SSL_PROTOCOL_MAX_VERSION**    | **Nothing (optional)** | The maximum value of SSL protocol (ssl3/tls1/tls1.1/tls1.2/tls1.3). |

## Development
### Before submit a PR
**You must make sure that clippy (`cargo clippy`) does not return any errors/warning. You must also run `cargo fmt`.**

### Versioning
**This project uses semantic versioning, which has the format: MAJOR.MINOR.PATCH with:**
* `MAJOR` version when you make incompatible API changes.
* `MINOR` version when you add functionality in a backwards compatible manner.
* `PATCH` version when you make backwards compatible bug fixes.

## Contributors
[<img width="45" src="https://avatars.githubusercontent.com/u/59535754?v=4" alt="Asthowen">](https://github.com/Asthowen)

## License
**[AsthoBin](https://github.com/Asthowen/AsthoBin) | [GNU General Public License v3.0](https://github.com/Asthowen/AsthoBin/blob/main/LICENSE)**