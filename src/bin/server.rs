use axum::{
    response::{IntoResponse, Response as AxumResponse},
    routing::{get, post},
    Router,
    extract::DefaultBodyLimit,
    extract::Multipart,
    body::Body,
    http::{header::CONTENT_TYPE, header::CONTENT_DISPOSITION, Response},
};
use tower_http::services::ServeDir;

use std::fs::{self};
use std::path::PathBuf;

use tokio::net::TcpListener;

mod view;
use view::show_form;
use rusttone::effects;

use crate::view::show_err_page;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_form))
        .route("/process", post(process_wav_with_size_check))
        .nest_service("/images", ServeDir::new("images"))
        .layer(DefaultBodyLimit::disable());

    let listener = TcpListener::bind("0.0.0.0:3003").await.unwrap();
    println!("🚀 Running on http://0.0.0.0:3003");

    axum::serve(listener, app)
        .await
        .unwrap();
}

// Wrapper to manually check file size
async fn process_wav_with_size_check(mut multipart: Multipart) -> AxumResponse {
    const MAX_SIZE: usize = 5 * 1024 * 1024;
    
    let mut effect = String::new();
    let mut wav_data = Vec::new();
    
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        
        match name.as_str() {
            "effect" => {
                effect = field.text().await.unwrap();
            }
            "file" => {
                // Read bytes and check size
                match field.bytes().await {
                    Ok(bytes) => {
                        if bytes.len() > MAX_SIZE {
                            return show_err_page(Box::new(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "File too large"
                            ))).await.into_response();
                        }
                        wav_data = bytes.to_vec();
                    }
                    Err(_) => {
                        return show_err_page(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "File too large"
                        ))).await.into_response();
                    }
                }
            }
            _ => {}
        }
    }
    
    process_wav(effect, wav_data).await.into_response()
}



// -----------------------------------------------------------
// 2) Processing WAV file
// -----------------------------------------------------------
async fn process_wav(effect: String, wav_data: Vec<u8>) -> impl IntoResponse {

    // สร้าง temp file
    let input_path = PathBuf::from("temp_in.wav");
    let output_path = PathBuf::from("temp_out.wav");

    fs::write(&input_path, &wav_data).unwrap();

    // เรียกเอฟเฟกต์
    match effect.as_str() {
        "echo" => effects::apply_echo(&input_path, &output_path),
        "multi" => {
            // เดี๋ยวเขียนให้เพิ่มทีหลัง
            effects::apply_multi(&input_path, &output_path)
        }
        "reverb" => {
            // เดี๋ยวค่อยทำ
            effects::apply_reverb(&input_path, &output_path)
        }
        _ => effects::apply_echo(&input_path, &output_path),
    }

    let out = fs::read(&output_path).unwrap();

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    // Send correct response Axum 0.8
    Response::builder()
        .header(CONTENT_TYPE, "audio/wav")
        .header(CONTENT_DISPOSITION, "attachment; filename=\"processed.wav\"")
        .body(Body::from(out))
        .unwrap()
        .into_response()
}
