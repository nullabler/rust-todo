// use crate::models::posts::PostModel;
use crate::app::config::Config;
use diesel::prelude::*;

use diesel::mysql::MysqlConnection;


//     // post: PostModel
// }

pub struct Connect {
    pub connect: Result<MysqlConnection, ConnectionError>,
}


impl Connect {
    pub fn set_connect(&mut self, config: super::config::Config ) {
        self.connect = MysqlConnection::establish(config.db_url())
                .expect(&format!("Error connecting to {}", config.db_url()));
    }
}

pub trait Db {
    const CONNECTION: Connect = Connect {
        connect: MysqlConnection::establish("")
            .expect(&format!("Error connecting. Check config")),
    };

    fn new(config: &Config) -> Self {}
        // let connect = MysqlConnection::establish(config.db_url()).expect(&format!("Error connecting to {}", config.db_url()));
        // Db {
        //     connect: MysqlConnection::establish(config.db_url())
        //         .expect(&format!("Error connecting to {}", config.db_url())),
        //     // post: PostModel::new(&connect)
        // }
    // }
}
