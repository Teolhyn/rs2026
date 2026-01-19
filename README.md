## Projektin rakenne

.
├── ANALYYSI.md // Oma analyysi
├── PROMPTIT.md // Käytetyt promptit
└── src
    ├── common // Yhteinen moduuli
    │   ├── error.rs
    │   ├── mod.rs
    │   └── time.rs
    ├── db // Tietokantamoduuli
    │   ├── mod.rs
    │   └── schema.rs
    ├── lib.rs
    ├── main.rs
    ├── reservation
    │   ├── handlers.rs // endpointit
    │   ├── mod.rs
    │   ├── repository.rs // logiikka
    │   ├── types.rs // tyypit
    │   └── validation.rs
    ├── room
    │   ├── handlers.rs
    │   ├── mod.rs
    │   ├── repository.rs
    │   └── types.rs
    └── user
        ├── handlers.rs
        ├── mod.rs
        ├── repository.rs
        └── types.rs

# Ohjeet käynnistykseen

Jos sinulla ei ole Rustia:

### Asenna `rustup` ja jatka ohjeiden mukaan

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Asenna `diesel-cli`

```sh
cargo install diesel_cli --no-default-features --features sqlite

```

### luo .env

```sh
cp .env.example .env
```

### Aja migraatiot

```sh
diesel migration run
```

### Käynnistä ohjelma

```sh
cargo run
```

### Käynnistä ohjelma (release optimoitu)

```sh
cargo run --release
```
