use std::path::Path;
use std::sync::Arc;

use clickhouse_arrow::prelude::*;
use clickhouse_arrow::test_utils::ClickHouseContainer;

/// Connects to `ClickHouse` over TLS using a custom CA and executes a query.
///
/// # Panics
/// Panics if the TLS client cannot be built, the query fails, or the client cannot be shut down.
pub async fn test_tls_connection_with_custom_ca(ch: Arc<ClickHouseContainer>) {
    let cafile = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/bin/tls-server.pem"));
    let client = ClientBuilder::new()
        .with_endpoint(ch.get_native_url())
        .with_username(&ch.user)
        .with_password(&ch.password)
        .with_ipv4_only(true)
        .with_tls(true)
        .with_cafile(cafile)
        .with_domain("localhost")
        .build_native()
        .await
        .expect("building TLS client");

    client.execute("SELECT 1", None).await.expect("executing query over TLS");
    client.shutdown().await.expect("shutting down TLS client");
}
