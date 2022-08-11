
#[macro_use]
extern crate rocket;

#[get("/")]
async fn hi() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![hi])
        .launch()
        .await?;

    Ok(())
}
