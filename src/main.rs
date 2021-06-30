use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;
use warp::http::StatusCode;
use warp::{reply::with_status, Filter};

trait Repository {
    fn new_paste(&mut self, text: String) -> Uuid;
    fn get_paste(&self, id: Uuid) -> Option<&String>;
}

struct MemoryRepository {
    map: HashMap<Uuid, String>,
}

impl MemoryRepository {
    fn new() -> Self {
        MemoryRepository {
            map: HashMap::new(),
        }
    }
}

impl Repository for MemoryRepository {
    fn new_paste(&mut self, text: String) -> Uuid {
        let id = Uuid::new_v4();
        self.map.insert(id, text);
        id
    }

    fn get_paste(&self, id: Uuid) -> Option<&String> {
        self.map.get(&id)
    }
}

#[tokio::main]
async fn main() {
    let repo: Arc<Mutex<dyn Repository + Send>> = Arc::new(Mutex::new(MemoryRepository::new()));

    let local_repo = repo.clone();
    let new_paste = warp::path::end()
        .and(warp::post())
        .and(warp::body::bytes())
        .map(move |bytes: warp::hyper::body::Bytes| {
            // TODO: limit upload size
            let text: Vec<_> = bytes.to_vec();
            let text = String::from_utf8(text);
            match text {
                Ok(t) => with_status(
                    format!("{}", local_repo.lock().unwrap().new_paste(t)),
                    StatusCode::CREATED,
                ),
                Err(_) => with_status(
                    "Text is not a valid UTF-8".to_string(),
                    StatusCode::BAD_REQUEST,
                ),
            }
        });

    let local_repo = repo.clone();
    let get = warp::path!(Uuid).and(warp::get()).map(move |id| {
        let repo = local_repo.lock().unwrap();
        let paste = repo.get_paste(id);
        match paste {
            Some(p) => with_status(p.to_string(), StatusCode::OK),
            None => with_status("Not found".to_string(), StatusCode::NOT_FOUND),
        }
    });

    let routing = new_paste.or(get);

    let port = std::env::var("PASTEBIN_PORT")
        .ok()
        .and_then(|e| e.parse().ok())
        .unwrap_or(2137);
    let host = std::env::var("PASTEBIN_HOST")
        .ok()
        .and_then(|e| e.parse().ok())
        .unwrap_or(std::net::Ipv4Addr::new(127, 0, 0, 1));
    warp::serve(routing).run((host, port)).await;
}
