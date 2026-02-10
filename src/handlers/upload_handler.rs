use axum::{Json, body::Body, extract::Query, http::StatusCode, response::IntoResponse};
use futures_util::TryStreamExt;
use serde::Deserialize;
use serde_json::json;
use std::path::Path;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Deserialize)]
pub struct QueryParams {
    pub filename: Option<String>,
}
pub async fn upload_handler(
    Query(query): Query<QueryParams>,
    body: Body,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let filename = query.filename.ok_or((
        StatusCode::BAD_REQUEST,
        "filename query parameter is required".to_string(),
    ))?;

    let path = Path::new(&filename);
    let mut file = File::create(&path)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    println!("\nStart reading body!");
    let mut stream = body.into_data_stream();
    while let Some(chunk) = stream
        .try_next()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?
    {
        file.write_all(&chunk)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }
    file.flush()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    println!("\nDone reading body!");
    Ok((StatusCode::OK, Json(json!({ "status": "ok" }))))

}
