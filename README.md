# konnektoren-mini-app
Telegram Mini App for Konnektoren.

This is a reduced version of the [Konnektoren](https://konnektoren.help) app for Telegram.

## Quick Start

Using [just](https://github.com/casey/just) (recommended):
```bash
# Install dependencies (first time only)
just setup

# Start development server
just serve
```

Or with standard commands:
```bash
# Install trunk (first time only)
cargo install trunk
rustup target add wasm32-unknown-unknown

# Start development server
trunk serve
```

Visit `http://localhost:8080` in your browser.

## Development Commands

This project uses [just](https://github.com/casey/just) as a command runner. Available commands:

- `just setup` - Set up development environment
- `just serve` - Start development server
- `just build` - Build for production
- `just test` - Run all tests
- `just fmt` - Format code
- `just lint` - Run linter
- `just update` - Update dependencies

Run `just` without arguments to see all available commands.

## Build

Using just:
```bash
just build
```

Or directly:
```bash
trunk build --release
```

## Deploy

The app is deployed to GitHub Pages using GitHub Actions. The deployment configuration is in `.github/workflows/gh-pages.yml`.

To deploy the app, push to the `main` branch.

## Coin

We have a coin you can earn for learning:
[TestKonnektoren](https://testnet.tonscan.org/jetton/EQCP0BHV18JPMrt0JbSUulzcL4geZ_JrGgMZmbVv8gBk2iTe)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
