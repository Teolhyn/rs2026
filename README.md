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

### API

Palvelin käynnistyy osoitteeseen `http://localhost:3000`.

#### Käyttäjät (Users)

**Luo käyttäjä**
```sh
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "name": "Testaaja"}'
```

**Hae käyttäjä**
```sh
curl http://localhost:3000/users/1
```

#### Huoneet (Rooms)

**Luo huone**
```sh
curl -X POST http://localhost:3000/rooms \
  -H "Content-Type: application/json" \
  -d '{"name": "Neuvotteluhuone A", "capacity": 10}'
```

**Listaa huoneet**
```sh
curl http://localhost:3000/rooms
```

**Listaa huoneet kapasiteetin mukaan**
```sh
curl "http://localhost:3000/rooms?min_capacity=5&max_capacity=20"
```

#### Varaukset (Reservations)

**Luo varaus**
```sh
curl -X POST http://localhost:3000/rooms/1/reservations \
  -H "Content-Type: application/json" \
  -d '{"user_id": 1, "start_time": "2026-01-21T14:00:00Z", "end_time": "2026-01-21T15:00:00Z"}'
```

**Listaa huoneen varaukset**
```sh
curl http://localhost:3000/rooms/1/reservations
```

**Peruuta varaus**
```sh
curl -X DELETE http://localhost:3000/rooms/1/reservations/1
```
