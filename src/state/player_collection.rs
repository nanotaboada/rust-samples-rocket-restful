//! Thread-safe player database state and initialization.
//!
//! Provides the storage type used across the whole application and initializes
//! a SQLite database (file-backed in production, in-memory for tests) seeded
//! with the Argentina 2022 World Cup squad on first run.

use rusqlite::Connection;
use std::sync::Mutex;

/// Thread-safe SQLite connection wrapper.
///
/// A type alias for `Mutex<Connection>`. Rocket shares application state
/// across multiple async worker threads to handle concurrent requests.
/// Wrapping the `Connection` in a [`std::sync::Mutex`] ensures that only
/// one thread accesses the database at a time, preventing data races.
///
/// ## Rust note: locking and `MutexGuard`
/// Before accessing the inner `Connection`, callers must call `.lock()`:
/// ```ignore
/// let connection = state.lock().map_err(|_| Status::InternalServerError)?;
/// ```
/// `lock()` blocks the current thread until no other thread holds the lock,
/// then returns a `MutexGuard<Connection>`. The guard releases the lock
/// automatically when it goes out of scope — RAII pattern.
pub type PlayerCollection = Mutex<Connection>;

/// Opens or creates the SQLite database, creates the schema, and seeds the
/// Argentina 2022 World Cup squad (26 players) if the table is empty.
///
/// Used at startup in [`main`](crate). Returns a plain `Connection`,
/// which is then wrapped in a [`PlayerCollection`] (i.e. `Mutex`) before
/// being handed to Rocket's state management via `.manage()`.
pub fn initialize_database() -> Connection {
    let path = std::env::var("STORAGE_PATH")
        .ok()
        .and_then(|v| {
            let t = v.trim().to_string();
            if t.is_empty() { None } else { Some(t) }
        })
        .unwrap_or_else(|| "storage/players-sqlite3.db".to_string());
    if let Some(parent) = std::path::Path::new(&path).parent() {
        std::fs::create_dir_all(parent).expect("Failed to create storage directory");
    }
    let mut connection = Connection::open(&path).expect("Failed to open database");
    connection.trace(Some(|statement| println!("[SQL] {statement}")));
    create_schema(&connection);
    if is_empty(&connection) {
        seed(&connection);
    }
    connection
}

/// Opens an in-memory SQLite database, creates the schema, and seeds the
/// 26-player dataset. Used exclusively in tests.
#[allow(dead_code)]
pub fn initialize_test_database() -> Connection {
    let connection = Connection::open_in_memory().expect("Failed to open in-memory database");
    create_schema(&connection);
    seed(&connection);
    connection
}

/// Opens an in-memory SQLite database and creates the schema with no rows.
///
/// Used exclusively in tests that need an empty but valid database (e.g. to
/// verify behaviour against an empty collection). The schema is kept in sync
/// with production via the shared `create_schema` helper.
#[allow(dead_code)]
pub fn initialize_empty_test_database() -> Connection {
    let connection = Connection::open_in_memory().expect("Failed to open in-memory database");
    create_schema(&connection);
    connection
}

/// Creates the `players` table if it does not already exist.
fn create_schema(connection: &Connection) {
    connection
        .execute_batch(
            "CREATE TABLE IF NOT EXISTS players (
            id            TEXT    NOT NULL PRIMARY KEY,
            first_name    TEXT    NOT NULL,
            middle_name   TEXT    NOT NULL,
            last_name     TEXT    NOT NULL,
            date_of_birth TEXT    NOT NULL,
            squad_number  INTEGER NOT NULL UNIQUE,
            position      TEXT    NOT NULL,
            abbr_position TEXT    NOT NULL,
            team          TEXT    NOT NULL,
            league        TEXT    NOT NULL,
            starting11    INTEGER NOT NULL CHECK (starting11 IN (0, 1))
        );",
        )
        .expect("Failed to create schema");
}

/// Returns `true` when the players table contains no rows.
fn is_empty(connection: &Connection) -> bool {
    let count: i64 = connection
        .query_row("SELECT COUNT(*) FROM players", [], |row| row.get(0))
        .unwrap_or(0);
    count == 0
}

type PlayerRow<'a> = (
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    u32,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    i32,
);

