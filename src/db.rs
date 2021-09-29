use rocket_db_pools::{rocket::figment::Figment, Config, Database, Pool};
use sea_orm::{DatabaseConnection, DbErr};

#[derive(Database)]
#[database("LoD_backend")]
pub struct DB(SeaPool);

pub struct SeaPool {
    pub connection: DatabaseConnection,
}

#[rocket::async_trait]
impl Pool for SeaPool {
    type Connection = DatabaseConnection;
    type Error = DbErr;

    async fn init(figment: &Figment) -> Result<Self, Self::Error> {
        let config: Config = figment.extract().unwrap();
        let connection = sea_orm::Database::connect(&config.url).await?;

        Ok(SeaPool { connection })
    }

    async fn get(&self) -> Result<Self::Connection, Self::Error> {
        Ok(self.connection.clone())
    }
}
