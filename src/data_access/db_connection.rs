use super::DbConnection;
use mysql::{PooledConn, OptsBuilder, Pool};
use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

impl DbConnection<'_>
{
    pub fn new() -> PooledConn
    {
        let mut database_json = File::open("./database.json").unwrap();
        let mut file_content = String::new();
        database_json.read_to_string(&mut file_content).unwrap();
        let db_config: ConfigFromFile = serde_json::from_str(&file_content).unwrap();

        let mut db_conn = DbConnection {
            ip: db_config.ip,
            user: db_config.user,
            password: db_config.password,
            database: db_config.database,
            port: db_config.port,
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

#[derive(Serialize, Deserialize)]
struct ConfigFromFile<'a>
{
    ip: &'a str,
    user: &'a str,
    password: &'a str,
    database: &'a str,
    port: u16,
}
