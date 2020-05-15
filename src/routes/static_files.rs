use actix_web::body::Body;
use actix_web::{web, HttpRequest, HttpResponse};
use mime_guess::from_path;
use std::borrow::Cow;

use crate::error::{Error, Result};
use crate::model::StaticAssets;

fn handle_embedded_file_blocking(
    path: String,
) -> Result<(Cow<'static, [u8]>, mime_guess::MimeGuess)> {
    match StaticAssets::get(&path) {
        Some(content) => {
            // Get mime type from actual file served, avoids using types like
            // image/jpeg for an image path that should actually serve index.html
            Ok((content, from_path(&path)))
        }
        None => {
            debug!("file not found: {}", &path);
            // If file not found, serve index.html
            // (only if index.html is found too to prevent a loop)
            if path == "index.html" {
                Err(Error::NotFound)
            } else {
                handle_embedded_file_blocking("index.html".to_owned())
            }
        }
    }
}

async fn handle_embedded_file(path: web::Path<String>) -> Result<HttpResponse> {
    let content;

    // If using files from filesystem, blocking calls so run it on threadpool
    if cfg!(feature = "use-fs-for-static") {
        content = web::block(move || handle_embedded_file_blocking(path.clone())).await?;
    } else {
        content = handle_embedded_file_blocking(path.clone())?
    }

    let body: Body = match content.0 {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
    };

    let mut res = HttpResponse::Ok();

    // If there's a mimetype, add it
    if let Some(content_type) = content.1.first() {
        res.content_type(content_type.as_ref());
    }

    Ok(res.body(body))
}

pub async fn path(_req: HttpRequest, path: web::Path<String>) -> Result<HttpResponse> {
    // let path = &req.path()[1..]; // trim the preceding `/` in path
    handle_embedded_file(path).await
}
