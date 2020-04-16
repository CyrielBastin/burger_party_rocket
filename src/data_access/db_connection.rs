use super::DbConnection;
use mysql::{PooledConn, OptsBuilder, Pool};

impl DbConnection<'_>
{
    pub fn new() -> PooledConn
    {
        let mut db_conn = DbConnection {
            ip: "127.0.0.1",
            user: "root",
            password: "",
            database: "fastfood",
            port: 3308,
            pool: None
        };
        let opts_builder = OptsBuilder::new()
            .ip_or_hostname(Some(db_conn.ip))
            .user(Some(db_conn.user))
            .pass(Some(db_conn.password))
            .db_name(Some(db_conn.database))
            .tcp_port(db_conn.port);

        db_conn.pool = Some(Pool::new(opts_builder).unwrap());

        let conn = db_conn.pool.unwrap();
        conn.get_conn().unwrap()
    }
}
