use axum::{
    response::Html,
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
            max-width: 600px;
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
    <div class="container">
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
            </div>
        </div>
    </div>

    <script src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/js/bootstrap.bundle.min.js"></script>
</body>
</html>
"#)
}
