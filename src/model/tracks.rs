use serde::{ Deserialize , Serialize};


trait Task {
    fn new() -> Self;
    fn update(&self) -> Self;
    fn delete(&self) -> Self;
    fn get(&self) -> Self;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    // pub id: String,
    pub name: String,
    pub description: String,
   pub  published: bool,
}

impl Task for Track {
    fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            published: false,
        }
    }

    fn update(&self) -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            published: false,
        }
    }

    fn delete(&self) -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            published: false,
        }
    }

    fn get(&self) -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            published: false,
        }
    }
}

pub struct Tracks {
    pub tracks: Vec<Track>,
}