/// Inserts the Argentina 2022 World Cup squad (26 players).
fn seed(connection: &Connection) {
    let players: &[PlayerRow<'_>] = &[
        (
            "01772c59-43f0-5d85-b913-c78e4e281452",
            "Damián",
            "Emiliano",
            "Martínez",
            "1992-09-02T00:00:00.000Z",
            23,
            "Goalkeeper",
            "GK",
            "Aston Villa FC",
            "Premier League",
            1,
        ),
        (
            "da31293b-4c7e-5e0f-a168-469ee29ecbc4",
            "Nahuel",
            "",
            "Molina",
            "1998-04-06T00:00:00.000Z",
            26,
            "Right-Back",
            "RB",
            "Atlético Madrid",
            "La Liga",
            1,
        ),
        (
            "c096c69e-762b-5281-9290-bb9c167a24a0",
            "Cristian",
            "Gabriel",
            "Romero",
            "1998-04-27T00:00:00.000Z",
            13,
            "Centre-Back",
            "CB",
            "Tottenham Hotspur",
            "Premier League",
            1,
        ),
        (
            "d5f7dd7a-1dcb-5960-ba27-e34865b63358",
            "Nicolás",
            "Hernán Gonzalo",
            "Otamendi",
            "1988-02-12T00:00:00.000Z",
            19,
            "Centre-Back",
            "CB",
            "SL Benfica",
            "Liga Portugal",
            1,
        ),
        (
            "2f6f90a0-9b9d-5023-96d2-a2aaf03143a6",
            "Nicolás",
            "Alejandro",
            "Tagliafico",
            "1992-08-31T00:00:00.000Z",
            3,
            "Left-Back",
            "LB",
            "Olympique Lyon",
            "Ligue 1",
            1,
        ),
        (
            "b5b46e79-929e-5ed2-949d-0d167109c022",
            "Ángel",
            "Fabián",
            "Di María",
            "1988-02-14T00:00:00.000Z",
            11,
            "Right Winger",
            "RW",
            "SL Benfica",
            "Liga Portugal",
            1,
        ),
        (
            "0293b282-1da8-562e-998e-83849b417a42",
            "Rodrigo",
            "Javier",
            "de Paul",
            "1994-05-24T00:00:00.000Z",
            7,
            "Central Midfield",
            "CM",
            "Atlético Madrid",
            "La Liga",
            1,
        ),
        (
            "d3ba552a-dac3-588a-b961-1ea7224017fd",
            "Enzo",
            "Jeremías",
            "Fernández",
            "2001-01-17T00:00:00.000Z",
            24,
            "Central Midfield",
            "CM",
            "SL Benfica",
            "Liga Portugal",
            1,
        ),
        (
            "9613cae9-16ab-5b54-937e-3135123b9e0d",
            "Alexis",
            "",
            "Mac Allister",
            "1998-12-24T00:00:00.000Z",
            20,
            "Central Midfield",
            "CM",
            "Brighton & Hove Albion",
            "Premier League",
            1,
        ),
        (
            "acc433bf-d505-51fe-831e-45eb44c4d43c",
            "Lionel",
            "Andrés",
            "Messi",
            "1987-06-24T00:00:00.000Z",
            10,
            "Right Winger",
            "RW",
            "Paris Saint-Germain",
            "Ligue 1",
            1,
        ),
        (
            "38bae91d-8519-55a2-b30a-b9fe38849bfb",
            "Julián",
            "",
            "Álvarez",
            "2000-01-31T00:00:00.000Z",
            9,
            "Centre-Forward",
            "CF",
            "Manchester City",
            "Premier League",
            1,
        ),
        (
            "5a9cd988-95e6-54c1-bc34-9aa08acca8d0",
            "Franco",
            "Daniel",
            "Armani",
            "1986-10-16T00:00:00.000Z",
            1,
            "Goalkeeper",
            "GK",
            "River Plate",
            "Copa de la Liga",
            0,
        ),
        (
            "c62f2ac1-41e8-5d34-b073-2ba0913d0e31",
            "Gerónimo",
            "",
            "Rulli",
            "1992-05-20T00:00:00.000Z",
            12,
            "Goalkeeper",
            "GK",
            "Ajax Amsterdam",
            "Eredivisie",
            0,
        ),
        (
            "5fdb10e8-38c0-5084-9a3f-b369a960b9c2",
            "Juan",
            "Marcos",
            "Foyth",
            "1998-01-12T00:00:00.000Z",
            2,
            "Right-Back",
            "RB",
            "Villarreal",
            "La Liga",
            0,
        ),
        (
            "bbd441f7-fcfb-5834-8468-2a9004b64c8c",
            "Gonzalo",
            "Ariel",
            "Montiel",
            "1997-01-01T00:00:00.000Z",
            4,
            "Right-Back",
            "RB",
            "Nottingham Forest",
            "Premier League",
            0,
        ),
        (
            "d8bfea25-f189-5d5e-b3a5-ed89329b9f7c",
            "Germán",
            "Alejo",
            "Pezzella",
            "1991-06-27T00:00:00.000Z",
            6,
            "Centre-Back",
            "CB",
            "Real Betis Balompié",
            "La Liga",
            0,
        ),
        (
            "dca343a8-12e5-53d6-89a8-916b120a5ee4",
            "Marcos",
            "Javier",
            "Acuña",
            "1991-10-28T00:00:00.000Z",
            8,
            "Left-Back",
            "LB",
            "Sevilla FC",
            "La Liga",
            0,
        ),
        (
            "98306555-a466-5d18-804e-dc82175e697b",
            "Lisandro",
            "",
            "Martínez",
            "1998-01-18T00:00:00.000Z",
            25,
            "Centre-Back",
            "CB",
            "Manchester United",
            "Premier League",
            0,
        ),
        (
            "9d140400-196f-55d8-86e1-e0b96a375c83",
            "Leandro",
            "Daniel",
            "Paredes",
            "1994-06-29T00:00:00.000Z",
            5,
            "Defensive Midfield",
            "DM",
            "AS Roma",
            "Serie A",
            0,
        ),
        (
            "d3b0e8e8-2c34-531a-b608-b24fed0ef986",
            "Exequiel",
            "Alejandro",
            "Palacios",
            "1998-10-05T00:00:00.000Z",
            14,
            "Central Midfield",
            "CM",
            "Bayer 04 Leverkusen",
            "Bundesliga",
            0,
        ),
        (
            "7cc8d527-56a2-58bd-9528-2618fc139d30",
            "Alejandro",
            "Darío",
            "Gómez",
            "1988-02-15T00:00:00.000Z",
            17,
            "Left Winger",
            "LW",
            "AC Monza",
            "Serie A",
            0,
        ),
        (
            "191c82af-0c51-526a-b903-c3600b61b506",
            "Guido",
            "",
            "Rodríguez",
            "1994-04-12T00:00:00.000Z",
            18,
            "Defensive Midfield",
            "DM",
            "Real Betis Balompié",
            "La Liga",
            0,
        ),
        (
            "b1306b7b-a3a4-5f7c-90fd-dd5bdbed57ba",
            "Ángel",
            "Martín",
            "Correa",
            "1995-03-09T00:00:00.000Z",
            15,
            "Right Winger",
            "RW",
            "Atlético Madrid",
            "La Liga",
            0,
        ),
        (
            "ecec27e8-487b-5622-b116-0855020477ed",
            "Thiago",
            "Ezequiel",
            "Almada",
            "2001-04-26T00:00:00.000Z",
            16,
            "Attacking Midfield",
            "AM",
            "Atlanta United FC",
            "Major League Soccer",
            0,
        ),
        (
            "7941cd7c-4df1-5952-97e8-1e7f5d08e8aa",
            "Paulo",
            "Exequiel",
            "Dybala",
            "1993-11-15T00:00:00.000Z",
            21,
            "Second Striker",
            "SS",
            "AS Roma",
            "Serie A",
            0,
        ),
        (
            "79c96f29-c59f-5f98-96b8-3a5946246624",
            "Lautaro",
            "Javier",
            "Martínez",
            "1997-08-22T00:00:00.000Z",
            22,
            "Centre-Forward",
            "CF",
            "Inter Milan",
            "Serie A",
            0,
        ),
    ];

    for (
        id,
        first_name,
        middle_name,
        last_name,
        date_of_birth,
        squad_number,
        position,
        abbr_position,
        team,
        league,
        starting11,
    ) in players
    {
        connection.execute(
            "INSERT INTO players (id, first_name, middle_name, last_name, date_of_birth, squad_number, position, abbr_position, team, league, starting11)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            rusqlite::params![id, first_name, middle_name, last_name, date_of_birth, squad_number, position, abbr_position, team, league, starting11],
        )
        .expect("Failed to seed player");
    }
}
