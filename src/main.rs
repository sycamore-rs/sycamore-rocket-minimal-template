use std::io;

use rocket::{fs::FileServer, get, launch, response, routes, tokio::fs};
use sycamore::prelude::*;

#[get("/")]
async fn index() -> io::Result<response::content::RawHtml<String>> {
    let index_html = String::from_utf8(fs::read("app/dist/index.html").await?)
        .expect("index.html should be valid utf-8");

    let rendered = sycamore::render_to_string(|cx| {
        view! { cx,
            app::App {}
        }
    });

    let index_html = index_html.replace("%sycamore.body", &rendered);

    Ok(response::content::RawHtml(index_html))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from("app/dist"))
}
