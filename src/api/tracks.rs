use actix_web::{web, HttpResponse, get};
use mongodb::{Client,Collection, bson::doc};
use crate::model::tracks::Track;

const DB_NAME: &'static str = "sinatra";
const COLL_NAME: &'static str = "tracks";

/// Gets the user with the supplied track.
#[get("/track/{track}")]
pub async fn get_track(client: web::Data<Client>, track: web::Path<String>) -> HttpResponse {
    let track = track.into_inner();
    let collection: Collection<Track> = client.database(DB_NAME).collection(COLL_NAME);


    match collection
        .find_one(doc! { "name": &track }, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with track {track}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

// #[get("/tracks")]
// pub async fn get_tracks(client: web::Data<Client>) ->HttpResponse{

//     let collection: Collection<Track> = client.database(DB_NAME).collection(COLL_NAME);

//         let mut cursor = collection.find(None, None).await.unwrap();

//         match collection.find(None, None).await {
//             Ok(mut cursor) => {
//                 let mut tracks: Vec<Track> = Vec::new();
//                 while let Some(result) = cursor.await.next().await {
//                     match result {
//                         Ok(track) => tracks.push(track),
//                         Err(e) => println!("Error {:?}", e),
//                     }
//                 }
//                 HttpResponse::Ok().json(tracks)
//             }
//             Err(e) => HttpResponse::InternalServerError().body(format!("Error {:?}", e)),
//         }
// }




