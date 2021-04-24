use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;
use warp::Filter;

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

impl MemoryRepository {
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
    let repo = Arc::new(Mutex::new(MemoryRepository::new()));

    let local_repo = repo.clone();
    let new_paste = warp::path!("new" / String)
        .map(move |text| format!("{}", local_repo.lock().unwrap().new_paste(text)));

    let local_repo = repo.clone();
    let get = warp::path!("g" / Uuid).map(move |id| {
        let repo = local_repo.lock().unwrap();
        let paste = repo.get_paste(id);
        match paste {
            Some(p) => p.to_string(),
            None => "Not found".to_string(), // TODO: use 404 status code
        }
    });

    let routing = warp::get().and(new_paste).or(get);

    warp::serve(routing).run(([127, 0, 0, 1], 3030)).await;
}
