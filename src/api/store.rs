use rocket_contrib::JSON;

#[derive(Deserialize)]
struct StoreRequest {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct Response {
    succesful: bool,
    message: String,
}

#[post("/store", data = "<request>")]
fn store(request: JSON<StoreRequest>) -> JSON<Response> {
    let store_request: StoreRequest = request.0;

    let response = Response {
        succesful: true,
        message: "Stored".to_string(),
    };

    JSON(response)
}
