/*
 * analyticsbase; File: analytics_handler.rs
 * Copyright (C) 2021 nzelot <leontsteiner@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::generated::analytics_protocol_generated as protocol;
use crate::analytics_model::{Metadata, PageChange, PlaybackChange, SongChange, MessageKind};
use crate::db::analytics::AnalyticsDB;

type DB = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub async fn analytics_message(body: bytes::Bytes, db: DB) -> Result<impl warp::Reply, std::convert::Infallible> {
    match db.get() {
        Ok(mut db_conn) => {
            let _ = consume_analytics_message(&mut db_conn, body.to_vec());
            Ok(reply(String::from(""), http::StatusCode::ACCEPTED))
        }
        Err(e) => {
            Ok(reply(e.to_string(), http::StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

fn reply<T: warp::Reply>(r: T, status: http::StatusCode) -> impl warp::Reply {
    warp::reply::with_status(r, status)
}

fn consume_analytics_message<DB>(db: &mut DB, buffer: Vec<u8>) -> std::result::Result<(), HandlerError>
    where DB: AnalyticsDB
{
    let msg = protocol::root_as_analytics_message(buffer.as_slice())
        .expect("Expected AnalyticsMessage. Got something different.");

    let metadata = Metadata::new(&msg);
    println!("Received a analytics message from {:?} at time {:?} of type {:?}", metadata.tmstp, metadata.origin, metadata.kind);

    match metadata.kind {
        MessageKind::PageChange => {
            let payload = msg.payload_as_page_change().unwrap();
            let page = PageChange::new(&payload);
            println!("\tPage change from {:?} to {:?}", page.src, page.dst);
            db.store_page_change(&metadata, &page)?;
        }

        MessageKind::PlaybackChange => {
            let payload = msg.payload_as_playback_change().unwrap();
            let playback = PlaybackChange::new(&payload);
            println!("\tPlayback change: Source: {:?}; Name: {}; Started: {}", playback.source, playback.name, playback.started);
            db.store_playback_change(&metadata, &playback)?;
        }

        MessageKind::SongChange => {
            let payload = msg.payload_as_playback_song_change().unwrap();
            let song = SongChange::new(&payload);
            println!("\tPlayback song change: Raw: {}; Title: {}, Artist: {}, Album: {}",
                     song.raw_meta,
                     song.title,
                     song.artist,
                     song.album
            );
            db.store_song_change(&metadata, &song)?;
        }
    };

    println!();
    Ok(())
}

pub async fn heartbeat() -> Result<impl warp::Reply, std::convert::Infallible> {
    println!("Received a Heartbeat request.");
    println!();
    Ok(reply(String::from(""), http::StatusCode::OK))
}

#[derive(Debug)]
pub struct HandlerError(String);

impl From<crate::db::DbError> for HandlerError {
    fn from(e: crate::db::DbError) -> Self {
        HandlerError(format!("{:?}", e))
    }
}

impl warp::Reply for HandlerError {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::with_status(self.0, http::StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}