use axum::{
    response::Html,
    http::StatusCode,
    response::IntoResponse,
    BoxError,
};

pub async fn show_form() -> Html<&'static str> {
    Html(r#"

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RustTone - Upload WAV</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
    <style>
        :root {
            --miku-turquoise: #39c5bb;
            --miku-dark: #2a9d95;
            --miku-light: #e8f9f8;
            --miku-accent: #00d4c5;
        }

        body {
            background: linear-gradient(135deg, var(--miku-light) 0%, #ffffff 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        }

        .container {
            max-width: 1200px;
        }

        .main-layout {
            display: grid;
            grid-template-columns: 1fr;
            gap: 2rem;
        }

        @media (min-width: 992px) {
            .main-layout {
                grid-template-columns: 1fr 1fr;
            }
        }

        .card {
            border: none;
            border-radius: 20px;
            box-shadow: 0 10px 40px rgba(57, 197, 187, 0.2);
            overflow: hidden;
        }

        .card-header {
            background: linear-gradient(135deg, var(--miku-turquoise) 0%, var(--miku-accent) 100%);
            color: white;
            padding: 2rem;
            border: none;
        }

        .card-header h2 {
            margin: 0;
            font-weight: 600;
            font-size: 2rem;
        }

        .card-body {
            padding: 2.5rem;
            background: white;
        }

        .form-label {
            color: var(--miku-dark);
            font-weight: 600;
            margin-bottom: 0.5rem;
        }

        .form-control, .form-select {
            border: 2px solid var(--miku-light);
            border-radius: 12px;
            padding: 0.75rem 1rem;
            transition: all 0.3s ease;
        }

        .form-control:focus, .form-select:focus {
            border-color: var(--miku-turquoise);
            box-shadow: 0 0 0 0.2rem rgba(57, 197, 187, 0.25);
        }

        .btn-primary {
            background: linear-gradient(135deg, var(--miku-turquoise) 0%, var(--miku-accent) 100%);
            border: none;
            border-radius: 12px;
            padding: 0.875rem 2.5rem;
            font-weight: 600;
            font-size: 1.1rem;
            transition: all 0.3s ease;
            box-shadow: 0 4px 15px rgba(57, 197, 187, 0.3);
        }

        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(57, 197, 187, 0.4);
            background: linear-gradient(135deg, var(--miku-dark) 0%, var(--miku-turquoise) 100%);
        }

        .btn-primary:active {
            transform: translateY(0);
        }

        .mb-4 {
            margin-bottom: 1.5rem;
        }

        .form-control::file-selector-button {
            background-color: var(--miku-turquoise);
            color: white;
            border: none;
            border-radius: 8px;
            padding: 0.5rem 1rem;
            margin-right: 1rem;
            cursor: pointer;
            transition: all 0.3s ease;
        }

        .form-control::file-selector-button:hover {
            background-color: var(--miku-dark);
        }
    </style>
</head>
<body>
    <div class="container py-4">
        <div class="main-layout">
            <!-- Left Column: Main Upload Form -->
            <div>
                <div class="card">
                    <div class="card-header text-center">
                        <h2>🎵 RustTone - Upload WAV</h2>
                    </div>
                    <div class="card-body">
                        <form action="/process" method="post" enctype="multipart/form-data">
                            <div class="mb-4">
                                <label for="fileUpload" class="form-label">Upload WAV:</label>
                                <input type="file" class="form-control" id="fileUpload" name="file" required accept=".wav,audio/wav" />
                            </div>

                            <div class="mb-4">
                                <label for="effectSelect" class="form-label">Effect:</label>
                                <select class="form-select" id="effectSelect" name="effect">
                                    <option value="echo">Echo</option>
                                    <option value="multi">Multiple Echo</option>
                                    <option value="reverb">Reverb</option>
                                </select>
                            </div>

                            <div class="text-center">
                                <button type="submit" class="btn btn-primary">Process Audio</button>
                            </div>
                        </form>
                        
                        <!-- Future Development Notice -->
                        <div class="mt-4 text-center">
                            <p class="text-muted mb-2" style="font-size: 0.95rem;">
                                <i class="fas fa-magic" style="color: var(--miku-turquoise);"></i>
                                More effects and adjustments in the future.
                            </p>
                            <div style="font-size: 2rem;">
                                <img src="https://raw.githubusercontent.com/Porrapat/rusttone/refs/heads/master/images/hasune_miku_happy.jpg?token=GHSAT0AAAAAADDCKSLJUB7IXXECDEWGHUHG2JEILGQ" width="250" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <!-- Right Column: Resources and Examples -->
            <div>
                <!-- Links Section -->
                <div class="card mb-4">
                    <div class="card-body">
                        <h5 class="text-center mb-3" style="color: var(--miku-dark); font-weight: 600;">📚 Project Resources</h5>
                        <div class="d-flex flex-column gap-2">
                            <a href="https://github.com/Porrapat/rusttone" target="_blank" class="btn btn-outline-primary" style="border-color: var(--miku-turquoise); color: var(--miku-turquoise);">
                                <i class="fab fa-github"></i> GitHub Repository
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/blob/master/paper_project_effect_guitar.pdf" target="_blank" class="btn btn-outline-primary" style="border-color: var(--miku-turquoise); color: var(--miku-turquoise);">
                                <i class="fas fa-file-pdf"></i> Project Paper
                            </a>
                        </div>
                    </div>
                </div>

                <!-- Example Files Section -->
                <div class="card">
                    <div class="card-body">
                        <h5 class="text-center mb-3" style="color: var(--miku-dark); font-weight: 600;">🎼 Example Audio Files</h5>
                        <p class="text-center text-muted small mb-3">Try these sample WAV files with different effects</p>
                        <div class="d-flex flex-column gap-2">
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/destiny_1.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> พรหมลิขิต 1 (destiny_1.wav)
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/destiny_2.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> พรหมลิขิต 2 (destiny_2.wav)
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/fon_dtok_tee_nah_dtahng.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> ฝนตกที่หน้าต่าง (fon_dtok_tee_nah_dtahng.wav)
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/moom_1.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> มุม 1 (moom_1.wav)
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/moom_2.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> มุม 2 (moom_2.wav)
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/namtar_1.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> น้ำตา 1 (namtar_1.wav)
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/namtar_2.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> น้ำตา 2 (namtar_2.wav)
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/plook_jai_seua_pah.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> ปลุกใจเสือป่า (plook_jai_seua_pah.wav)
                            </a>
                            <a href="https://github.com/Porrapat/rusttone/raw/master/source_wav/big_wav_file.wav" class="btn btn-sm btn-outline-secondary text-start" style="border-color: var(--miku-light); color: #666;">
                                <i class="fas fa-download"></i> ไฟล์นี่ใหญ่เกิน big_wav_file.wav (This file is exceeds limit)
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js"></>
</body>
</html>
"#)
}

pub async fn show_err_page(err: BoxError) -> impl IntoResponse {
    eprintln!("Error occurred: {}", err);
    
    (StatusCode::PAYLOAD_TOO_LARGE,
    Html(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Upload Error - RustTone</title>
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.0/css/all.min.css">
    <style>
        :root {
            --miku-turquoise: #39c5bb;
            --miku-dark: #2a9d95;
            --miku-light: #e8f9f8;
            --miku-accent: #00d4c5;
            --error-red: #ff6b9d;
            --error-dark: #e74c7d;
        }

        body {
            background: linear-gradient(135deg, var(--miku-light) 0%, #ffffff 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            overflow: hidden;
        }

        .container {
            max-width: 600px;
            animation: slideIn 0.5s ease-out;
        }

        @keyframes slideIn {
            from {
                opacity: 0;
                transform: translateY(-30px);
            }
            to {
                opacity: 1;
                transform: translateY(0);
            }
        }

        .error-card {
            border: none;
            border-radius: 20px;
            box-shadow: 0 15px 50px rgba(255, 107, 157, 0.25);
            overflow: hidden;
            background: white;
        }

        .error-header {
            background: linear-gradient(135deg, var(--error-red) 0%, var(--error-dark) 100%);
            color: white;
            padding: 2.5rem;
            text-align: center;
            position: relative;
            overflow: hidden;
        }

        .error-header::before {
            content: '';
            position: absolute;
            top: -50%;
            left: -50%;
            width: 200%;
            height: 200%;
            background: radial-gradient(circle, rgba(255,255,255,0.1) 0%, transparent 70%);
            animation: pulse 3s ease-in-out infinite;
        }

        @keyframes pulse {
            0%, 100% {
                transform: scale(1);
                opacity: 0.5;
            }
            50% {
                transform: scale(1.1);
                opacity: 0.3;
            }
        }

        .error-icon {
            font-size: 4rem;
            margin-bottom: 1rem;
            animation: shake 0.5s ease-in-out;
            position: relative;
            z-index: 1;
        }

        @keyframes shake {
            0%, 100% { transform: translateX(0); }
            25% { transform: translateX(-10px); }
            75% { transform: translateX(10px); }
        }

        .error-header h1 {
            margin: 0;
            font-weight: 700;
            font-size: 2rem;
            position: relative;
            z-index: 1;
        }

        .error-body {
            padding: 2.5rem;
            text-align: center;
        }

        .error-message {
            background: linear-gradient(135deg, #fff5f8 0%, #ffecf2 100%);
            border-left: 4px solid var(--error-red);
            padding: 1.5rem;
            border-radius: 12px;
            margin-bottom: 1.5rem;
            box-shadow: 0 4px 15px rgba(255, 107, 157, 0.1);
        }

        .error-message h3 {
            color: var(--error-dark);
            font-size: 1.3rem;
            font-weight: 600;
            margin-bottom: 0.75rem;
        }

        .error-message p {
            color: #666;
            margin: 0.5rem 0;
            line-height: 1.6;
        }

        .file-size-info {
            background: var(--miku-light);
            border-radius: 10px;
            padding: 1rem;
            margin: 1.5rem 0;
            display: inline-block;
        }

        .file-size-info i {
            color: var(--miku-turquoise);
            margin-right: 0.5rem;
        }

        .file-size-info strong {
            color: var(--miku-dark);
        }

        .btn-back {
            background: linear-gradient(135deg, var(--miku-turquoise) 0%, var(--miku-accent) 100%);
            border: none;
            border-radius: 12px;
            padding: 0.875rem 2.5rem;
            font-weight: 600;
            font-size: 1.1rem;
            color: white;
            transition: all 0.3s ease;
            box-shadow: 0 4px 15px rgba(57, 197, 187, 0.3);
            text-decoration: none;
            display: inline-block;
        }

        .btn-back:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(57, 197, 187, 0.4);
            background: linear-gradient(135deg, var(--miku-dark) 0%, var(--miku-turquoise) 100%);
            color: white;
        }

        .btn-back:active {
            transform: translateY(0);
        }

        .btn-back i {
            margin-right: 0.5rem;
        }

        .tips {
            background: #f8f9fa;
            border-radius: 10px;
            padding: 1.25rem;
            margin-top: 1.5rem;
            text-align: left;
        }

        .tips h4 {
            color: var(--miku-dark);
            font-size: 1rem;
            font-weight: 600;
            margin-bottom: 0.75rem;
        }

        .tips ul {
            margin: 0;
            padding-left: 1.5rem;
            color: #666;
        }

        .tips li {
            margin: 0.5rem 0;
            line-height: 1.5;
        }

        .decoration {
            position: fixed;
            opacity: 0.05;
            pointer-events: none;
            z-index: 0;
        }

        .decoration-1 {
            top: 10%;
            left: 5%;
            font-size: 8rem;
            color: var(--miku-turquoise);
        }

        .decoration-2 {
            bottom: 10%;
            right: 5%;
            font-size: 6rem;
            color: var(--error-red);
        }
    </style>
</head>
<body>
    <i class="fas fa-music decoration decoration-1"></i>
    <i class="fas fa-exclamation-triangle decoration decoration-2"></i>

    <div class="container">
        <div class="error-card">
            <div class="error-header">
                <div class="error-icon">
                    <i class="fas fa-exclamation-circle"></i>
                </div>
                <h1>🎵 Upload Failed</h1>
            </div>
            <div class="error-body">
                <div class="error-message">
                    <h3><i class="fas fa-file-audio"></i> File Too Large!</h3>
                    <p>The audio file exceeds the maximum allowed size.</p>
                    <p>Please choose a smaller file and try again.</p>
                </div>

                <div class="file-size-info">
                    <i class="fas fa-info-circle"></i>
                    <strong>Maximum file size: 5 MB</strong>
                </div>

                <div class="mt-4">
                    <a href="/" class="btn-back">
                        <i class="fas fa-arrow-left"></i> Back to Upload
                    </a>
                </div>
            </div>
        </div>
    </div>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>
    "#))
}
