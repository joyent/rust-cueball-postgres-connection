/*
 * Copyright 2019 Joyent, Inc.
 */

//! A basic example that demonstrates using the StaticIpResolver for cueball to
//! establish a basic connection pool of PostgreSQL connections.

use std::net::{IpAddr, Ipv4Addr};
use std::sync::Mutex;

use slog::{Drain, Logger, o};

use cueball::connection_pool::ConnectionPool;
use cueball::connection_pool::types::ConnectionPoolOptions;
use cueball_static_resolver::StaticIpResolver;
use cueball_postgres_connection::{PostgresConnection, PostgresConnectionConfig};

fn main() {

    let be1 = (IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5432);

    let resolver = StaticIpResolver::new(vec![be1]);

    let plain = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let log = Logger::root(
        Mutex::new(
            slog_term::FullFormat::new(plain).build()
        ).fuse(),
        o!("build-id" => "0.1.0")
    );

    let user = "postgres";
    let database = "test";
    let application_name = "postgres-connection-pool";
    let pg_config = PostgresConnectionConfig {
        user: Some(user),
        password: None,
        host: None,
        port: None,
        database: Some(database),
        application_name: Some(application_name)
    };
    let connection_creator = PostgresConnection::connection_creator(pg_config);
    let pool_opts = ConnectionPoolOptions {
        maximum: 5,
        claim_timeout: None,
        log: log,
        rebalancer_action_delay: None
    };

    let _pool = ConnectionPool::new(
        pool_opts,
        resolver,
        connection_creator
    );

    println!("Cueball!");

    loop {}
}