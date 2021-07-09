/*
 * analyticsbase; File: main.rs
 * Copyright (C) 2021 nzelot <leontsteiner@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::net::SocketAddr;

mod generated;
mod db;
mod analytics_model;
mod analytics_handler;

#[tokio::main]
async fn main() {
    let db = db::initialize_db().expect("Failed to created DB!");

    let api = filters::endpoints(db);

    let env_ip_str = match std::env::var("SERVER_IP") {
        Ok(given_ip) => given_ip,
        Err(_) => "192.168.2.111:2222".to_string()
    };
    let sock_addr: SocketAddr = env_ip_str.parse().unwrap();

    println!("Analyticsbase listening on => {}", env_ip_str);
    warp::serve(api).run(sock_addr).await;
}

mod filters {
    use warp::Filter;

    type DB = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

    pub fn endpoints(
        db: DB
    ) -> impl warp::Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        analytics_heartbeat()
            .or(analytics_message(db))
    }

    pub fn analytics_heartbeat() -> impl warp::Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path!("analytics" / "heartbeat")
            .and(warp::get())
            .and_then(crate::analytics_handler::heartbeat)
    }

    pub fn analytics_message(
        db: DB
    ) -> impl warp::Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
        warp::path!("analytics")
            .and(warp::post())
            .and(warp::body::bytes())
            .and(with_db(db))
            .and_then(crate::analytics_handler::analytics_message)
    }

    fn with_db(db: DB) -> impl Filter<Extract=(DB, ), Error=std::convert::Infallible> + Clone {
        warp::any().map(move || db.clone())
    }
}