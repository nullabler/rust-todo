use mysql_async::{Pool, prelude::*};

pub struct Db {
    pool: Pool
}

impl Db {
    pub fn new (db_url: &str) -> Self {
        // let database_url = "mysql://root:root@mysql:3306/todo";
        // let pool = mysql_async::Pool::new(database_url);
        // let mut conn = pool.get_conn().await?;

        // conn.query_drop(
        //     r"CREATE TABLE payment (
        //     customer_id int not null,
        //     amount int not null,
        //     account_name text
        // )"
        // ).await?;

        Db {
            pool: mysql_async::Pool::new(db_url),
        }
    }
}
