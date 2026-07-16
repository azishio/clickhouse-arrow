#![expect(
    unused_crate_dependencies,
    reason = "integration test targets share workspace dev-dependencies"
)]

pub mod common;
pub mod tests;

const CONF: &str = "config_tls.xml";
const TRACING_DIRECTIVES: &[(&str, &str)] =
    &[("testcontainers", "debug"), ("clickhouse_arrow", "debug")];

e2e_test!(
    e2e_tls_custom_ca,
    tests::tls::test_tls_connection_with_custom_ca,
    TRACING_DIRECTIVES,
    Some(CONF)
);
