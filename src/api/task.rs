use actix_web::{
        get,
        post,
        put,
        error::ResponseError,
        web::Path,
        web::Json,
        web::Data,
        HttpResponse,
        http::{StatusCode, header::ContentType}
};
use serde::{Serialize, Deserialize};
use derive_more::{Display};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskIdentifier {
    pub id: u32,
}

#[get("/task/{id}")]
pub async fn get_task(id: Path<TaskIdentifier>) -> Json<String> {
    Json(id.into_inner().id)
}