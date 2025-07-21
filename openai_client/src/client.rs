use crate::{ConversaError, ConversaResult, OpenAIClient};
use crate::types::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateSpeechResponse {
	ApplicationOctetStream(Vec<u8>),
	TextEventStream(crate::types::CreateSpeechResponseStreamEvent),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateTranscriptionResponse {
	ApplicationJson(crate::types::CreateTranscriptionResponseJson),
	ApplicationJsonVerbose(crate::types::CreateTranscriptionResponseVerboseJson),
	TextEventStream(crate::types::CreateTranscriptionResponseStreamEvent),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranslationResponse {
	CreateTranslationResponseJson(CreateTranslationResponseJson),
	CreateTranslationResponseVerboseJson(CreateTranslationResponseVerboseJson),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateBatchRequestBodyEndpoint {
	#[serde(rename="/v1/responses")]
	V1Responses,
	#[serde(rename="/v1/chat/completions")]
	V1ChatCompletions,
	#[serde(rename="/v1/embeddings")]
	V1Embeddings,
	#[serde(rename="/v1/completions")]
	V1Completions,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateBatchRequestBodyCompletionWindow {
	#[serde(rename="24h")]
	Size24h,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateBatchRequestBody {
	/** The ID of an uploaded file that contains requests for the new batch.

See [upload file](/docs/api-reference/files/create) for how to upload a file.

Your input file must be formatted as a [JSONL file](/docs/api-reference/batch/request-input), and must be uploaded with the purpose `batch`. The file can contain up to 50,000 requests, and can be up to 200 MB in size. */
	pub input_file_id: String,
	/** The endpoint to be used for all requests in the batch. Currently `/v1/responses`, `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported. Note that `/v1/embeddings` batches are also restricted to a maximum of 50,000 embedding inputs across all requests in the batch. */
	pub endpoint: CreateBatchRequestBodyEndpoint,
	/** The time frame within which the batch should be processed. Currently only `24h` is supported. */
	pub completion_window: CreateBatchRequestBodyCompletionWindow,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChatCompletionResponse {
	ApplicationJson(crate::types::CreateChatCompletionResponse),
	TextEventStream(crate::types::CreateChatCompletionStreamResponse),
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateChatCompletionRequestBody {
	pub metadata: Metadata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateEvalRequestBody {
	/** Rename the evaluation. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteEvalResponse {
	pub object: String,
	pub deleted: bool,
	pub eval_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteEvalRunResponse {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deleted: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub run_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPaginatedFineTuningJobsQuery(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminApiKeysCreateRequestBody {
	pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminApiKeysDeleteResponse {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub deleted: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAuditLogsQuery {
	/** Return only events whose `effective_at` (Unix seconds) is greater than this value. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gt: Option<u64>,
	/** Return only events whose `effective_at` (Unix seconds) is greater than or equal to this value. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gte: Option<u64>,
	/** Return only events whose `effective_at` (Unix seconds) is less than this value. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lt: Option<u64>,
	/** Return only events whose `effective_at` (Unix seconds) is less than or equal to this value. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lte: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateResponseResponse {
	ApplicationJson(crate::types::Response),
	TextEventStream(crate::types::ResponseStreamEvent),
}
impl OpenAIClient {
	/** Returns a list of assistants. */
	pub async fn list_assistants(&self, limit: Option<u64>, order: Option<&str>, after: Option<&str>, before: Option<&str>, ) -> ConversaResult<crate::types::ListAssistantsResponse> {
		let address = format!("{}/assistants", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create an assistant with a model and instructions. */
	pub async fn create_assistant(&self, request_body: crate::types::CreateAssistantRequest, ) -> ConversaResult<crate::types::AssistantObject> {
		let address = format!("{}/assistants", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves an assistant. */
	pub async fn get_assistant(&self, assistant_id: &str, ) -> ConversaResult<crate::types::AssistantObject> {
		let address = format!("{}/assistants/{assistant_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modifies an assistant. */
	pub async fn modify_assistant(&self, assistant_id: &str, request_body: crate::types::ModifyAssistantRequest, ) -> ConversaResult<crate::types::AssistantObject> {
		let address = format!("{}/assistants/{assistant_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete an assistant. */
	pub async fn delete_assistant(&self, assistant_id: &str, ) -> ConversaResult<crate::types::DeleteAssistantResponse> {
		let address = format!("{}/assistants/{assistant_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Generates audio from the input text. */
	pub async fn create_speech(&self, request_body: crate::types::CreateSpeechRequest, ) -> ConversaResult<CreateSpeechResponse> {
		let address = format!("{}/audio/speech", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		match _content_type.as_str() {
			"application/octet-stream" => Ok(CreateSpeechResponse::ApplicationOctetStream(serde_json::from_slice(&response_bytes)?)),
			"text/event-stream" => Ok(CreateSpeechResponse::TextEventStream(serde_json::from_slice(&response_bytes)?)),
			_ => Err(ConversaError::UnexpectedContentType(_content_type)),
		}
	}

	/** Transcribes audio into the input language. */
	pub async fn create_transcription(&self, request_body: crate::types::CreateTranscriptionRequest, ) -> ConversaResult<CreateTranscriptionResponse> {
		let address = format!("{}/audio/transcriptions", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.multipart(request_body.into_multipart_form());
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		match _content_type.as_str() {
			"application/json" => Ok(CreateTranscriptionResponse::ApplicationJson(serde_json::from_slice(&response_bytes)?)),
			"text/event-stream" => Ok(CreateTranscriptionResponse::TextEventStream(serde_json::from_slice(&response_bytes)?)),
			_ => Err(ConversaError::UnexpectedContentType(_content_type)),
		}
	}

	/** Translates audio into English. */
	pub async fn create_translation(&self, request_body: crate::types::CreateTranslationRequest, ) -> ConversaResult<CreateTranslationResponse> {
		let address = format!("{}/audio/translations", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.multipart(request_body.into_multipart_form());
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates and executes a batch from an uploaded file of requests */
	pub async fn create_batch(&self, request_body: CreateBatchRequestBody, ) -> ConversaResult<crate::types::Batch> {
		let address = format!("{}/batches", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List your organization's batches. */
	pub async fn list_batches(&self, after: Option<&str>, limit: Option<u64>, ) -> ConversaResult<crate::types::ListBatchesResponse> {
		let address = format!("{}/batches", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a batch. */
	pub async fn retrieve_batch(&self, batch_id: &str, ) -> ConversaResult<crate::types::Batch> {
		let address = format!("{}/batches/{batch_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Cancels an in-progress batch. The batch will be in status `cancelling` for up to 10 minutes, before changing to `cancelled`, where it will have partial results (if any) available in the output file. */
	pub async fn cancel_batch(&self, batch_id: &str, ) -> ConversaResult<crate::types::Batch> {
		let address = format!("{}/batches/{batch_id}/cancel", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List stored Chat Completions. Only Chat Completions that have been stored
with the `store` parameter set to `true` will be returned. */
	pub async fn list_chat_completions(&self, model: Option<&str>, metadata: Option<&crate::types::Metadata>, after: Option<&str>, limit: Option<u64>, order: Option<&str>, ) -> ConversaResult<crate::types::ChatCompletionList> {
		let address = format!("{}/chat/completions", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = model {
			request = request.query(&[("model", q)]);
		}
		if let Some(q) = metadata {
			request = request.query(&[("metadata", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** **Starting a new project?** We recommend trying [Responses](/docs/api-reference/responses) 
to take advantage of the latest OpenAI platform features. Compare
[Chat Completions with Responses](/docs/guides/responses-vs-chat-completions?api-mode=responses).

---

Creates a model response for the given chat conversation. Learn more in the
[text generation](/docs/guides/text-generation), [vision](/docs/guides/vision),
and [audio](/docs/guides/audio) guides.

Parameter support can differ depending on the model used to generate the
response, particularly for newer reasoning models. Parameters that are only
supported for reasoning models are noted below. For the current state of 
unsupported parameters in reasoning models, 
[refer to the reasoning guide](/docs/guides/reasoning). */
	pub async fn create_chat_completion(&self, request_body: crate::types::CreateChatCompletionRequest, ) -> ConversaResult<CreateChatCompletionResponse> {
		let address = format!("{}/chat/completions", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		match _content_type.as_str() {
			"application/json" => Ok(CreateChatCompletionResponse::ApplicationJson(serde_json::from_slice(&response_bytes)?)),
			"text/event-stream" => Ok(CreateChatCompletionResponse::TextEventStream(serde_json::from_slice(&response_bytes)?)),
			_ => Err(ConversaError::UnexpectedContentType(_content_type)),
		}
	}

	/** Get a stored chat completion. Only Chat Completions that have been created
with the `store` parameter set to `true` will be returned. */
	pub async fn get_chat_completion(&self, completion_id: &str, ) -> ConversaResult<crate::types::CreateChatCompletionResponse> {
		let address = format!("{}/chat/completions/{completion_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modify a stored chat completion. Only Chat Completions that have been
created with the `store` parameter set to `true` can be modified. Currently,
the only supported modification is to update the `metadata` field. */
	pub async fn update_chat_completion(&self, completion_id: &str, request_body: UpdateChatCompletionRequestBody, ) -> ConversaResult<crate::types::CreateChatCompletionResponse> {
		let address = format!("{}/chat/completions/{completion_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete a stored chat completion. Only Chat Completions that have been
created with the `store` parameter set to `true` can be deleted. */
	pub async fn delete_chat_completion(&self, completion_id: &str, ) -> ConversaResult<crate::types::ChatCompletionDeleted> {
		let address = format!("{}/chat/completions/{completion_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get the messages in a stored chat completion. Only Chat Completions that
have been created with the `store` parameter set to `true` will be
returned. */
	pub async fn get_chat_completion_messages(&self, completion_id: &str, after: Option<&str>, limit: Option<u64>, order: Option<&str>, ) -> ConversaResult<crate::types::ChatCompletionMessageList> {
		let address = format!("{}/chat/completions/{completion_id}/messages", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates a completion for the provided prompt and parameters. */
	pub async fn create_completion(&self, request_body: crate::types::CreateCompletionRequest, ) -> ConversaResult<crate::types::CreateCompletionResponse> {
		let address = format!("{}/completions", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List Containers */
	pub async fn list_containers(&self, limit: Option<u64>, order: Option<&str>, after: Option<&str>, ) -> ConversaResult<crate::types::ContainerListResource> {
		let address = format!("{}/containers", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create Container */
	pub async fn create_container(&self, request_body: Option<crate::types::CreateContainerBody>, ) -> ConversaResult<crate::types::ContainerResource> {
		let address = format!("{}/containers", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(b) = request_body {
			request = request.body(serde_json::to_string(&b)?);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieve Container */
	pub async fn retrieve_container(&self, container_id: &str, ) -> ConversaResult<crate::types::ContainerResource> {
		let address = format!("{}/containers/{container_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete Container */
	pub async fn delete_container(&self, container_id: &str, ) -> ConversaResult<()> {
		let address = format!("{}/containers/{container_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(())
	}

	/** Create a Container File

You can send either a multipart/form-data request with the raw file content, or a JSON request with a file ID. */
	pub async fn create_container_file(&self, container_id: &str, request_body: crate::types::CreateContainerFileBody, ) -> ConversaResult<crate::types::ContainerFileResource> {
		let address = format!("{}/containers/{container_id}/files", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.multipart(request_body.into_multipart_form());
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List Container files */
	pub async fn list_container_files(&self, container_id: &str, limit: Option<u64>, order: Option<&str>, after: Option<&str>, ) -> ConversaResult<crate::types::ContainerFileListResource> {
		let address = format!("{}/containers/{container_id}/files", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieve Container File */
	pub async fn retrieve_container_file(&self, container_id: &str, file_id: &str, ) -> ConversaResult<crate::types::ContainerFileResource> {
		let address = format!("{}/containers/{container_id}/files/{file_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete Container File */
	pub async fn delete_container_file(&self, container_id: &str, file_id: &str, ) -> ConversaResult<()> {
		let address = format!("{}/containers/{container_id}/files/{file_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(())
	}

	/** Retrieve Container File Content */
	pub async fn retrieve_container_file_content(&self, container_id: &str, file_id: &str, ) -> ConversaResult<()> {
		let address = format!("{}/containers/{container_id}/files/{file_id}/content", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(())
	}

	/** Creates an embedding vector representing the input text. */
	pub async fn create_embedding(&self, request_body: crate::types::CreateEmbeddingRequest, ) -> ConversaResult<crate::types::CreateEmbeddingResponse> {
		let address = format!("{}/embeddings", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List evaluations for a project. */
	pub async fn list_evals(&self, after: Option<&str>, limit: Option<u64>, order: Option<&str>, order_by: Option<&str>, ) -> ConversaResult<crate::types::EvalList> {
		let address = format!("{}/evals", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = order_by {
			request = request.query(&[("order_by", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create the structure of an evaluation that can be used to test a model's performance.
An evaluation is a set of testing criteria and the config for a data source, which dictates the schema of the data used in the evaluation. After creating an evaluation, you can run it on different models and model parameters. We support several types of graders and datasources.
For more information, see the [Evals guide](/docs/guides/evals). */
	pub async fn create_eval(&self, request_body: crate::types::CreateEvalRequest, ) -> ConversaResult<crate::types::Eval> {
		let address = format!("{}/evals", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 201 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get an evaluation by ID. */
	pub async fn get_eval(&self, eval_id: &str, ) -> ConversaResult<crate::types::Eval> {
		let address = format!("{}/evals/{eval_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Update certain properties of an evaluation. */
	pub async fn update_eval(&self, eval_id: &str, request_body: UpdateEvalRequestBody, ) -> ConversaResult<crate::types::Eval> {
		let address = format!("{}/evals/{eval_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete an evaluation. */
	pub async fn delete_eval(&self, eval_id: &str, ) -> ConversaResult<DeleteEvalResponse> {
		let address = format!("{}/evals/{eval_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get a list of runs for an evaluation. */
	pub async fn get_eval_runs(&self, eval_id: &str, after: Option<&str>, limit: Option<u64>, order: Option<&str>, status: Option<&str>, ) -> ConversaResult<crate::types::EvalRunList> {
		let address = format!("{}/evals/{eval_id}/runs", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = status {
			request = request.query(&[("status", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Kicks off a new run for a given evaluation, specifying the data source, and what model configuration to use to test. The datasource will be validated against the schema specified in the config of the evaluation. */
	pub async fn create_eval_run(&self, eval_id: &str, request_body: crate::types::CreateEvalRunRequest, ) -> ConversaResult<crate::types::EvalRun> {
		let address = format!("{}/evals/{eval_id}/runs", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 201 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get an evaluation run by ID. */
	pub async fn get_eval_run(&self, eval_id: &str, run_id: &str, ) -> ConversaResult<crate::types::EvalRun> {
		let address = format!("{}/evals/{eval_id}/runs/{run_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Cancel an ongoing evaluation run. */
	pub async fn cancel_eval_run(&self, eval_id: &str, run_id: &str, ) -> ConversaResult<crate::types::EvalRun> {
		let address = format!("{}/evals/{eval_id}/runs/{run_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete an eval run. */
	pub async fn delete_eval_run(&self, eval_id: &str, run_id: &str, ) -> ConversaResult<DeleteEvalRunResponse> {
		let address = format!("{}/evals/{eval_id}/runs/{run_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get a list of output items for an evaluation run. */
	pub async fn get_eval_run_output_items(&self, eval_id: &str, run_id: &str, after: Option<&str>, limit: Option<u64>, status: Option<&str>, order: Option<&str>, ) -> ConversaResult<crate::types::EvalRunOutputItemList> {
		let address = format!("{}/evals/{eval_id}/runs/{run_id}/output_items", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = status {
			request = request.query(&[("status", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get an evaluation run output item by ID. */
	pub async fn get_eval_run_output_item(&self, eval_id: &str, run_id: &str, output_item_id: &str, ) -> ConversaResult<crate::types::EvalRunOutputItem> {
		let address = format!("{}/evals/{eval_id}/runs/{run_id}/output_items/{output_item_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of files. */
	pub async fn list_files(&self, purpose: Option<&str>, limit: Option<u64>, order: Option<&str>, after: Option<&str>, ) -> ConversaResult<crate::types::ListFilesResponse> {
		let address = format!("{}/files", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = purpose {
			request = request.query(&[("purpose", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Upload a file that can be used across various endpoints. Individual files can be up to 512 MB, and the size of all files uploaded by one organization can be up to 100 GB.

The Assistants API supports files up to 2 million tokens and of specific file types. See the [Assistants Tools guide](/docs/assistants/tools) for details.

The Fine-tuning API only supports `.jsonl` files. The input also has certain required formats for fine-tuning [chat](/docs/api-reference/fine-tuning/chat-input) or [completions](/docs/api-reference/fine-tuning/completions-input) models.

The Batch API only supports `.jsonl` files up to 200 MB in size. The input also has a specific required [format](/docs/api-reference/batch/request-input).

Please [contact us](https://help.openai.com/) if you need to increase these storage limits. */
	pub async fn create_file(&self, request_body: crate::types::CreateFileRequest, ) -> ConversaResult<crate::types::OpenAIFile> {
		let address = format!("{}/files", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.multipart(request_body.into_multipart_form());
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete a file. */
	pub async fn delete_file(&self, file_id: &str, ) -> ConversaResult<crate::types::DeleteFileResponse> {
		let address = format!("{}/files/{file_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns information about a specific file. */
	pub async fn retrieve_file(&self, file_id: &str, ) -> ConversaResult<crate::types::OpenAIFile> {
		let address = format!("{}/files/{file_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns the contents of the specified file. */
	pub async fn download_file(&self, file_id: &str, ) -> ConversaResult<String> {
		let address = format!("{}/files/{file_id}/content", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Run a grader. */
	pub async fn run_grader(&self, request_body: crate::types::RunGraderRequest, ) -> ConversaResult<crate::types::RunGraderResponse> {
		let address = format!("{}/fine_tuning/alpha/graders/run", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Validate a grader. */
	pub async fn validate_grader(&self, request_body: crate::types::ValidateGraderRequest, ) -> ConversaResult<crate::types::ValidateGraderResponse> {
		let address = format!("{}/fine_tuning/alpha/graders/validate", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** **NOTE:** This endpoint requires an [admin API key](../admin-api-keys).

Organization owners can use this endpoint to view all permissions for a fine-tuned model checkpoint. */
	pub async fn list_fine_tuning_checkpoint_permissions(&self, fine_tuned_model_checkpoint: &str, project_id: Option<&str>, after: Option<&str>, limit: Option<u64>, order: Option<&str>, ) -> ConversaResult<crate::types::ListFineTuningCheckpointPermissionResponse> {
		let address = format!("{}/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = project_id {
			request = request.query(&[("project_id", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** **NOTE:** Calling this endpoint requires an [admin API key](../admin-api-keys).

This enables organization owners to share fine-tuned models with other projects in their organization. */
	pub async fn create_fine_tuning_checkpoint_permission(&self, fine_tuned_model_checkpoint: &str, request_body: crate::types::CreateFineTuningCheckpointPermissionRequest, ) -> ConversaResult<crate::types::ListFineTuningCheckpointPermissionResponse> {
		let address = format!("{}/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** **NOTE:** This endpoint requires an [admin API key](../admin-api-keys).

Organization owners can use this endpoint to delete a permission for a fine-tuned model checkpoint. */
	pub async fn delete_fine_tuning_checkpoint_permission(&self, fine_tuned_model_checkpoint: &str, permission_id: &str, ) -> ConversaResult<crate::types::DeleteFineTuningCheckpointPermissionResponse> {
		let address = format!("{}/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions/{permission_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates a fine-tuning job which begins the process of creating a new model from a given dataset.

Response includes details of the enqueued job including job status and the name of the fine-tuned models once complete.

[Learn more about fine-tuning](/docs/guides/model-optimization) */
	pub async fn create_fine_tuning_job(&self, request_body: crate::types::CreateFineTuningJobRequest, ) -> ConversaResult<crate::types::FineTuningJob> {
		let address = format!("{}/fine_tuning/jobs", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List your organization's fine-tuning jobs */
	pub async fn list_paginated_fine_tuning_jobs(&self, after: Option<&str>, limit: Option<u64>, metadata: Option<ListPaginatedFineTuningJobsQuery>, ) -> ConversaResult<crate::types::ListPaginatedFineTuningJobsResponse> {
		let address = format!("{}/fine_tuning/jobs", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = metadata {
			request = request.query(&[("metadata", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get info about a fine-tuning job.

[Learn more about fine-tuning](/docs/guides/model-optimization) */
	pub async fn retrieve_fine_tuning_job(&self, fine_tuning_job_id: &str, ) -> ConversaResult<crate::types::FineTuningJob> {
		let address = format!("{}/fine_tuning/jobs/{fine_tuning_job_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Immediately cancel a fine-tune job. */
	pub async fn cancel_fine_tuning_job(&self, fine_tuning_job_id: &str, ) -> ConversaResult<crate::types::FineTuningJob> {
		let address = format!("{}/fine_tuning/jobs/{fine_tuning_job_id}/cancel", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List checkpoints for a fine-tuning job. */
	pub async fn list_fine_tuning_job_checkpoints(&self, fine_tuning_job_id: &str, after: Option<&str>, limit: Option<u64>, ) -> ConversaResult<crate::types::ListFineTuningJobCheckpointsResponse> {
		let address = format!("{}/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get status updates for a fine-tuning job. */
	pub async fn list_fine_tuning_events(&self, fine_tuning_job_id: &str, after: Option<&str>, limit: Option<u64>, ) -> ConversaResult<crate::types::ListFineTuningJobEventsResponse> {
		let address = format!("{}/fine_tuning/jobs/{fine_tuning_job_id}/events", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Pause a fine-tune job. */
	pub async fn pause_fine_tuning_job(&self, fine_tuning_job_id: &str, ) -> ConversaResult<crate::types::FineTuningJob> {
		let address = format!("{}/fine_tuning/jobs/{fine_tuning_job_id}/pause", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Resume a fine-tune job. */
	pub async fn resume_fine_tuning_job(&self, fine_tuning_job_id: &str, ) -> ConversaResult<crate::types::FineTuningJob> {
		let address = format!("{}/fine_tuning/jobs/{fine_tuning_job_id}/resume", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates an edited or extended image given one or more source images and a prompt. This endpoint only supports `gpt-image-1` and `dall-e-2`. */
	pub async fn create_image_edit(&self, request_body: crate::types::CreateImageEditRequest, ) -> ConversaResult<crate::types::ImagesResponse> {
		let address = format!("{}/images/edits", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.multipart(request_body.into_multipart_form());
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates an image given a prompt. [Learn more](/docs/guides/images). */
	pub async fn create_image(&self, request_body: crate::types::CreateImageRequest, ) -> ConversaResult<crate::types::ImagesResponse> {
		let address = format!("{}/images/generations", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates a variation of a given image. This endpoint only supports `dall-e-2`. */
	pub async fn create_image_variation(&self, request_body: crate::types::CreateImageVariationRequest, ) -> ConversaResult<crate::types::ImagesResponse> {
		let address = format!("{}/images/variations", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.multipart(request_body.into_multipart_form());
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Lists the currently available models, and provides basic information about each one such as the owner and availability. */
	pub async fn list_models(&self, ) -> ConversaResult<crate::types::ListModelsResponse> {
		let address = format!("{}/models", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a model instance, providing basic information about the model such as the owner and permissioning. */
	pub async fn retrieve_model(&self, model: &str, ) -> ConversaResult<crate::types::Model> {
		let address = format!("{}/models/{model}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete a fine-tuned model. You must have the Owner role in your organization to delete a model. */
	pub async fn delete_model(&self, model: &str, ) -> ConversaResult<crate::types::DeleteModelResponse> {
		let address = format!("{}/models/{model}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Classifies if text and/or image inputs are potentially harmful. Learn
more in the [moderation guide](/docs/guides/moderation). */
	pub async fn create_moderation(&self, request_body: crate::types::CreateModerationRequest, ) -> ConversaResult<crate::types::CreateModerationResponse> {
		let address = format!("{}/moderations", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List organization API keys */
	pub async fn admin_api_keys_list(&self, after: Option<&str>, order: Option<&str>, limit: Option<u64>, ) -> ConversaResult<crate::types::ApiKeyList> {
		let address = format!("{}/organization/admin_api_keys", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create an organization admin API key */
	pub async fn admin_api_keys_create(&self, request_body: AdminApiKeysCreateRequestBody, ) -> ConversaResult<crate::types::AdminApiKey> {
		let address = format!("{}/organization/admin_api_keys", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieve a single organization API key */
	pub async fn admin_api_keys_get(&self, key_id: &str, ) -> ConversaResult<crate::types::AdminApiKey> {
		let address = format!("{}/organization/admin_api_keys/{key_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete an organization admin API key */
	pub async fn admin_api_keys_delete(&self, key_id: &str, ) -> ConversaResult<AdminApiKeysDeleteResponse> {
		let address = format!("{}/organization/admin_api_keys/{key_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List user actions and configuration changes within this organization. */
	pub async fn list_audit_logs(&self, effective_at: Option<ListAuditLogsQuery>, project_ids: Option<&[String]>, event_types: Option<&[crate::types::AuditLogEventType]>, actor_ids: Option<&[String]>, actor_emails: Option<&[String]>, resource_ids: Option<&[String]>, limit: Option<u64>, after: Option<&str>, before: Option<&str>, ) -> ConversaResult<crate::types::ListAuditLogsResponse> {
		let address = format!("{}/organization/audit_logs", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = effective_at {
			request = request.query(&[("effective_at", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = event_types {
			request = request.query(&[("event_types", q)]);
		}
		if let Some(q) = actor_ids {
			request = request.query(&[("actor_ids", q)]);
		}
		if let Some(q) = actor_emails {
			request = request.query(&[("actor_emails", q)]);
		}
		if let Some(q) = resource_ids {
			request = request.query(&[("resource_ids", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List uploaded certificates for this organization. */
	pub async fn list_organization_certificates(&self, limit: Option<u64>, after: Option<&str>, order: Option<&str>, ) -> ConversaResult<crate::types::ListCertificatesResponse> {
		let address = format!("{}/organization/certificates", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Upload a certificate to the organization. This does **not** automatically activate the certificate.

Organizations can upload up to 50 certificates. */
	pub async fn upload_certificate(&self, request_body: crate::types::UploadCertificateRequest, ) -> ConversaResult<crate::types::Certificate> {
		let address = format!("{}/organization/certificates", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Activate certificates at the organization level.

You can atomically and idempotently activate up to 10 certificates at a time. */
	pub async fn activate_organization_certificates(&self, request_body: crate::types::ToggleCertificatesRequest, ) -> ConversaResult<crate::types::ListCertificatesResponse> {
		let address = format!("{}/organization/certificates/activate", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Deactivate certificates at the organization level.

You can atomically and idempotently deactivate up to 10 certificates at a time. */
	pub async fn deactivate_organization_certificates(&self, request_body: crate::types::ToggleCertificatesRequest, ) -> ConversaResult<crate::types::ListCertificatesResponse> {
		let address = format!("{}/organization/certificates/deactivate", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get a certificate that has been uploaded to the organization.

You can get a certificate regardless of whether it is active or not. */
	pub async fn get_certificate(&self, certificate_id: &str, include: Option<&[String]>, ) -> ConversaResult<crate::types::Certificate> {
		let address = format!("{}/organization/certificates/{certificate_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = include {
			request = request.query(&[("include", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modify a certificate. Note that only the name can be modified. */
	pub async fn modify_certificate(&self, certificate_id: &str, request_body: crate::types::ModifyCertificateRequest, ) -> ConversaResult<crate::types::Certificate> {
		let address = format!("{}/organization/certificates/{certificate_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete a certificate from the organization.

The certificate must be inactive for the organization and all projects. */
	pub async fn delete_certificate(&self, certificate_id: &str, ) -> ConversaResult<crate::types::DeleteCertificateResponse> {
		let address = format!("{}/organization/certificates/{certificate_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get costs details for the organization. */
	pub async fn usage_costs(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, project_ids: Option<&[String]>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/costs", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of invites in the organization. */
	pub async fn list_invites(&self, limit: Option<u64>, after: Option<&str>, ) -> ConversaResult<crate::types::InviteListResponse> {
		let address = format!("{}/organization/invites", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create an invite for a user to the organization. The invite must be accepted by the user before they have access to the organization. */
	pub async fn invite_user(&self, request_body: crate::types::InviteRequest, ) -> ConversaResult<crate::types::Invite> {
		let address = format!("{}/organization/invites", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves an invite. */
	pub async fn retrieve_invite(&self, invite_id: &str, ) -> ConversaResult<crate::types::Invite> {
		let address = format!("{}/organization/invites/{invite_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete an invite. If the invite has already been accepted, it cannot be deleted. */
	pub async fn delete_invite(&self, invite_id: &str, ) -> ConversaResult<crate::types::InviteDeleteResponse> {
		let address = format!("{}/organization/invites/{invite_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of projects. */
	pub async fn list_projects(&self, limit: Option<u64>, after: Option<&str>, include_archived: Option<bool>, ) -> ConversaResult<crate::types::ProjectListResponse> {
		let address = format!("{}/organization/projects", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = include_archived {
			request = request.query(&[("include_archived", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create a new project in the organization. Projects can be created and archived, but cannot be deleted. */
	pub async fn create_project(&self, request_body: crate::types::ProjectCreateRequest, ) -> ConversaResult<crate::types::Project> {
		let address = format!("{}/organization/projects", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a project. */
	pub async fn retrieve_project(&self, project_id: &str, ) -> ConversaResult<crate::types::Project> {
		let address = format!("{}/organization/projects/{project_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modifies a project in the organization. */
	pub async fn modify_project(&self, project_id: &str, request_body: crate::types::ProjectUpdateRequest, ) -> ConversaResult<crate::types::Project> {
		let address = format!("{}/organization/projects/{project_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of API keys in the project. */
	pub async fn list_project_api_keys(&self, project_id: &str, limit: Option<u64>, after: Option<&str>, ) -> ConversaResult<crate::types::ProjectApiKeyListResponse> {
		let address = format!("{}/organization/projects/{project_id}/api_keys", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves an API key in the project. */
	pub async fn retrieve_project_api_key(&self, project_id: &str, key_id: &str, ) -> ConversaResult<crate::types::ProjectApiKey> {
		let address = format!("{}/organization/projects/{project_id}/api_keys/{key_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Deletes an API key from the project. */
	pub async fn delete_project_api_key(&self, project_id: &str, key_id: &str, ) -> ConversaResult<crate::types::ProjectApiKeyDeleteResponse> {
		let address = format!("{}/organization/projects/{project_id}/api_keys/{key_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Archives a project in the organization. Archived projects cannot be used or updated. */
	pub async fn archive_project(&self, project_id: &str, ) -> ConversaResult<crate::types::Project> {
		let address = format!("{}/organization/projects/{project_id}/archive", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** List certificates for this project. */
	pub async fn list_project_certificates(&self, project_id: &str, limit: Option<u64>, after: Option<&str>, order: Option<&str>, ) -> ConversaResult<crate::types::ListCertificatesResponse> {
		let address = format!("{}/organization/projects/{project_id}/certificates", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Activate certificates at the project level.

You can atomically and idempotently activate up to 10 certificates at a time. */
	pub async fn activate_project_certificates(&self, project_id: &str, request_body: crate::types::ToggleCertificatesRequest, ) -> ConversaResult<crate::types::ListCertificatesResponse> {
		let address = format!("{}/organization/projects/{project_id}/certificates/activate", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Deactivate certificates at the project level. You can atomically and 
idempotently deactivate up to 10 certificates at a time. */
	pub async fn deactivate_project_certificates(&self, project_id: &str, request_body: crate::types::ToggleCertificatesRequest, ) -> ConversaResult<crate::types::ListCertificatesResponse> {
		let address = format!("{}/organization/projects/{project_id}/certificates/deactivate", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns the rate limits per model for a project. */
	pub async fn list_project_rate_limits(&self, project_id: &str, limit: Option<u64>, after: Option<&str>, before: Option<&str>, ) -> ConversaResult<crate::types::ProjectRateLimitListResponse> {
		let address = format!("{}/organization/projects/{project_id}/rate_limits", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Updates a project rate limit. */
	pub async fn update_project_rate_limits(&self, project_id: &str, rate_limit_id: &str, request_body: crate::types::ProjectRateLimitUpdateRequest, ) -> ConversaResult<crate::types::ProjectRateLimit> {
		let address = format!("{}/organization/projects/{project_id}/rate_limits/{rate_limit_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of service accounts in the project. */
	pub async fn list_project_service_accounts(&self, project_id: &str, limit: Option<u64>, after: Option<&str>, ) -> ConversaResult<crate::types::ProjectServiceAccountListResponse> {
		let address = format!("{}/organization/projects/{project_id}/service_accounts", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates a new service account in the project. This also returns an unredacted API key for the service account. */
	pub async fn create_project_service_account(&self, project_id: &str, request_body: crate::types::ProjectServiceAccountCreateRequest, ) -> ConversaResult<crate::types::ProjectServiceAccountCreateResponse> {
		let address = format!("{}/organization/projects/{project_id}/service_accounts", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a service account in the project. */
	pub async fn retrieve_project_service_account(&self, project_id: &str, service_account_id: &str, ) -> ConversaResult<crate::types::ProjectServiceAccount> {
		let address = format!("{}/organization/projects/{project_id}/service_accounts/{service_account_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Deletes a service account from the project. */
	pub async fn delete_project_service_account(&self, project_id: &str, service_account_id: &str, ) -> ConversaResult<crate::types::ProjectServiceAccountDeleteResponse> {
		let address = format!("{}/organization/projects/{project_id}/service_accounts/{service_account_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of users in the project. */
	pub async fn list_project_users(&self, project_id: &str, limit: Option<u64>, after: Option<&str>, ) -> ConversaResult<crate::types::ProjectUserListResponse> {
		let address = format!("{}/organization/projects/{project_id}/users", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Adds a user to the project. Users must already be members of the organization to be added to a project. */
	pub async fn create_project_user(&self, project_id: &str, request_body: crate::types::ProjectUserCreateRequest, ) -> ConversaResult<crate::types::ProjectUser> {
		let address = format!("{}/organization/projects/{project_id}/users", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a user in the project. */
	pub async fn retrieve_project_user(&self, project_id: &str, user_id: &str, ) -> ConversaResult<crate::types::ProjectUser> {
		let address = format!("{}/organization/projects/{project_id}/users/{user_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modifies a user's role in the project. */
	pub async fn modify_project_user(&self, project_id: &str, user_id: &str, request_body: crate::types::ProjectUserUpdateRequest, ) -> ConversaResult<crate::types::ProjectUser> {
		let address = format!("{}/organization/projects/{project_id}/users/{user_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Deletes a user from the project. */
	pub async fn delete_project_user(&self, project_id: &str, user_id: &str, ) -> ConversaResult<crate::types::ProjectUserDeleteResponse> {
		let address = format!("{}/organization/projects/{project_id}/users/{user_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get audio speeches usage details for the organization. */
	pub async fn usage_audio_speeches(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, project_ids: Option<&[String]>, user_ids: Option<&[String]>, api_key_ids: Option<&[String]>, models: Option<&[String]>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/usage/audio_speeches", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = user_ids {
			request = request.query(&[("user_ids", q)]);
		}
		if let Some(q) = api_key_ids {
			request = request.query(&[("api_key_ids", q)]);
		}
		if let Some(q) = models {
			request = request.query(&[("models", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get audio transcriptions usage details for the organization. */
	pub async fn usage_audio_transcriptions(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, project_ids: Option<&[String]>, user_ids: Option<&[String]>, api_key_ids: Option<&[String]>, models: Option<&[String]>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/usage/audio_transcriptions", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = user_ids {
			request = request.query(&[("user_ids", q)]);
		}
		if let Some(q) = api_key_ids {
			request = request.query(&[("api_key_ids", q)]);
		}
		if let Some(q) = models {
			request = request.query(&[("models", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get code interpreter sessions usage details for the organization. */
	pub async fn usage_code_interpreter_sessions(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, project_ids: Option<&[String]>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/usage/code_interpreter_sessions", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get completions usage details for the organization. */
	pub async fn usage_completions(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, project_ids: Option<&[String]>, user_ids: Option<&[String]>, api_key_ids: Option<&[String]>, models: Option<&[String]>, batch: Option<bool>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/usage/completions", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = user_ids {
			request = request.query(&[("user_ids", q)]);
		}
		if let Some(q) = api_key_ids {
			request = request.query(&[("api_key_ids", q)]);
		}
		if let Some(q) = models {
			request = request.query(&[("models", q)]);
		}
		if let Some(q) = batch {
			request = request.query(&[("batch", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get embeddings usage details for the organization. */
	pub async fn usage_embeddings(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, project_ids: Option<&[String]>, user_ids: Option<&[String]>, api_key_ids: Option<&[String]>, models: Option<&[String]>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/usage/embeddings", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = user_ids {
			request = request.query(&[("user_ids", q)]);
		}
		if let Some(q) = api_key_ids {
			request = request.query(&[("api_key_ids", q)]);
		}
		if let Some(q) = models {
			request = request.query(&[("models", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get images usage details for the organization. */
	pub async fn usage_images(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, sources: Option<&[String]>, sizes: Option<&[String]>, project_ids: Option<&[String]>, user_ids: Option<&[String]>, api_key_ids: Option<&[String]>, models: Option<&[String]>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/usage/images", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = sources {
			request = request.query(&[("sources", q)]);
		}
		if let Some(q) = sizes {
			request = request.query(&[("sizes", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = user_ids {
			request = request.query(&[("user_ids", q)]);
		}
		if let Some(q) = api_key_ids {
			request = request.query(&[("api_key_ids", q)]);
		}
		if let Some(q) = models {
			request = request.query(&[("models", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get moderations usage details for the organization. */
	pub async fn usage_moderations(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, project_ids: Option<&[String]>, user_ids: Option<&[String]>, api_key_ids: Option<&[String]>, models: Option<&[String]>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/usage/moderations", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = user_ids {
			request = request.query(&[("user_ids", q)]);
		}
		if let Some(q) = api_key_ids {
			request = request.query(&[("api_key_ids", q)]);
		}
		if let Some(q) = models {
			request = request.query(&[("models", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Get vector stores usage details for the organization. */
	pub async fn usage_vector_stores(&self, start_time: u64, end_time: Option<u64>, bucket_width: Option<&str>, project_ids: Option<&[String]>, group_by: Option<&[String]>, limit: Option<u64>, page: Option<&str>, ) -> ConversaResult<crate::types::UsageResponse> {
		let address = format!("{}/organization/usage/vector_stores", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.query(&[("start_time",start_time)]);
		if let Some(q) = end_time {
			request = request.query(&[("end_time", q)]);
		}
		if let Some(q) = bucket_width {
			request = request.query(&[("bucket_width", q)]);
		}
		if let Some(q) = project_ids {
			request = request.query(&[("project_ids", q)]);
		}
		if let Some(q) = group_by {
			request = request.query(&[("group_by", q)]);
		}
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = page {
			request = request.query(&[("page", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Lists all of the users in the organization. */
	pub async fn list_users(&self, limit: Option<u64>, after: Option<&str>, emails: Option<&[String]>, ) -> ConversaResult<crate::types::UserListResponse> {
		let address = format!("{}/organization/users", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = emails {
			request = request.query(&[("emails", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a user by their identifier. */
	pub async fn retrieve_user(&self, user_id: &str, ) -> ConversaResult<crate::types::User> {
		let address = format!("{}/organization/users/{user_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modifies a user's role in the organization. */
	pub async fn modify_user(&self, user_id: &str, request_body: crate::types::UserRoleUpdateRequest, ) -> ConversaResult<crate::types::User> {
		let address = format!("{}/organization/users/{user_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Deletes a user from the organization. */
	pub async fn delete_user(&self, user_id: &str, ) -> ConversaResult<crate::types::UserDeleteResponse> {
		let address = format!("{}/organization/users/{user_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create an ephemeral API token for use in client-side applications with the
Realtime API. Can be configured with the same session parameters as the
`session.update` client event.

It responds with a session object, plus a `client_secret` key which contains
a usable ephemeral API token that can be used to authenticate browser clients
for the Realtime API. */
	pub async fn create_realtime_session(&self, request_body: crate::types::RealtimeSessionCreateRequest, ) -> ConversaResult<crate::types::RealtimeSessionCreateResponse> {
		let address = format!("{}/realtime/sessions", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create an ephemeral API token for use in client-side applications with the
Realtime API specifically for realtime transcriptions. 
Can be configured with the same session parameters as the `transcription_session.update` client event.

It responds with a session object, plus a `client_secret` key which contains
a usable ephemeral API token that can be used to authenticate browser clients
for the Realtime API. */
	pub async fn create_realtime_transcription_session(&self, request_body: crate::types::RealtimeTranscriptionSessionCreateRequest, ) -> ConversaResult<crate::types::RealtimeTranscriptionSessionCreateResponse> {
		let address = format!("{}/realtime/transcription_sessions", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates a model response. Provide [text](/docs/guides/text) or
[image](/docs/guides/images) inputs to generate [text](/docs/guides/text)
or [JSON](/docs/guides/structured-outputs) outputs. Have the model call
your own [custom code](/docs/guides/function-calling) or use built-in
[tools](/docs/guides/tools) like [web search](/docs/guides/tools-web-search)
or [file search](/docs/guides/tools-file-search) to use your own data
as input for the model's response. */
	pub async fn create_response(&self, request_body: crate::types::CreateResponse, ) -> ConversaResult<CreateResponseResponse> {
		let address = format!("{}/responses", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		match _content_type.as_str() {
			"application/json" => Ok(CreateResponseResponse::ApplicationJson(serde_json::from_slice(&response_bytes)?)),
			"text/event-stream" => Ok(CreateResponseResponse::TextEventStream(serde_json::from_slice(&response_bytes)?)),
			_ => Err(ConversaError::UnexpectedContentType(_content_type)),
		}
	}

	/** Retrieves a model response with the given ID. */
	pub async fn get_response(&self, response_id: &str, include: Option<&[crate::types::Includable]>, stream: Option<bool>, starting_after: Option<u64>, ) -> ConversaResult<crate::types::Response> {
		let address = format!("{}/responses/{response_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = include {
			request = request.query(&[("include", q)]);
		}
		if let Some(q) = stream {
			request = request.query(&[("stream", q)]);
		}
		if let Some(q) = starting_after {
			request = request.query(&[("starting_after", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Deletes a model response with the given ID. */
	pub async fn delete_response(&self, response_id: &str, ) -> ConversaResult<()> {
		let address = format!("{}/responses/{response_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(())
	}

	/** Cancels a model response with the given ID. Only responses created with
the `background` parameter set to `true` can be cancelled. 
[Learn more](/docs/guides/background). */
	pub async fn cancel_response(&self, response_id: &str, ) -> ConversaResult<crate::types::Response> {
		let address = format!("{}/responses/{response_id}/cancel", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of input items for a given response. */
	pub async fn list_input_items(&self, response_id: &str, limit: Option<u64>, order: Option<&str>, after: Option<&str>, before: Option<&str>, include: Option<&[crate::types::Includable]>, ) -> ConversaResult<crate::types::ResponseItemList> {
		let address = format!("{}/responses/{response_id}/input_items", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		if let Some(q) = include {
			request = request.query(&[("include", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create a thread. */
	pub async fn create_thread(&self, request_body: Option<crate::types::CreateThreadRequest>, ) -> ConversaResult<crate::types::ThreadObject> {
		let address = format!("{}/threads", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(b) = request_body {
			request = request.body(serde_json::to_string(&b)?);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create a thread and run it in one request. */
	pub async fn create_thread_and_run(&self, request_body: crate::types::CreateThreadAndRunRequest, ) -> ConversaResult<crate::types::RunObject> {
		let address = format!("{}/threads/runs", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a thread. */
	pub async fn get_thread(&self, thread_id: &str, ) -> ConversaResult<crate::types::ThreadObject> {
		let address = format!("{}/threads/{thread_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modifies a thread. */
	pub async fn modify_thread(&self, thread_id: &str, request_body: crate::types::ModifyThreadRequest, ) -> ConversaResult<crate::types::ThreadObject> {
		let address = format!("{}/threads/{thread_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete a thread. */
	pub async fn delete_thread(&self, thread_id: &str, ) -> ConversaResult<crate::types::DeleteThreadResponse> {
		let address = format!("{}/threads/{thread_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of messages for a given thread. */
	pub async fn list_messages(&self, thread_id: &str, limit: Option<u64>, order: Option<&str>, after: Option<&str>, before: Option<&str>, run_id: Option<&str>, ) -> ConversaResult<crate::types::ListMessagesResponse> {
		let address = format!("{}/threads/{thread_id}/messages", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		if let Some(q) = run_id {
			request = request.query(&[("run_id", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create a message. */
	pub async fn create_message(&self, thread_id: &str, request_body: crate::types::CreateMessageRequest, ) -> ConversaResult<crate::types::MessageObject> {
		let address = format!("{}/threads/{thread_id}/messages", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieve a message. */
	pub async fn get_message(&self, thread_id: &str, message_id: &str, ) -> ConversaResult<crate::types::MessageObject> {
		let address = format!("{}/threads/{thread_id}/messages/{message_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modifies a message. */
	pub async fn modify_message(&self, thread_id: &str, message_id: &str, request_body: crate::types::ModifyMessageRequest, ) -> ConversaResult<crate::types::MessageObject> {
		let address = format!("{}/threads/{thread_id}/messages/{message_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Deletes a message. */
	pub async fn delete_message(&self, thread_id: &str, message_id: &str, ) -> ConversaResult<crate::types::DeleteMessageResponse> {
		let address = format!("{}/threads/{thread_id}/messages/{message_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of runs belonging to a thread. */
	pub async fn list_runs(&self, thread_id: &str, limit: Option<u64>, order: Option<&str>, after: Option<&str>, before: Option<&str>, ) -> ConversaResult<crate::types::ListRunsResponse> {
		let address = format!("{}/threads/{thread_id}/runs", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create a run. */
	pub async fn create_run(&self, thread_id: &str, include: Option<&[String]>, request_body: crate::types::CreateRunRequest, ) -> ConversaResult<crate::types::RunObject> {
		let address = format!("{}/threads/{thread_id}/runs", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = include {
			request = request.query(&[("include", q)]);
		}
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a run. */
	pub async fn get_run(&self, thread_id: &str, run_id: &str, ) -> ConversaResult<crate::types::RunObject> {
		let address = format!("{}/threads/{thread_id}/runs/{run_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modifies a run. */
	pub async fn modify_run(&self, thread_id: &str, run_id: &str, request_body: crate::types::ModifyRunRequest, ) -> ConversaResult<crate::types::RunObject> {
		let address = format!("{}/threads/{thread_id}/runs/{run_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Cancels a run that is `in_progress`. */
	pub async fn cancel_run(&self, thread_id: &str, run_id: &str, ) -> ConversaResult<crate::types::RunObject> {
		let address = format!("{}/threads/{thread_id}/runs/{run_id}/cancel", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of run steps belonging to a run. */
	pub async fn list_run_steps(&self, thread_id: &str, run_id: &str, limit: Option<u64>, order: Option<&str>, after: Option<&str>, before: Option<&str>, include: Option<&[String]>, ) -> ConversaResult<crate::types::ListRunStepsResponse> {
		let address = format!("{}/threads/{thread_id}/runs/{run_id}/steps", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		if let Some(q) = include {
			request = request.query(&[("include", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a run step. */
	pub async fn get_run_step(&self, thread_id: &str, run_id: &str, step_id: &str, include: Option<&[String]>, ) -> ConversaResult<crate::types::RunStepObject> {
		let address = format!("{}/threads/{thread_id}/runs/{run_id}/steps/{step_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = include {
			request = request.query(&[("include", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** When a run has the `status: "requires_action"` and `required_action.type` is `submit_tool_outputs`, this endpoint can be used to submit the outputs from the tool calls once they're all completed. All outputs must be submitted in a single request. */
	pub async fn submit_tool_ouputs_to_run(&self, thread_id: &str, run_id: &str, request_body: crate::types::SubmitToolOutputsRunRequest, ) -> ConversaResult<crate::types::RunObject> {
		let address = format!("{}/threads/{thread_id}/runs/{run_id}/submit_tool_outputs", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Creates an intermediate [Upload](/docs/api-reference/uploads/object) object
that you can add [Parts](/docs/api-reference/uploads/part-object) to.
Currently, an Upload can accept at most 8 GB in total and expires after an
hour after you create it.

Once you complete the Upload, we will create a
[File](/docs/api-reference/files/object) object that contains all the parts
you uploaded. This File is usable in the rest of our platform as a regular
File object.

For certain `purpose` values, the correct `mime_type` must be specified. 
Please refer to documentation for the 
[supported MIME types for your use case](/docs/assistants/tools/file-search#supported-files).

For guidance on the proper filename extensions for each purpose, please
follow the documentation on [creating a
File](/docs/api-reference/files/create). */
	pub async fn create_upload(&self, request_body: crate::types::CreateUploadRequest, ) -> ConversaResult<crate::types::Upload> {
		let address = format!("{}/uploads", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Cancels the Upload. No Parts may be added after an Upload is cancelled. */
	pub async fn cancel_upload(&self, upload_id: &str, ) -> ConversaResult<crate::types::Upload> {
		let address = format!("{}/uploads/{upload_id}/cancel", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Completes the [Upload](/docs/api-reference/uploads/object). 

Within the returned Upload object, there is a nested [File](/docs/api-reference/files/object) object that is ready to use in the rest of the platform.

You can specify the order of the Parts by passing in an ordered list of the Part IDs.

The number of bytes uploaded upon completion must match the number of bytes initially specified when creating the Upload object. No Parts may be added after an Upload is completed. */
	pub async fn complete_upload(&self, upload_id: &str, request_body: crate::types::CompleteUploadRequest, ) -> ConversaResult<crate::types::Upload> {
		let address = format!("{}/uploads/{upload_id}/complete", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Adds a [Part](/docs/api-reference/uploads/part-object) to an [Upload](/docs/api-reference/uploads/object) object. A Part represents a chunk of bytes from the file you are trying to upload. 

Each Part can be at most 64 MB, and you can add Parts until you hit the Upload maximum of 8 GB.

It is possible to add multiple Parts in parallel. You can decide the intended order of the Parts when you [complete the Upload](/docs/api-reference/uploads/complete). */
	pub async fn add_upload_part(&self, upload_id: &str, request_body: crate::types::AddUploadPartRequest, ) -> ConversaResult<crate::types::UploadPart> {
		let address = format!("{}/uploads/{upload_id}/parts", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.multipart(request_body.into_multipart_form());
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of vector stores. */
	pub async fn list_vector_stores(&self, limit: Option<u64>, order: Option<&str>, after: Option<&str>, before: Option<&str>, ) -> ConversaResult<crate::types::ListVectorStoresResponse> {
		let address = format!("{}/vector_stores", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create a vector store. */
	pub async fn create_vector_store(&self, request_body: crate::types::CreateVectorStoreRequest, ) -> ConversaResult<crate::types::VectorStoreObject> {
		let address = format!("{}/vector_stores", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a vector store. */
	pub async fn get_vector_store(&self, vector_store_id: &str, ) -> ConversaResult<crate::types::VectorStoreObject> {
		let address = format!("{}/vector_stores/{vector_store_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Modifies a vector store. */
	pub async fn modify_vector_store(&self, vector_store_id: &str, request_body: crate::types::UpdateVectorStoreRequest, ) -> ConversaResult<crate::types::VectorStoreObject> {
		let address = format!("{}/vector_stores/{vector_store_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete a vector store. */
	pub async fn delete_vector_store(&self, vector_store_id: &str, ) -> ConversaResult<crate::types::DeleteVectorStoreResponse> {
		let address = format!("{}/vector_stores/{vector_store_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create a vector store file batch. */
	pub async fn create_vector_store_file_batch(&self, vector_store_id: &str, request_body: crate::types::CreateVectorStoreFileBatchRequest, ) -> ConversaResult<crate::types::VectorStoreFileBatchObject> {
		let address = format!("{}/vector_stores/{vector_store_id}/file_batches", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a vector store file batch. */
	pub async fn get_vector_store_file_batch(&self, vector_store_id: &str, batch_id: &str, ) -> ConversaResult<crate::types::VectorStoreFileBatchObject> {
		let address = format!("{}/vector_stores/{vector_store_id}/file_batches/{batch_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Cancel a vector store file batch. This attempts to cancel the processing of files in this batch as soon as possible. */
	pub async fn cancel_vector_store_file_batch(&self, vector_store_id: &str, batch_id: &str, ) -> ConversaResult<crate::types::VectorStoreFileBatchObject> {
		let address = format!("{}/vector_stores/{vector_store_id}/file_batches/{batch_id}/cancel", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of vector store files in a batch. */
	pub async fn list_files_in_vector_store_batch(&self, vector_store_id: &str, batch_id: &str, limit: Option<u64>, order: Option<&str>, after: Option<&str>, before: Option<&str>, filter: Option<&str>, ) -> ConversaResult<crate::types::ListVectorStoreFilesResponse> {
		let address = format!("{}/vector_stores/{vector_store_id}/file_batches/{batch_id}/files", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		if let Some(q) = filter {
			request = request.query(&[("filter", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Returns a list of vector store files. */
	pub async fn list_vector_store_files(&self, vector_store_id: &str, limit: Option<u64>, order: Option<&str>, after: Option<&str>, before: Option<&str>, filter: Option<&str>, ) -> ConversaResult<crate::types::ListVectorStoreFilesResponse> {
		let address = format!("{}/vector_stores/{vector_store_id}/files", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		if let Some(q) = limit {
			request = request.query(&[("limit", q)]);
		}
		if let Some(q) = order {
			request = request.query(&[("order", q)]);
		}
		if let Some(q) = after {
			request = request.query(&[("after", q)]);
		}
		if let Some(q) = before {
			request = request.query(&[("before", q)]);
		}
		if let Some(q) = filter {
			request = request.query(&[("filter", q)]);
		}
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Create a vector store file by attaching a [File](/docs/api-reference/files) to a [vector store](/docs/api-reference/vector-stores/object). */
	pub async fn create_vector_store_file(&self, vector_store_id: &str, request_body: crate::types::CreateVectorStoreFileRequest, ) -> ConversaResult<crate::types::VectorStoreFileObject> {
		let address = format!("{}/vector_stores/{vector_store_id}/files", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieves a vector store file. */
	pub async fn get_vector_store_file(&self, vector_store_id: &str, file_id: &str, ) -> ConversaResult<crate::types::VectorStoreFileObject> {
		let address = format!("{}/vector_stores/{vector_store_id}/files/{file_id}", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Delete a vector store file. This will remove the file from the vector store but the file itself will not be deleted. To delete the file, use the [delete file](/docs/api-reference/files/delete) endpoint. */
	pub async fn delete_vector_store_file(&self, vector_store_id: &str, file_id: &str, ) -> ConversaResult<crate::types::DeleteVectorStoreFileResponse> {
		let address = format!("{}/vector_stores/{vector_store_id}/files/{file_id}", self.base_address);
		let mut request = self.client.delete(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Update attributes on a vector store file. */
	pub async fn update_vector_store_file_attributes(&self, vector_store_id: &str, file_id: &str, request_body: crate::types::UpdateVectorStoreFileAttributesRequest, ) -> ConversaResult<crate::types::VectorStoreFileObject> {
		let address = format!("{}/vector_stores/{vector_store_id}/files/{file_id}", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Retrieve the parsed contents of a vector store file. */
	pub async fn retrieve_vector_store_file_content(&self, vector_store_id: &str, file_id: &str, ) -> ConversaResult<crate::types::VectorStoreFileContentResponse> {
		let address = format!("{}/vector_stores/{vector_store_id}/files/{file_id}/content", self.base_address);
		let mut request = self.client.get(&address);
		request = request.bearer_auth(&self.api_key);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}

	/** Search a vector store for relevant chunks based on a query and file attributes filter. */
	pub async fn search_vector_store(&self, vector_store_id: &str, request_body: crate::types::VectorStoreSearchRequest, ) -> ConversaResult<crate::types::VectorStoreSearchResultsPage> {
		let address = format!("{}/vector_stores/{vector_store_id}/search", self.base_address);
		let mut request = self.client.post(&address);
		request = request.bearer_auth(&self.api_key);
		request = request.body(serde_json::to_string(&request_body)?);
		let result = request.send().await?;
		let status_code = result.status().as_u16();
		let _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();
		let response_bytes = result.bytes().await?;
		if status_code == 400 {
			return Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))
		}
		if status_code == 404 {
			return Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))
		}
		if status_code != 200 {
			return Err(ConversaError::UnexpectedStatusCode{code: status_code, response: String::from_utf8(response_bytes.to_vec())?})
		}
		Ok(serde_json::from_slice(&response_bytes)?)
	}


}
