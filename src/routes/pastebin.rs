use rocket::{response::status, serde::json::Json};
use rocket::{Build, Rocket};
use rocket_db_pools::Connection;
use sea_orm::{entity::*, ActiveModelTrait};
use uuid::Uuid;

use crate::db::DB;
use crate::models::{code, code::Entity as Code};
use crate::req::CodeReq;

pub async fn init(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket.mount("/pastebin", routes![index, create, get, delete])
}

#[get("/")]
pub fn index() -> &'static str {
    "
    # pastebin

    Upload code and share with other by uuid

    USAGE

        POST /

            Accepts raw data in te body of the request and responds with a URL of a page containing the body's content

        GET /<id>

            retrieves the content for the paste with id `<id>`

        DELETE /<id>

            delete content with id `<id>`
    "
}

#[post("/", data = "<code>", format = "json")]
pub async fn create(db: Connection<DB>, code: Json<CodeReq>) -> Json<Uuid> {
    let code = code.into_inner();

    let code = code::ActiveModel {
        language: Set(code.language.to_owned()),
        content: Set(code.content.to_owned()),
        ..Default::default()
    };
    let res = code.save(&db).await.expect("Cannot save code");
    Json(res.id.unwrap())
}

#[get("/<id>")]
pub async fn get(
    db: Connection<DB>,
    id: Uuid,
) -> Result<Json<code::Model>, status::NotFound<String>> {
    match Code::find_by_id(id).one(&db).await {
        Ok(Some(code)) => Ok(Json(code)),
        _ => Err(status::NotFound("Code not found".to_string())),
    }
}

#[delete("/<id>")]
pub async fn delete(db: Connection<DB>, id: Uuid) -> Result<Json<Uuid>, status::NotFound<String>> {
    match Code::find_by_id(id).one(&db).await {
        Ok(Some(code)) => {
            let code: code::ActiveModel = code.into();
            code.delete(&db).await.expect("Cannot delete code");
            Ok(Json(id))
        }
        _ => Err(status::NotFound("Code not found".to_string())),
    }
}
