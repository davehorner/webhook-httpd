use multer::Multipart;
use tokio::io;
use std::io::Write;
use tokio::io::AsyncWriteExt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the CONTENT_TYPE environment variable to get the boundary
    let content_type = match std::env::var("CONTENT_TYPE") {
        Ok(content_type) => content_type,
        Err(_) => {
            eprintln!("Error: CONTENT_TYPE environment variable not found.");
            return Ok(());
        }
    };

    // Parse the boundary from the content_type, exit early if we don't have it.
    let boundary = match content_type.split(';').find(|s| s.trim().starts_with("boundary=")) {
        Some(boundary) => boundary.trim().trim_start_matches("boundary=").to_string(),
        None => {
            eprintln!("Error: Boundary not found in CONTENT_TYPE.");
            return Ok(());
        }
    };

    let mut multipart = Multipart::with_reader(io::stdin(), &boundary);
    while let Some(mut field) = multipart.next_field().await? {
        let field_name = field.name().unwrap_or_default().to_string();
        let has_filename = field.file_name().map(|s| s.to_string());

        // Check if the field has a file name
        if let Some(file_name) = has_filename {
            eprintln!("Writing a file: {}", &file_name);

            while let Some(chunk) = field.chunk().await? {
                // Write the file content to stdout
                if let Err(e) = io::stdout().write_all(&chunk).await {
                    eprintln!("Error writing to stdout: {}", e);
                    break;
                }
            }
        } else {
            while let Some(chunk) = field.chunk().await? {
                eprintln!("Field '{}' = {}", field_name, String::from_utf8_lossy(&chunk));
            }
        }
    }

    Ok(())
}

