use axum::{
    // extract::Multipart,
    response::{IntoResponse, Html},
    routing::{get, post},
    Router,
};

use axum_extra::extract::Multipart;

use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

use axum::{
    http::{header::CONTENT_TYPE, header::CONTENT_DISPOSITION, Response},
    body::Full,
};

mod view;
use view::show_form;
use rusttone::effects;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_form))
        .route("/process", post(process_wav));

    println!("🚀 Running on http://127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}



// -----------------------------------------------------------
// 2) ประมวลผลไฟล์ WAV
// -----------------------------------------------------------
async fn process_wav(mut multipart: Multipart) -> impl IntoResponse {
    let mut effect = String::new();
    let mut wav_data = Vec::new();

    // อ่าน multipart field
    while let Some(field) = multipart.next_field().await.unwrap() {
        // clone ชื่อ field มาซะ
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "effect" => {
                effect = field.text().await.unwrap();
            }
            "file" => {
                wav_data = field.bytes().await.unwrap().to_vec();
            }
            _ => {}
        }
    }

    // สร้าง temp file
    let input_path = PathBuf::from("temp_in.wav");
    let output_path = PathBuf::from("temp_out.wav");

    fs::write(&input_path, &wav_data).unwrap();

    // เรียกเอฟเฟกต์
    match effect.as_str() {
        "echo" => effects::apply_echo(&input_path, &output_path),
        "multi" => {
            // เดี๋ยวเขียนให้เพิ่มทีหลัง
            effects::apply_echo(&input_path, &output_path)
        }
        "reverb" => {
            // เดี๋ยวค่อยทำ
            effects::apply_echo(&input_path, &output_path)
        }
        _ => effects::apply_echo(&input_path, &output_path),
    }

    let out = fs::read(&output_path).unwrap();

    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);

    // ส่ง response แบบถูกต้องของ Axum 0.6
    Response::builder()
        .header(CONTENT_TYPE, "audio/wav")
        .header(CONTENT_DISPOSITION, "attachment; filename=\"processed.wav\"")
        .body(Full::from(out))
        .unwrap()
}
