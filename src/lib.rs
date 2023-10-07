// use libanalyze::{Header, Replay};

use poem::{
    error::{BadRequest, ParseMultipartError},
    handler,
    web::Multipart,
    Result,
};
use tokio::io::AsyncReadExt;

// #[handler]
// pub fn hello(Path(name): Path<String>) -> String {
//     format!("hello: {}", name)
// }

#[handler]
pub async fn upload(mut multipart: Multipart) -> Result<()> {
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
            })?;

        let mut replay_content = field.into_async_read();

        let mut file_id = [0; 4];
        replay_content.read_exact(&mut file_id).await.map_err(|e| {
            let response = format!("{}. Could not parse file id.", e);
            ParseMultipartError::InvalidContentType(response)
        })?;

        let file_id = u32::from_le_bytes(file_id);

        if file_id != libanalyze::FILE_ID {
            let response = format!("Invalid file id: {}", file_id);
            return Err(ParseMultipartError::InvalidContentType(response).into());
        }

        let mut protocol_version = [0; 2];
        replay_content
            .read_exact(&mut protocol_version)
            .await
            .map_err(|e| {
                let response = format!("{}. Could not parse protocol version.", e);
                ParseMultipartError::InvalidContentType(response)
            })?;

        let protocol_version = u16::from_le_bytes(protocol_version);

        if protocol_version != libanalyze::PROTOCOL_VERSION {
            let response = format!("Invalid protocol version: {}", protocol_version);
            return Err(ParseMultipartError::InvalidContentType(response).into());
        }
    }
    Ok(())
}
