// use libanalyze::{Header, Replay};
use async_tempfile::TempFile;

use poem::{
    error::{BadRequest, ParseMultipartError},
    handler,
    web::Multipart,
    Request, Result,
};
use std::env;
use tokio::io::AsyncReadExt;

use std::path::Path;
use std::{thread, time};
use tokio::io::{BufReader, BufWriter};

// #[handler]
// pub fn hello(Path(name): Path<String>) -> String {
//     format!("hello: {}", name)
// }

const FILE_SIZE_LIMIT: usize = 1024 * 1024 * 1024; // 1 GB

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

        let mut replay_content = BufReader::new(field.into_async_read());

        let file_id = {
            let mut file_id: [u8; 4] = [0; 4];
            replay_content.read_exact(&mut file_id).await.map_err(|e| {
                let response = format!("Could not parse file id: {}", e);
                ParseMultipartError::InvalidContentType(response)
            })?;
            u32::from_le_bytes(file_id)
        };

        if file_id != libanalyze::FILE_ID {
            let response = format!("Invalid file id: {}", file_id);
            return Err(ParseMultipartError::InvalidContentType(response).into());
        }

        let protocol_version = {
            let mut protocol_version: [u8; 2] = [0; 2];
            replay_content
                .read_exact(&mut protocol_version)
                .await
                .map_err(|e| {
                    let response = format!("Could not parse protocol version: {}", e);
                    ParseMultipartError::InvalidContentType(response)
                })?;
            u16::from_le_bytes(protocol_version)
        };

        if protocol_version != libanalyze::PROTOCOL_VERSION {
            let response = format!("Invalid protocol version: {}", protocol_version);
            return Err(ParseMultipartError::InvalidContentType(response).into());
        }

        let mut replay_data: [u8; 0] = [0; 0];

        let temp_dir = env::temp_dir();
        let temp_replay_dir = Path::new(&temp_dir).join("bombsquad-replay-analysis");
        tokio::fs::create_dir_all(&temp_replay_dir).await.unwrap();

        let mut file =
            TempFile::new_with_name_in(format!("{}_", file_name).as_str(), temp_replay_dir).await
                ..unwrap().open_rw().unwrap();

        let mut writer = BufWriter::new(file);

        replay_content.read(&mut writer).await.map_err(|e| {
            let response = format!("Broken pipe: {}", e);
            ParseMultipartError::InvalidContentType(response)
        })?;

        writeln!(file, "Brian was here. Briefly.").unwrap();
    }
    Ok(())
}
