use axum::{
    // extract::Multipart,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

// mod effects;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(show_form));
        // .route("/process", post(process_wav));

    println!("🚀 Running on http://127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// -----------------------------------------------------------
// 1) หน้า Form กาก ๆ
// -----------------------------------------------------------
async fn show_form() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<body>
    <h2>RustTone - Upload WAV</h2>

    <form action="/process" method="post" enctype="multipart/form-data">
        <p>
            <label>Upload WAV:</label>
            <input type="file" name="file" required />
        </p>

        <p>
            <label>Effect:</label>
            <select name="effect">
                <option value="echo">Echo</option>
                <option value="multi">Multiple Echo</option>
                <option value="reverb">Reverb</option>
            </select>
        </p>

        <button type="submit">Process</button>
    </form>

</body>
</html>
"#)
}

// // -----------------------------------------------------------
// // 2) ประมวลผลไฟล์ WAV
// // -----------------------------------------------------------
// async fn process_wav(mut multipart: Multipart) -> impl IntoResponse {
//     let mut effect = String::new();
//     let mut wav_data: Vec<u8> = Vec::new();

//     // รับข้อมูลใน form
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let name = field.name().unwrap().to_string();

//         match name.as_str() {
//             "effect" => {
//                 effect = field.text().await.unwrap();
//             }
//             "file" => {
//                 wav_data = field.bytes().await.unwrap().to_vec();
//             }
//             _ => {}
//         }
//     }

//     // เซฟไฟล์ชั่วคราว
//     let input_path = PathBuf::from("temp_in.wav");
//     let output_path = PathBuf::from("temp_out.wav");

//     fs::write(&input_path, &wav_data).unwrap();

//     // -------------------------------------------------------
//     // เรียกฟังก์ชันประมวลผลตาม effect
//     // -------------------------------------------------------
//     match effect.as_str() {
//         "echo" => {
//             effects::apply_echo(&input_path, &output_path);
//         }
//         "reverb" => {
//             effects::apply_reverb(&input_path, &output_path);
//         }
//         "gain" => {
//             effects::apply_gain(&input_path, &output_path);
//         }
//         _ => {}
//     }

//     // -------------------------------------------------------
//     // โหลดไฟล์ที่ประมวลผลแล้วกลับออกมา
//     // -------------------------------------------------------
//     let processed_data = fs::read(&output_path).unwrap();

//     // ลบไฟล์ temp
//     let _ = fs::remove_file(input_path);
//     let _ = fs::remove_file(output_path);

//     (
//         [("Content-Type", "audio/wav"),
//          ("Content-Disposition", "attachment; filename=\"out.wav\"")],
//         processed_data,
//     )
// }
