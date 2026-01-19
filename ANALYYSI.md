# Mitä tekoäly teki hyvin?

- Todella hyvä tyyppiturvallisuus. Tyyppien rakenne & nimet ovat myös mielestäni "itsestään dokumentoivia". Tätä painotin alkuperäisessä promptissa.
  - Esimerkiksi `TimeSlot` joka on tehty niin, ettei virheellistä tietoa voi tulla koska kentät ovat yksityisiä ja tämän tyypin voi rakentaa vain sen omalla `new()` funktiolla, jossa on validointi:

```rust
pub struct TimeSlot {
      start: DateTime<Utc>,
      end: DateTime<Utc>,  // Huom! nämä kentät ovat yksityisiä!
  }

  impl TimeSlot {
      pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) ->
  // Palauttaa Result-tyypin joka voidaan "avata" `?` operaattorilla.
  // ? taas palauttaa joko virheen (tässä tapauksessa ValidationError::EndBeforeStart tai Ok():n sisällä olevan `TimeSlot` arvon.)
  Result<Self, ValidationError> {
          // Tässä validointi
          if end <= start {
              return Err(ValidationError::EndBeforeStart);
          }
          Ok(Self { start, end })
      }
  }
```

- Hyvä virheenhallinta käyttäen `thiserror` laatikkoa. Verboosit virheet.
- Todella helposti ymmärrettävä ja hyvin jaoteltu rakenne. Koodi on jaettu selvästi omiin paketteihin ominaisuuden mukaan, kuten pyysin. Vaikka en sitä erikseen maininnut promptissa, niin tekoäly myös teki perinteisen tasojaon ominaisuuksien sisällä, joka mielestäni helpottaa skaalausta ja koodin ymmärrettävyyttä. Uusien ominaisuuksien lisääminen ei vaadi tasoilta toiselle hyppimistä koko repon sisällä, kuten jos koko tuote olisi jaettu tasojen mukaan.
- Tämä eka versio oli lähes "one shot". Tässä toki auttaa paljolti se, että käytin aluksi suunnittelu-moodia. Suunnitelman toteutuksen aikana ei tarvittu kuin muutama ohjausliike.

# Mitä tekoäly teki huonosti?

- Tekoäly teki syystä tai toisesta vain yhteen moduuliin yksikkötestit. En toki pyytänyt testejä erikseen, mutta en ihan ymmärrä miksi se teki ne yhteen moduuliin, mutta ei muihin. Alkuperäinen testikattavuus:

```
2026-01-19T09:19:39.645840Z  INFO cargo_tarpaulin::report: Coverage Results:
|| Uncovered Lines:
|| src/common/error.rs: 53-54, 56, 59, 62, 65, 68, 71, 73-74, 77-79, 82, 87-89, 93-94
|| src/common/time.rs: 19-21, 23, 26-27, 30-31
|| src/db/mod.rs: 8-10
|| src/main.rs: 9-10, 12-13, 15-19, 21-22, 25, 27-28
|| src/reservation/handlers.rs: 14-15, 18, 22, 26, 32-36, 39-40, 43, 47-52, 56, 61, 63-65, 69-70
|| src/reservation/repository.rs: 11, 15, 18-19, 21-29, 31-32, 36-37, 40, 43-47, 50, 54, 56-60, 63, 67, 69-74, 77, 81, 84-88, 91-94, 97-99
|| src/reservation/types.rs: 16-17, 22-23, 37-40, 48-52, 71-72, 75-76, 79-80, 83-84, 87-88, 92-93, 126, 128-134
|| src/reservation/validation.rs: 16, 24-25, 29-30, 34, 36-39
|| src/room/handlers.rs: 11-14, 17, 21-22, 25-27
|| src/room/repository.rs: 9-10, 12, 14-18, 21-22, 24-28, 31-32, 34-37, 40-41, 46-48
|| src/room/types.rs: 11-12, 17-18, 33-34, 60, 62-65
|| src/user/handlers.rs: 11-14, 17, 21-22, 25, 29-30
|| src/user/repository.rs: 9-10, 12, 14-18, 21-22, 24-28, 31-32, 37-39
|| src/user/types.rs: 11-12, 17-18, 33-34, 60, 62-65
|| Tested/Total Lines:
|| src/common/error.rs: 0/19
|| src/common/time.rs: 6/14
|| src/db/mod.rs: 0/3
|| src/main.rs: 0/14
|| src/reservation/handlers.rs: 0/26
|| src/reservation/repository.rs: 0/52
|| src/reservation/types.rs: 0/33
|| src/reservation/validation.rs: 0/10
|| src/room/handlers.rs: 0/10
|| src/room/repository.rs: 0/26
|| src/room/types.rs: 0/11
|| src/user/handlers.rs: 0/10
|| src/user/repository.rs: 0/20
|| src/user/types.rs: 0/11
||
2.32% coverage, 6/259 lines covered
```

- Erityisesti rustin kanssa tekoäly tykkää usein tehdä täysin omia implementaatiota yksinkertaisille asioille (kuten `from_str` ). Nämä usein sotkeentuvat standardikirjaston implementaatioden kanssa, koska niissä käytetään samoja funktionimiä.
- Syötteelle ei ole validointia (esim. sähköpostiosoitteet ja huoneen kapasiteetti)
  - Esimerkiksi:

  ```sh
  ❯ curl -X POST localhost:3000/users -H "Content-Type: application/json" -d '{"email":"xx","name":"asdfr"}'
  {"id":2,"email":"xx","name":"asdfr","created_at":"2026-01-19T10:01:59+00:00"}%

  ❯ curl -X POST localhost:3000/rooms -H "Content-Type: application/json" -d '{"name":"Conference A","capacity":10568609}'
  {"id":2,"name":"Conference A","capacity":10568609,"created_at":"2026-01-19T10:04:13+00:00"}%  

  ```

- Tekoäly ei tehnyt mitään CI ratkaisua, toisaalta tätä en myöskään erikseen pyytänyt.
- Tekoäly käyttää lähes poikkeuksetta vanhoja versioita joistakin laatikoista (kirjastoista).

# Mitkä olivat tärkeimmät parannukset, jotka teit tekoälyn tuottamaa koodiin ja miksi?
