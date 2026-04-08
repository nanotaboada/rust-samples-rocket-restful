# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `UNKNOWN_PLAYER_ID` constant in `tests/common/mod.rs` for valid-UUID-absent-from-DB 404 scenarios (#69)
- `unknown` test cases for GET `/players/{uuid}`, PUT and DELETE `/players/squadnumber/{n}` in `player_routes_tests.rs` (#69)
- `unknown` test case for `get_by_id` at the service layer in `player_service_tests.rs` (#69)
- `///` doc comments on all three player ID constants in `tests/common/mod.rs` documenting the `existing` / `nonexistent` / `unknown` vocabulary (#69)

### Changed

### Fixed

### Removed

## [1.0.0 - Agüero] - 2026-04-06

Initial release. See [README.md](README.md) for complete feature list and documentation.

---

## Ballon d'Or Nominees 🏅

Release codenames follow an A-Z sequence using Ballon d'Or award nominees surnames:

| # | Tag Name | Player | Country | Notable Year |
|---|----------|--------|---------|--------------|
| A | `aguero` | Sergio Agüero | Argentina | 2011 |
| B | `benzema` | Karim Benzema | France | 2022 (Winner) |
| C | `cannavaro` | Fabio Cannavaro | Italy | 2006 (Winner) |
| D | `drogba` | Didier Drogba | Ivory Coast | 2010 |
| E | `etoo` | Samuel Eto'o | Cameroon | 2005, 2006 |
| F | `figo` | Luís Figo | Portugal | 2000 (Winner) |
| G | `griezmann` | Antoine Griezmann | France | 2016, 2018 |
| H | `haaland` | Erling Haaland | Norway | 2023 |
| I | `iniesta` | Andrés Iniesta | Spain | 2010, 2012 |
| J | `jorginho` | Jorginho | Italy | 2021 |
| K | `kaka` | Kaká | Brazil | 2007 (Winner) |
| L | `lewandowski` | Robert Lewandowski | Poland | 2020, 2021 |
| M | `messi` | Lionel Messi | Argentina | 8× Winner |
| N | `neymar` | Neymar | Brazil | 2015, 2017 |
| O | `owen` | Michael Owen | England | 2001 (Winner) |
| P | `pirlo` | Andrea Pirlo | Italy | 2007, 2011 |
| Q | `quaresma` | Ricardo Quaresma | Portugal | 2008 |
| R | `ronaldo` | Cristiano Ronaldo | Portugal | 5× Winner |
| S | `salah` | Mohamed Salah | Egypt | 2018, 2019 |
| T | `torres` | Fernando Torres | Spain | 2008 |
| U | `umtiti` | Samuel Umtiti | France | 2018 |
| V | `vandijk` | Virgil van Dijk | Netherlands | 2019 |
| W | `weah` | George Weah | Liberia | 1995 (Winner) |
| X | `xavi` | Xavi | Spain | 2009, 2010, 2011 |
| Y | `yayatoure` | Yaya Touré | Ivory Coast | 2011, 2013 |
| Z | `zlatan` | Zlatan Ibrahimović | Sweden | 2013, 2015 |

[unreleased]: https://github.com/nanotaboada/rust-samples-rocket-restful/compare/v1.0.0-aguero...HEAD
[1.0.0 - Agüero]: https://github.com/nanotaboada/rust-samples-rocket-restful/releases/tag/v1.0.0-aguero
