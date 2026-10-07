use std::{fs::File, io::Read, path::PathBuf};

use actix_web::{HttpResponse, Responder};

pub fn render_page(page_shorthand: &str) -> std::io::Result<String> {
    let mut path = PathBuf::new();
    path.push("./site");
    path.push(format!("{}.md", page_shorthand));

    println!("path: {:?}", path);

    let mut file_content = String::new();
    File::open(path)?.read_to_string(&mut file_content)?;

    Ok(markdown::to_html(&file_content))
}

pub fn handle_page_request(page_shorthand: &str) -> impl Responder {
    let rendered_page = render_page(page_shorthand);

    match rendered_page {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(err) => {
            println!("Err on rendering ({}): {:?}", page_shorthand, err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
