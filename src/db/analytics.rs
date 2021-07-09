/*
 * analyticsbase; File: analytics.rs
 * Copyright (C) 2021 nzelot <leontsteiner@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use super::{Result, DB};
use crate::analytics_model::{Metadata, PlaybackChange, SongChange, PageChange};

pub trait AnalyticsDB {
    fn store_page_change(&mut self, meta: &Metadata, change: &PageChange) -> Result<()>;
    fn store_playback_change(&mut self, meta: &Metadata, playback: &PlaybackChange) -> Result<()>;
    fn store_song_change(&mut self, meta: &Metadata, song: &SongChange) -> Result<()>;
}

impl AnalyticsDB for DB {
    fn store_page_change(&mut self, meta: &Metadata, change: &PageChange) -> Result<()> {
        let mut stmt = self.prepare_cached("INSERT INTO analytics (tmstp,origin,kind,transition_src,transition_dst) VALUES (?,?,?,?,?)")?;

        let kind: u8 = meta.kind.into();
        let src: u8 = change.src.into();
        let dst: u8 = change.dst.into();

        stmt.execute(rusqlite::params![
            meta.tmstp.timestamp_millis(),
            &meta.origin,
            kind,
            src,
            dst
        ])?;

        println!("\tPage Change stored.");
        Ok(())
    }

    fn store_playback_change(&mut self, meta: &Metadata, playback: &PlaybackChange) -> Result<()> {
        let mut stmt = self.prepare_cached("INSERT INTO analytics (tmstp,origin,kind,playback_source,playback_name,playback_started) VALUES (?,?,?,?,?,?)")?;
        let kind: u8 = meta.kind.into();
        let source: u8 = playback.source.into();
        stmt.execute(rusqlite::params![
            meta.tmstp.timestamp_millis(),
            &meta.origin,
            kind,
            source,
            &playback.name,
            playback.started
        ])?;
        println!("\tPlayback Change stored.");
        Ok(())
    }

    fn store_song_change(&mut self, meta: &Metadata, song: &SongChange) -> Result<()> {
        let mut stmt = self.prepare_cached("INSERT INTO analytics (tmstp,origin,kind,song_raw,song_title,song_artist,song_album) VALUES (?,?,?,?,?,?,?)")?;
        let kind: u8 = meta.kind.into();
        stmt.execute(rusqlite::params![
            meta.tmstp.timestamp_millis(),
            &meta.origin,
            kind,

            &song.raw_meta,
            &song.title,
            &song.artist,
            &song.album
        ])?;
        println!("\tSong Change stored.");
        Ok(())
    }
}