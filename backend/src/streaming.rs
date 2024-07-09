use actix_web::{get, http::header, web, HttpRequest, HttpResponse, Result};
use log::info;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, BufReader};
use tokio_util::io::ReaderStream;

const DEFAULT_CHUNK_SIZE: usize = 65_536; // 64 KiB
const HIGH_SPEED_CHUNK_SIZE: usize = 131_072; // 128 KiB
const LOW_SPEED_CHUNK_SIZE: usize = 32_768; // 32 KiB
const DEFAULT_SPEED: usize = 1_000_000; // 1 Mbps

#[get("/video")]
async fn video_stream(req: HttpRequest) -> Result<HttpResponse> {
    let video_path = "/workspaces/ClipForge/backend/videos/sample-file-4k-uhd.mp4";
    let connection_speed = estimate_connection_speed(&req);
    println!("Connection speed: {} bps", connection_speed);
    let chunk_size = adapt_chunk_size(connection_speed);
    println!("Chunk size: {} bytes", chunk_size);

    let mut file = File::open(video_path)
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to open video file"))?;

    let file_size = file.metadata().await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read file metadata"))?.len();

    let (start, end) = parse_range_header(req.headers().get(header::RANGE), file_size)
        .unwrap_or((0, file_size - 1));

    if start >= file_size {
        return Err(actix_web::error::ErrorRangeNotSatisfiable("Range is not satisfiable"));
    }

    file.seek(tokio::io::SeekFrom::Start(start))
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to seek in video file"))?;

    let reader = BufReader::with_capacity(chunk_size, file.take(end - start + 1));
    let stream = ReaderStream::new(reader);

    Ok(HttpResponse::PartialContent()
        .content_type("video/mp4")
        .append_header(("Content-Range", format!("bytes {}-{}/{}", start, end, file_size)))
        .append_header(("Accept-Ranges", "bytes"))
        .streaming(stream))
}

fn estimate_connection_speed(req: &HttpRequest) -> usize {
    req.headers().get("X-Connection-Speed")
        .and_then(|value| value.to_str().ok())
        .and_then(|speed_str| speed_str.parse::<usize>().ok())
        .unwrap_or(DEFAULT_SPEED)
}

fn adapt_chunk_size(connection_speed: usize) -> usize {
    match connection_speed {
        0..=500_000 => LOW_SPEED_CHUNK_SIZE,
        500_001..=1_000_000 => DEFAULT_CHUNK_SIZE,
        _ => HIGH_SPEED_CHUNK_SIZE,
    }
}

fn parse_range_header(range_header: Option<&header::HeaderValue>, file_size: u64) -> Option<(u64, u64)> {
    range_header.and_then(|value| value.to_str().ok())
        .and_then(|range_str| {
            if let Some(range) = range_str.strip_prefix("bytes=") {
                if let Some((start, end)) = range.split_once('-') {
                    let start = start.parse::<u64>().ok()?;
                    let end = end.parse::<u64>().unwrap_or(file_size - 1);
                    return Some((start, end));
                }
            }
            None
        })
}