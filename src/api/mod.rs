use rocket;

mod store;

pub fn start() {
    rocket::ignite().mount("/", routes![store::store]).launch();
}
