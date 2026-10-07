use std::{fs::File, io::Read, mem, path::PathBuf};

use actix_web::{HttpResponse, Responder};

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct PageHeader {
    pub title: Option<String>,
    pub template: Option<String>,
}

fn split_header_and_content(raw_file: String) -> (Option<String>, String) {
    match raw_file.split_once("---metadata---") {
        Some((header, content)) => (Some(header.trim().to_string()), content.trim().to_string()),
        None => (None, raw_file),
    }
}

fn parse_header(header: String) -> PageHeader {
    let header: PageHeader = toml::from_str(&header).unwrap_or(PageHeader::default());

    header
}

fn get_page_content_path(page_shorthand: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("./site");
    path.push(format!("{}.md", page_shorthand));

    path
}

fn get_template_path(template: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push("./template");
    path.push(format!("{}.html", template));

    path
}

fn render_header_and_content_to_template(
    rendered_content: String,
    raw_template: String,
    header: &PageHeader,
) -> String {
    let header_added_template = raw_template.replace(
        "{title}",
        header.title.as_ref().unwrap_or(&"No Title".to_string()),
    );

    header_added_template.replace("{content}", &rendered_content)
}

fn render_page(page_shorthand: &str) -> std::io::Result<String> {
    let path = get_page_content_path(page_shorthand);
    println!("path: {:?}", path);

    let mut file_content = String::new();
    File::open(path)?.read_to_string(&mut file_content)?;

    let (raw_header, content) = split_header_and_content(file_content);
    let mut header = match raw_header {
        Some(header_value) => parse_header(header_value),
        None => PageHeader::default(),
    };
    println!("page header: {:?}", header);

    let rendered_content = markdown::to_html(&content);

    let template_name = mem::take(&mut header.template);
    let template_path = get_template_path(&template_name.unwrap_or("main".to_string()));

    let mut template_content = String::new();
    File::open(template_path)?.read_to_string(&mut template_content)?;

    Ok(render_header_and_content_to_template(
        rendered_content,
        template_content,
        &header,
    ))
}

pub fn handle_page_request(page_shorthand: String) -> impl Responder {
    let rendered_page = render_page(&page_shorthand);

    match rendered_page {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(err) => {
            println!("Err on rendering ({}): {:?}", page_shorthand, err);

            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
