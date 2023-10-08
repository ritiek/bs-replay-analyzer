use async_tempfile::TempFile;
use libanalyze::{Header, Replay};

use poem::{
    error::{BadRequest, ParseMultipartError},
    handler,
    web::Multipart,
    Request, Result,
};
use std::env;
use tokio::io::AsyncReadExt;

use std::path::Path;
use std::path::PathBuf;
use std::{thread, time};
use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::{BufReader, BufWriter};

// #[handler]
// pub fn hello(Path(name): Path<String>) -> String {
//     format!("hello: {}", name)
// }

const FILE_SIZE_LIMIT: usize = 1024 * 1024 * 1024; // 1 GB
const BUF_SIZE: usize = 20 * 1024 * 1024; // 20 MB

#[handler]
pub async fn upload(request: &Request, mut multipart: Multipart) -> Result<()> {
    let content_length = request
        .header("content-length")
        .ok_or_else(|| {
            let response = String::from("Missing 'content-length' request header");
            ParseMultipartError::InvalidContentType(response)
        })
        .and_then(|content_length| {
            content_length.parse::<usize>().map_err(|e| {
                let response = format!("Could not parse 'content-length' request header: {}", e);
                ParseMultipartError::InvalidContentType(response)
            })
        })
        .and_then(|content_length| {
            if content_length > FILE_SIZE_LIMIT {
                let response = String::from("File size cannot be more than 1 GB");
                Err(ParseMultipartError::InvalidContentType(response))
            } else {
                Ok(content_length)
            }
        })?;

    while let Some(field) = multipart.next_field().await? {
        let file_name = field
            .file_name()
            .ok_or_else(|| {
                let response = String::from("Missing file name");
                ParseMultipartError::InvalidContentType(response)
            })
            .and_then(|file_name| {
                if file_name.ends_with(".brp") {
                    Ok(file_name)
                } else {
                    let response = format!("Invalid file name: {}", file_name);
                    Err(ParseMultipartError::InvalidContentType(response))
                }
            })?
            .to_string();

        let mut replay_content = BufReader::with_capacity(BUF_SIZE, field.into_async_read());

        let file_id_le = {
            let mut file_id_le: [u8; 4] = [0; 4];
            replay_content
                .read_exact(&mut file_id_le)
                .await
                .map_err(|e| {
                    let response = format!("Could not parse file id: {}", e);
                    ParseMultipartError::InvalidContentType(response)
                })?;
            file_id_le
        };
        let file_id = u32::from_le_bytes(file_id_le);

        if file_id != libanalyze::FILE_ID {
            let response = format!("Invalid file id: {}", file_id);
            return Err(ParseMultipartError::InvalidContentType(response).into());
        }

        let protocol_version_le = {
            let mut protocol_version_le: [u8; 2] = [0; 2];
            replay_content
                .read_exact(&mut protocol_version_le)
                .await
                .map_err(|e| {
                    let response = format!("Could not parse protocol version: {}", e);
                    ParseMultipartError::InvalidContentType(response)
                })?;
            protocol_version_le
        };
        let protocol_version = u16::from_le_bytes(protocol_version_le);

        if protocol_version != libanalyze::PROTOCOL_VERSION {
            let response = format!("Invalid protocol version: {}", protocol_version);
            return Err(ParseMultipartError::InvalidContentType(response).into());
        }

        let temp_dir = env::temp_dir();
        let temp_replay_dir = Path::new(&temp_dir).join("bombsquad-replay-analysis");
        tokio::fs::create_dir_all(&temp_replay_dir).await.unwrap();

        let random_chars = std::iter::repeat_with(|| fastrand::alphanumeric())
            .take(5)
            .collect::<String>();

        let mut file = TempFile::new_with_name_in(
            format!("{}_{}", file_name, random_chars).as_str(),
            temp_replay_dir,
        )
        .await
        .unwrap()
        .open_rw()
        .await
        .unwrap();

        let mut writer = BufWriter::with_capacity(BUF_SIZE, &mut file);
        writer.write(&file_id_le).await.unwrap();
        writer.write(&protocol_version_le).await.unwrap();

        loop {
            let chunk = replay_content.fill_buf().await.unwrap();
            let chunk_length = chunk.len();
            if chunk_length == 0 {
                break;
            }
            writer.write(chunk).await.unwrap();
            replay_content.consume(chunk_length);
        }

        writer.flush().await.unwrap();

        // let duration = time::Duration::from_secs(30);
        // thread::sleep(duration);
        //
        let replay = Replay::new(file.file_path().to_path_buf());
        // let header = replay.get_header()?;

        let output_path = PathBuf::from("test.brp");
        // FIXME: This is io blocking
        let mut decompressed_replay = unsafe { replay.decompress(output_path) };
    }
    Ok(())
}
