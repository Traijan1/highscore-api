mod score;

pub fn routes() -> Vec<rocket::Route> {
    let routes = score::routes();
    routes
}
