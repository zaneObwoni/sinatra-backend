use actix_web::{web, HttpResponse, get};
use mongodb::{Client,Collection, bson::doc};
use crate::model::tracks::{Track, Tracks};


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
            HttpResponse::NotFound().body(format!("No track with {track} found"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/tracks")]
pub async fn get_tracks(client: web::Data<Client>) -> HttpResponse {
        let collection: Collection<Tracks> = client.database(DB_NAME).collection(COLL_NAME);
        // find all documents in the collection
        let cursor = collection.find(None, None).await.unwrap();
        // iterate over the results of the cursor
        let mut tracks = Vec::new();
        for result in cursor  {
            if let Ok(track) = result {
                tracks.push(track);
            }
        }

        HttpResponse::Ok().json(tracks)
}




