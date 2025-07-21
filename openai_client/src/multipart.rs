use serde::{Deserialize, Serialize};

use crate::ConversaError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(skip)]
    file_name: String,
    file_data: Vec<u8>,
}

impl TryFrom<&std::path::Path> for File {
    type Error = ConversaError;

    fn try_from(value: &std::path::Path) -> Result<Self, Self::Error> {
        let file_name = value
            .file_name()
            .ok_or(ConversaError::IoError(
                "Path does not have file name".to_string(),
            ))?
            .to_str()
            .ok_or(ConversaError::IoError(
                "Failed to convert OsStr to str".to_string(),
            ))?
            .to_string();

        let file_data = std::fs::read(value)?;
        Ok(Self {
            file_name,
            file_data,
        })
    }
}

// Some types need to be converted into multipart form. This is not easy to automate,
// so the methods to do this are manually defined here

impl crate::types::CreateTranscriptionRequest {
    pub(crate) fn into_multipart_form(self) -> reqwest::multipart::Form {
        let file_part = reqwest::multipart::Part::bytes(self.file.file_data.clone())
            .mime_str("application/octet-stream")
            .unwrap()
            .file_name(self.file.file_name.clone());
        reqwest::multipart::Form::new()
            .text(
                "model",
                serde_json::to_string(&self.model)
                    .unwrap()
                    .replace(r#"""#, ""),
            )
            .part("file", file_part)
    }
}

impl crate::types::CreateTranslationRequest {
    pub(crate) fn into_multipart_form(self) -> reqwest::multipart::Form {
        let file_part = reqwest::multipart::Part::bytes(self.file.file_data.clone())
            .mime_str("application/octet-stream")
            .unwrap()
            .file_name(self.file.file_name.clone());
        reqwest::multipart::Form::new()
            .text(
                "model",
                serde_json::to_string(&self.model)
                    .unwrap()
                    .replace(r#"""#, ""),
            )
            .part("file", file_part)
    }
}

impl crate::types::CreateContainerFileBody {
    pub(crate) fn into_multipart_form(self) -> reqwest::multipart::Form {
        let form = reqwest::multipart::Form::new();
        if let Some(file) = &self.file {
            let file_part = reqwest::multipart::Part::bytes(file.file_data.clone())
                .mime_str("application/octet-stream")
                .unwrap()
                .file_name(file.file_name.clone());
            form.part("file", file_part)
        } else {
            form
        }
    }
}

impl crate::types::CreateFileRequest {
    pub(crate) fn into_multipart_form(self) -> reqwest::multipart::Form {
        let file_part = reqwest::multipart::Part::bytes(self.file.file_data.clone())
            .mime_str("application/octet-stream")
            .unwrap()
            .file_name(self.file.file_name.clone());
        reqwest::multipart::Form::new()
            .text(
                "purpose",
                serde_json::to_string(&self.purpose)
                    .unwrap()
                    .replace(r#"""#, ""),
            )
            .part("file", file_part)
    }
}

impl crate::types::CreateImageEditRequest {
    pub(crate) fn into_multipart_form(self) -> reqwest::multipart::Form {
        todo!()
    }
}

impl crate::types::CreateImageVariationRequest {
    pub(crate) fn into_multipart_form(self) -> reqwest::multipart::Form {
        todo!()
    }
}

impl crate::types::AddUploadPartRequest {
    pub(crate) fn into_multipart_form(self) -> reqwest::multipart::Form {
        todo!()
    }
}
