/*
 * analyticsbase; File: mod.rs
 * Copyright (C) 2021 nzelot <leontsteiner@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::path::Path;

pub mod analytics;

type Result<R> = std::result::Result<R, DbError>;
type DB    = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

pub fn initialize_db<P : AsRef<Path>>(path : P) -> Result<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>> {
    let manager = r2d2_sqlite::SqliteConnectionManager::file("./analytics.db");
    let pool = r2d2::Pool::new(manager)?;
    let conn = pool.get()?;
    conn.execute_batch("\
        CREATE TABLE IF NOT EXISTS analytics (
            tmstp   INTEGER PRIMARY KEY,
            origin  VARCHAR(20) NOT NULL,
            kind    TINYINT,
            transition_src  TINYINT,
            transition_dst  TINYINT,
            playback_source TINYINT,
            playback_name   VARCHAR(25),
            playback_started BOOLEAN,
            song_raw        VARCHAR(60),
            song_title      VARCHAR(20),
            song_artist     VARCHAR(20),
            song_album      VARCHAR(20)
        );
    ")?;
    Ok(pool)
}

#[derive(Debug)]
pub struct DbError(String);

impl From<r2d2::Error> for DbError {
    fn from(e: r2d2::Error) -> Self {
        DbError(e.to_string())
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        let msg = format!("{:?}", e);
        DbError(msg)
    }
}