# Collective CLI

Provides a way to create evidence reports for a Polkadot On-Chain collective. Reports are machine readable and can be rendered.

## Usage

```sh
cargo install collective
```

**Render** a YAML report that conforms to the [schema.json](./schema.json) into HTML:

```sh
collective render evidence example.evidence > example.html
```
