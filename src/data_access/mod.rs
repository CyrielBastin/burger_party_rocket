use mysql::Pool;

pub struct DbConnection<'a>
{
    ip: &'a str,
    user: &'a str,
    password: &'a str,
    database: &'a str,
    port: u16,
    pool: Option<Pool>
}
mod db_connection;
