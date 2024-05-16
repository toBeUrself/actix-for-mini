pub fn get_conn_builder(
    user: &str,
    pwd: &str,
    host: &str,
    port: u16,
    name: &str,
) -> mysql::OptsBuilder {
    mysql::OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .tcp_port(port)
        .db_name(Some(name))
        .user(Some(user))
        .pass(Some(pwd))
}
