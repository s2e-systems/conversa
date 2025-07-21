use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddUploadPartRequest {
	/** The chunk of bytes for this Part. */
	pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminApiKeyOwner {
	#[serde(rename="type")]
	/** Always `user` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	/** The object type, which is always organization.user */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
	/** The identifier, which can be referenced in API endpoints */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The name of the user */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The Unix timestamp (in seconds) of when the user was created */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_at: Option<u64>,
	/** Always `owner` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
}

/** Represents an individual Admin API key in an org. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminApiKey {
	/** The object type, which is always `organization.admin_api_key` */
	pub object: String,
	/** The identifier, which can be referenced in API endpoints */
	pub id: String,
	/** The name of the API key */
	pub name: String,
	/** The redacted value of the API key */
	pub redacted_value: String,
	/** The value of the API key. Only shown on create. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<String>,
	/** The Unix timestamp (in seconds) of when the API key was created */
	pub created_at: u64,
	/** The Unix timestamp (in seconds) of when the API key was last used */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_used_at: Option<u64>,
	pub owner: AdminApiKeyOwner,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyList {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Vec<AdminApiKey>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_more: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AssistantObjectObject {
	#[serde(rename="assistant")]
	Assistant,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantObjectTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantObjectToolResourcesCodeInterpreter {
	/** A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter`` tool. There can be a maximum of 20 files associated with the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantObjectToolResourcesFileSearch {
	/** The ID of the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
}

/** A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantObjectToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<AssistantObjectToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<AssistantObjectToolResourcesFileSearch>,
}

/** Represents an `assistant` that can call the model and use tools. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantObject {
	/** The identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `assistant`. */
	pub object: AssistantObjectObject,
	/** The Unix timestamp (in seconds) for when the assistant was created. */
	pub created_at: u64,
	/** The name of the assistant. The maximum length is 256 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The description of the assistant. The maximum length is 512 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
	pub model: String,
	/** The system instructions that the assistant uses. The maximum length is 256,000 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`. */
	pub tools: Vec<AssistantObjectTools>,
	/** A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<AssistantObjectToolResources>,
	pub metadata: Metadata,
	/** What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or temperature but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
}

/** Represents an event emitted when streaming a Run.

Each event in a server-sent events stream has an `event` and `data` property:

***
event: thread.created
data: {"id": "thread_123", "object": "thread", ...}
***

We emit events whenever a new object is created, transitions to a new state, or is being
streamed in parts (deltas). For example, we emit `thread.run.created` when a new run
is created, `thread.run.completed` when a run completes, and so on. When an Assistant chooses
to create a message during a run, we emit a `thread.message.created event`, a
`thread.message.in_progress` event, many `thread.message.delta` events, and finally a
`thread.message.completed` event.

We may add additional events over time, so we recommend handling unknown events gracefully
in your code. See the [Assistants API quickstart](/docs/assistants/overview) to learn how to
integrate the Assistants API with streaming. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantStreamEvent {
	ThreadStreamEvent(ThreadStreamEvent),
	RunStreamEvent(RunStreamEvent),
	RunStepStreamEvent(RunStepStreamEvent),
	MessageStreamEvent(MessageStreamEvent),
	ErrorEvent(ErrorEvent),
	DoneEvent(DoneEvent),
}

pub type AssistantSupportedModels = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AssistantToolsCodeType {
	#[serde(rename="code_interpreter")]
	CodeInterpreter,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantToolsCode {
	#[serde(rename="type")]
	/** The type of tool being defined: `code_interpreter` */
	pub r#type: AssistantToolsCodeType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AssistantToolsFileSearchType {
	#[serde(rename="file_search")]
	FileSearch,
}

/** Overrides for the file search tool. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantToolsFileSearchFileSearch {
	/** The maximum number of results the file search tool should output. The default is 20 for `gpt-4*` models and 5 for `gpt-3.5-turbo`. This number should be between 1 and 50 inclusive.

Note that the file search tool may output fewer than `max_num_results` results. See the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_num_results: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranking_options: Option<FileSearchRankingOptions>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantToolsFileSearch {
	#[serde(rename="type")]
	/** The type of tool being defined: `file_search` */
	pub r#type: AssistantToolsFileSearchType,
	/** Overrides for the file search tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<AssistantToolsFileSearchFileSearch>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AssistantToolsFileSearchTypeOnlyType {
	#[serde(rename="file_search")]
	FileSearch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantToolsFileSearchTypeOnly {
	#[serde(rename="type")]
	/** The type of tool being defined: `file_search` */
	pub r#type: AssistantToolsFileSearchTypeOnlyType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AssistantToolsFunctionType {
	#[serde(rename="function")]
	Function,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantToolsFunction {
	#[serde(rename="type")]
	/** The type of tool being defined: `function` */
	pub r#type: AssistantToolsFunctionType,
	pub function: FunctionObject,
}

/** Specifies the format that the model must output. Compatible with [GPT-4o](/docs/models#gpt-4o), [GPT-4 Turbo](/docs/models#gpt-4-turbo-and-gpt-4), and all GPT-3.5 Turbo models since `gpt-3.5-turbo-1106`.

Setting to `{ "type": "json_schema", "json_schema": {...} }` enables Structured Outputs which ensures the model will match your supplied JSON schema. Learn more in the [Structured Outputs guide](/docs/guides/structured-outputs).

Setting to `{ "type": "json_object" }` enables JSON mode, which ensures the message the model generates is valid JSON.

**Important:** when using JSON mode, you **must** also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly "stuck" request. Also note that the message content may be partially cut off if `finish_reason="length"`, which indicates the generation exceeded `max_tokens` or the conversation exceeded the max context length. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiResponseFormatOption {
	/** `auto` is the default value */
	#[serde(rename="auto")]
	Auto,
	ResponseFormatText(ResponseFormatText),
	ResponseFormatJsonObject(ResponseFormatJsonObject),
	ResponseFormatJsonSchema(ResponseFormatJsonSchema),
}

/** Controls which (if any) tool is called by the model.
`none` means the model will not call any tools and instead generates a message.
`auto` is the default value and means the model can pick between generating a message or calling one or more tools.
`required` means the model must call one or more tools before responding to the user.
Specifying a particular tool like `{"type": "file_search"}` or `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantsApiToolChoiceOption {
	/** `none` means the model will not call any tools and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools before responding to the user. */
	#[serde(rename="none")]
	None,
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="required")]
	Required,
	AssistantsNamedToolChoice(AssistantsNamedToolChoice),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AssistantsNamedToolChoiceType {
	#[serde(rename="function")]
	Function,
	#[serde(rename="code_interpreter")]
	CodeInterpreter,
	#[serde(rename="file_search")]
	FileSearch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantsNamedToolChoiceFunction {
	/** The name of the function to call. */
	pub name: String,
}

/** Specifies a tool the model should use. Use to force the model to call a specific tool. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AssistantsNamedToolChoice {
	#[serde(rename="type")]
	/** The type of the tool. If type is `function`, the function name must be set */
	pub r#type: AssistantsNamedToolChoiceType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function: Option<AssistantsNamedToolChoiceFunction>,
}

/** The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`. For `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`, the only supported format is `json`. */
pub type AudioResponseFormat = String;

/** The project that the action was scoped to. Absent for actions not scoped to projects. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogProject {
	/** The project ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The project title. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

/** The payload used to create the API key. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogApiKeyCreatedData {
	/** A list of scopes allowed for the API key, e.g. `["api.model.request"]` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scopes: Option<Vec<String>>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogApiKeyCreated {
	/** The tracking ID of the API key. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to create the API key. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<AuditLogApiKeyCreatedData>,
}

/** The payload used to update the API key. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogApiKeyUpdatedChangesRequested {
	/** A list of scopes allowed for the API key, e.g. `["api.model.request"]` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scopes: Option<Vec<String>>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogApiKeyUpdated {
	/** The tracking ID of the API key. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to update the API key. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changes_requested: Option<AuditLogApiKeyUpdatedChangesRequested>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogApiKeyDeleted {
	/** The tracking ID of the API key. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The payload used to create the checkpoint permission. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCheckpointPermissionCreatedData {
	/** The ID of the project that the checkpoint permission was created for. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
	/** The ID of the fine-tuned model checkpoint. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fine_tuned_model_checkpoint: Option<String>,
}

/** The project and fine-tuned model checkpoint that the checkpoint permission was created for. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCheckpointPermissionCreated {
	/** The ID of the checkpoint permission. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to create the checkpoint permission. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<AuditLogCheckpointPermissionCreatedData>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCheckpointPermissionDeleted {
	/** The ID of the checkpoint permission. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The payload used to create the invite. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogInviteSentData {
	/** The email invited to the organization. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
	/** The role the email was invited to be. Is either `owner` or `member`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogInviteSent {
	/** The ID of the invite. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to create the invite. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<AuditLogInviteSentData>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogInviteAccepted {
	/** The ID of the invite. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogInviteDeleted {
	/** The ID of the invite. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogLoginFailed {
	/** The error code of the failure. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error_code: Option<String>,
	/** The error message of the failure. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error_message: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogLogoutFailed {
	/** The error code of the failure. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error_code: Option<String>,
	/** The error message of the failure. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error_message: Option<String>,
}

/** The payload used to update the organization settings. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogOrganizationUpdatedChangesRequested {
	/** The organization title. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
	/** The organization description. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** The organization name. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** Visibility of the threads page which shows messages created with the Assistants API and Playground. One of `ANY_ROLE`, `OWNERS`, or `NONE`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threads_ui_visibility: Option<String>,
	/** Visibility of the usage dashboard which shows activity and costs for your organization. One of `ANY_ROLE` or `OWNERS`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage_dashboard_visibility: Option<String>,
	/** How your organization logs data from supported API calls. One of `disabled`, `enabled_per_call`, `enabled_for_all_projects`, or `enabled_for_selected_projects` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_call_logging: Option<String>,
	/** The list of project ids if api_call_logging is set to `enabled_for_selected_projects` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_call_logging_project_ids: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogOrganizationUpdated {
	/** The organization ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to update the organization settings. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changes_requested: Option<AuditLogOrganizationUpdatedChangesRequested>,
}

/** The payload used to create the project. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogProjectCreatedData {
	/** The project name. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The title of the project as seen on the dashboard. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogProjectCreated {
	/** The project ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to create the project. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<AuditLogProjectCreatedData>,
}

/** The payload used to update the project. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogProjectUpdatedChangesRequested {
	/** The title of the project as seen on the dashboard. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub title: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogProjectUpdated {
	/** The project ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to update the project. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changes_requested: Option<AuditLogProjectUpdatedChangesRequested>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogProjectArchived {
	/** The project ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The payload used to update the rate limits. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogRateLimitUpdatedChangesRequested {
	/** The maximum requests per minute. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_minute: Option<u64>,
	/** The maximum tokens per minute. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tokens_per_1_minute: Option<u64>,
	/** The maximum images per minute. Only relevant for certain models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_images_per_1_minute: Option<u64>,
	/** The maximum audio megabytes per minute. Only relevant for certain models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_audio_megabytes_per_1_minute: Option<u64>,
	/** The maximum requests per day. Only relevant for certain models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_day: Option<u64>,
	/** The maximum batch input tokens per day. Only relevant for certain models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_1_day_max_input_tokens: Option<u64>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogRateLimitUpdated {
	/** The rate limit ID */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to update the rate limits. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changes_requested: Option<AuditLogRateLimitUpdatedChangesRequested>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogRateLimitDeleted {
	/** The rate limit ID */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The payload used to create the service account. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogServiceAccountCreatedData {
	/** The role of the service account. Is either `owner` or `member`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogServiceAccountCreated {
	/** The service account ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to create the service account. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<AuditLogServiceAccountCreatedData>,
}

/** The payload used to updated the service account. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogServiceAccountUpdatedChangesRequested {
	/** The role of the service account. Is either `owner` or `member`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogServiceAccountUpdated {
	/** The service account ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to updated the service account. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changes_requested: Option<AuditLogServiceAccountUpdatedChangesRequested>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogServiceAccountDeleted {
	/** The service account ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The payload used to add the user to the project. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogUserAddedData {
	/** The role of the user. Is either `owner` or `member`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogUserAdded {
	/** The user ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to add the user to the project. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<AuditLogUserAddedData>,
}

/** The payload used to update the user. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogUserUpdatedChangesRequested {
	/** The role of the user. Is either `owner` or `member`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogUserUpdated {
	/** The project ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The payload used to update the user. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub changes_requested: Option<AuditLogUserUpdatedChangesRequested>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogUserDeleted {
	/** The user ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCertificateCreated {
	/** The certificate ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The name of the certificate. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCertificateUpdated {
	/** The certificate ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The name of the certificate. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCertificateDeleted {
	/** The certificate ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The name of the certificate. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The certificate content in PEM format. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub certificate: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCertificatesActivatedCertificates {
	/** The certificate ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The name of the certificate. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCertificatesActivated {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub certificates: Option<Vec<AuditLogCertificatesActivatedCertificates>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCertificatesDeactivatedCertificates {
	/** The certificate ID. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The name of the certificate. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

/** The details for events with this `type`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogCertificatesDeactivated {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub certificates: Option<Vec<AuditLogCertificatesDeactivatedCertificates>>,
}

/** A log of a user action or configuration change within this organization. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLog {
	/** The ID of this log. */
	pub id: String,
	#[serde(rename="type")]
	pub r#type: AuditLogEventType,
	/** The Unix timestamp (in seconds) of the event. */
	pub effective_at: u64,
	/** The project that the action was scoped to. Absent for actions not scoped to projects. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project: Option<AuditLogProject>,
	pub actor: AuditLogActor,
	#[serde(rename="api_key.created")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_created: Option<AuditLogApiKeyCreated>,
	#[serde(rename="api_key.updated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_updated: Option<AuditLogApiKeyUpdated>,
	#[serde(rename="api_key.deleted")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_deleted: Option<AuditLogApiKeyDeleted>,
	#[serde(rename="checkpoint_permission.created")]
	/** The project and fine-tuned model checkpoint that the checkpoint permission was created for. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub checkpoint_permission_created: Option<AuditLogCheckpointPermissionCreated>,
	#[serde(rename="checkpoint_permission.deleted")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub checkpoint_permission_deleted: Option<AuditLogCheckpointPermissionDeleted>,
	#[serde(rename="invite.sent")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invite_sent: Option<AuditLogInviteSent>,
	#[serde(rename="invite.accepted")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invite_accepted: Option<AuditLogInviteAccepted>,
	#[serde(rename="invite.deleted")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub invite_deleted: Option<AuditLogInviteDeleted>,
	#[serde(rename="login.failed")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub login_failed: Option<AuditLogLoginFailed>,
	#[serde(rename="logout.failed")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logout_failed: Option<AuditLogLogoutFailed>,
	#[serde(rename="organization.updated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub organization_updated: Option<AuditLogOrganizationUpdated>,
	#[serde(rename="project.created")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_created: Option<AuditLogProjectCreated>,
	#[serde(rename="project.updated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_updated: Option<AuditLogProjectUpdated>,
	#[serde(rename="project.archived")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_archived: Option<AuditLogProjectArchived>,
	#[serde(rename="rate_limit.updated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_updated: Option<AuditLogRateLimitUpdated>,
	#[serde(rename="rate_limit.deleted")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rate_limit_deleted: Option<AuditLogRateLimitDeleted>,
	#[serde(rename="service_account.created")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_account_created: Option<AuditLogServiceAccountCreated>,
	#[serde(rename="service_account.updated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_account_updated: Option<AuditLogServiceAccountUpdated>,
	#[serde(rename="service_account.deleted")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_account_deleted: Option<AuditLogServiceAccountDeleted>,
	#[serde(rename="user.added")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_added: Option<AuditLogUserAdded>,
	#[serde(rename="user.updated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_updated: Option<AuditLogUserUpdated>,
	#[serde(rename="user.deleted")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_deleted: Option<AuditLogUserDeleted>,
	#[serde(rename="certificate.created")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub certificate_created: Option<AuditLogCertificateCreated>,
	#[serde(rename="certificate.updated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub certificate_updated: Option<AuditLogCertificateUpdated>,
	#[serde(rename="certificate.deleted")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub certificate_deleted: Option<AuditLogCertificateDeleted>,
	#[serde(rename="certificates.activated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub certificates_activated: Option<AuditLogCertificatesActivated>,
	#[serde(rename="certificates.deactivated")]
	/** The details for events with this `type`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub certificates_deactivated: Option<AuditLogCertificatesDeactivated>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AuditLogActorType {
	#[serde(rename="session")]
	Session,
	#[serde(rename="api_key")]
	ApiKey,
}

/** The actor who performed the audit logged action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogActor {
	#[serde(rename="type")]
	/** The type of actor. Is either `session` or `api_key`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<AuditLogActorType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session: Option<AuditLogActorSession>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key: Option<AuditLogActorApiKey>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AuditLogActorApiKeyType {
	#[serde(rename="user")]
	User,
	#[serde(rename="service_account")]
	ServiceAccount,
}

/** The API Key used to perform the audit logged action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogActorApiKey {
	/** The tracking id of the API key. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of API key. Can be either `user` or `service_account`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<AuditLogActorApiKeyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<AuditLogActorUser>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_account: Option<AuditLogActorServiceAccount>,
}

/** The service account that performed the audit logged action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogActorServiceAccount {
	/** The service account id. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}

/** The session in which the audit logged action was performed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogActorSession {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<AuditLogActorUser>,
	/** The IP address from which the action was performed. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ip_address: Option<String>,
}

/** The user who performed the audit logged action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AuditLogActorUser {
	/** The user id. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The user email. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub email: Option<String>,
}

/** The event type. */
pub type AuditLogEventType = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AutoChunkingStrategyRequestParamType {
	#[serde(rename="auto")]
	Auto,
}

/** The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoChunkingStrategyRequestParam {
	#[serde(rename="type")]
	/** Always `auto`. */
	pub r#type: AutoChunkingStrategyRequestParamType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BatchObject {
	#[serde(rename="batch")]
	Batch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchErrorsData {
	/** An error code identifying the error type. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	/** A human-readable message providing more details about the error. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<String>,
	/** The name of the parameter that caused the error, if applicable. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub param: Option<String>,
	/** The line number of the input file where the error occurred, if applicable. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub line: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchErrors {
	/** The object type, which is always `list`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Vec<BatchErrorsData>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BatchStatus {
	#[serde(rename="validating")]
	Validating,
	#[serde(rename="failed")]
	Failed,
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="finalizing")]
	Finalizing,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="expired")]
	Expired,
	#[serde(rename="cancelling")]
	Cancelling,
	#[serde(rename="cancelled")]
	Cancelled,
}

/** The request counts for different statuses within the batch. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRequestCounts {
	/** Total number of requests in the batch. */
	pub total: u64,
	/** Number of requests that have been completed successfully. */
	pub completed: u64,
	/** Number of requests that have failed. */
	pub failed: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Batch {
	pub id: String,
	/** The object type, which is always `batch`. */
	pub object: BatchObject,
	/** The OpenAI API endpoint used by the batch. */
	pub endpoint: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub errors: Option<BatchErrors>,
	/** The ID of the input file for the batch. */
	pub input_file_id: String,
	/** The time frame within which the batch should be processed. */
	pub completion_window: String,
	/** The current status of the batch. */
	pub status: BatchStatus,
	/** The ID of the file containing the outputs of successfully executed requests. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_file_id: Option<String>,
	/** The ID of the file containing the outputs of requests with errors. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error_file_id: Option<String>,
	/** The Unix timestamp (in seconds) for when the batch was created. */
	pub created_at: u64,
	/** The Unix timestamp (in seconds) for when the batch started processing. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub in_progress_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the batch will expire. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the batch started finalizing. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub finalizing_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the batch was completed. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completed_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the batch failed. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub failed_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the batch expired. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expired_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the batch started cancelling. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cancelling_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the batch was cancelled. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cancelled_at: Option<u64>,
	/** The request counts for different statuses within the batch. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub request_counts: Option<BatchRequestCounts>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum BatchRequestInputMethod {
	#[serde(rename="POST")]
	POST,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchRequestInputBody {
	CreateResponse(CreateResponse),
}

/** The per-line object of the batch input file */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRequestInput {
	/** A developer-provided per-request id that will be used to match outputs to inputs. Must be unique for each request in a batch. */
	pub custom_id: String,
	/** The HTTP method to be used for the request. Currently only `POST` is supported. */
	pub method: BatchRequestInputMethod,
	/** The OpenAI API relative URL to be used for the request. Currently `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported. */
	pub url: String,
	pub body: BatchRequestInputBody,
}

	/** The JSON body of the response */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRequestOutputResponseBody(pub HashMap<String,String>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRequestOutputResponse {
	/** The HTTP status code of the response */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status_code: Option<u64>,
	/** An unique identifier for the OpenAI API request. Please include this request ID when contacting support. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub request_id: Option<String>,
	/** The JSON body of the response */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub body: Option<BatchRequestOutputResponseBody>,
}

/** For requests that failed with a non-HTTP error, this will contain more information on the cause of the failure. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRequestOutputError {
	/** A machine-readable error code. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	/** A human-readable error message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<String>,
}

/** The per-line object of the batch output and error files */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BatchRequestOutput {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** A developer-provided per-request id that will be used to match outputs to inputs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub custom_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response: Option<BatchRequestOutputResponse>,
	/** For requests that failed with a non-HTTP error, this will contain more information on the cause of the failure. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<BatchRequestOutputError>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CertificateObject {
	#[serde(rename="certificate")]
	Certificate,
	#[serde(rename="organization.certificate")]
	OrganizationCertificate,
	#[serde(rename="organization.project.certificate")]
	OrganizationProjectCertificate,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CertificateCertificateDetails {
	/** The Unix timestamp (in seconds) of when the certificate becomes valid. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_at: Option<u64>,
	/** The Unix timestamp (in seconds) of when the certificate expires. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<u64>,
	/** The content of the certificate in PEM format. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<String>,
}

/** Represents an individual `certificate` uploaded to the organization. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
	/** The object type.

- If creating, updating, or getting a specific certificate, the object type is `certificate`.
- If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`.
- If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`. */
	pub object: CertificateObject,
	/** The identifier, which can be referenced in API endpoints */
	pub id: String,
	/** The name of the certificate. */
	pub name: String,
	/** The Unix timestamp (in seconds) of when the certificate was uploaded. */
	pub created_at: u64,
	pub certificate_details: CertificateCertificateDetails,
	/** Whether the certificate is currently active at the specified scope. Not returned when getting details for a specific certificate. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub active: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionDeletedObject {
	#[serde(rename="chat.completion.deleted")]
	ChatCompletionDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionDeleted {
	/** The type of object being deleted. */
	pub object: ChatCompletionDeletedObject,
	/** The ID of the chat completion that was deleted. */
	pub id: String,
	/** Whether the chat completion was deleted. */
	pub deleted: bool,
}

/** Specifying a particular function via `{"name": "my_function"}` forces the model to call that function. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionFunctionCallOption {
	/** The name of the function to call. */
	pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionFunctions {
	/** A description of what the function does, used by the model to choose when and how to call the function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64. */
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<FunctionParameters>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionListObject {
	#[serde(rename="list")]
	List,
}

/** An object representing a list of Chat Completions. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionList {
	/** The type of this object. It is always set to "list". */
	pub object: ChatCompletionListObject,
	/** An array of chat completion objects. */
	pub data: Vec<CreateChatCompletionResponse>,
	/** The identifier of the first chat completion in the data array. */
	pub first_id: String,
	/** The identifier of the last chat completion in the data array. */
	pub last_id: String,
	/** Indicates whether there are more Chat Completions available. */
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionMessageListObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionMessageListDataObject {
	/** The identifier of the chat message. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionMessageListData {
	#[serde(flatten)]
	pub chat_completion_response_message: ChatCompletionResponseMessage,
	#[serde(flatten)]
	pub object: ChatCompletionMessageListDataObject,
}

/** An object representing a list of chat completion messages. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionMessageList {
	/** The type of this object. It is always set to "list". */
	pub object: ChatCompletionMessageListObject,
	/** An array of chat completion message objects. */
	pub data: Vec<ChatCompletionMessageListData>,
	/** The identifier of the first chat message in the data array. */
	pub first_id: String,
	/** The identifier of the last chat message in the data array. */
	pub last_id: String,
	/** Indicates whether there are more chat messages available. */
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionMessageToolCallType {
	#[serde(rename="function")]
	Function,
}

/** The function that the model called. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionMessageToolCallFunction {
	/** The name of the function to call. */
	pub name: String,
	/** The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function. */
	pub arguments: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionMessageToolCall {
	/** The ID of the tool call. */
	pub id: String,
	#[serde(rename="type")]
	/** The type of the tool. Currently, only `function` is supported. */
	pub r#type: ChatCompletionMessageToolCallType,
	/** The function that the model called. */
	pub function: ChatCompletionMessageToolCallFunction,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionMessageToolCallChunkType {
	#[serde(rename="function")]
	Function,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionMessageToolCallChunkFunction {
	/** The name of the function to call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionMessageToolCallChunk {
	pub index: u64,
	/** The ID of the tool call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of the tool. Currently, only `function` is supported. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<ChatCompletionMessageToolCallChunkType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function: Option<ChatCompletionMessageToolCallChunkFunction>,
}

/** The tool calls generated by the model, such as function calls. */
pub type ChatCompletionMessageToolCalls = Vec<ChatCompletionMessageToolCall>;

/** Output types that you would like the model to generate for this request.
Most models are capable of generating text, which is the default:

`["text"]`

The `gpt-4o-audio-preview` model can also be used to [generate audio](/docs/guides/audio). To
request that this model generate both text and audio responses, you can
use:

`["text", "audio"]` */
pub type ChatCompletionModalities = Vec<String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionNamedToolChoiceType {
	#[serde(rename="function")]
	Function,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionNamedToolChoiceFunction {
	/** The name of the function to call. */
	pub name: String,
}

/** Specifies a tool the model should use. Use to force the model to call a specific function. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionNamedToolChoice {
	#[serde(rename="type")]
	/** The type of the tool. Currently, only `function` is supported. */
	pub r#type: ChatCompletionNamedToolChoiceType,
	pub function: ChatCompletionNamedToolChoiceFunction,
}

/** The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContent {
	/** The contents of the assistant message. */
	String(String),
	/** An array of content parts with a defined type. Can be one or more of type `text`, or exactly one of type `refusal`. */
	ArrayList(Vec<ChatCompletionRequestAssistantMessageContentPart>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestAssistantMessageRole {
	#[serde(rename="assistant")]
	Assistant,
}

/** Data about a previous audio response from the model. 
[Learn more](/docs/guides/audio). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestAssistantMessageAudio {
	/** Unique identifier for a previous audio response from the model. */
	pub id: String,
}

/** Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestAssistantMessageFunctionCall {
	/** The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function. */
	pub arguments: String,
	/** The name of the function to call. */
	pub name: String,
}

/** Messages sent by the model in response to user messages. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestAssistantMessage {
	/** The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<ChatCompletionRequestAssistantMessageContent>,
	/** The refusal message by the assistant. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<String>,
	/** The role of the messages author, in this case `assistant`. */
	pub role: ChatCompletionRequestAssistantMessageRole,
	/** An optional name for the participant. Provides the model information to differentiate between participants of the same role. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** Data about a previous audio response from the model. 
[Learn more](/docs/guides/audio). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<ChatCompletionRequestAssistantMessageAudio>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<ChatCompletionMessageToolCalls>,
	/** Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<ChatCompletionRequestAssistantMessageFunctionCall>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestAssistantMessageContentPart {
	ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
	ChatCompletionRequestMessageContentPartRefusal(ChatCompletionRequestMessageContentPartRefusal),
}

/** The contents of the developer message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
	/** The contents of the developer message. */
	String(String),
	/** An array of content parts with a defined type. For developer messages, only type `text` is supported. */
	ArrayList(Vec<ChatCompletionRequestMessageContentPartText>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestDeveloperMessageRole {
	#[serde(rename="developer")]
	Developer,
}

/** Developer-provided instructions that the model should follow, regardless of
messages sent by the user. With o1 models and newer, `developer` messages
replace the previous `system` messages. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestDeveloperMessage {
	/** The contents of the developer message. */
	pub content: ChatCompletionRequestDeveloperMessageContent,
	/** The role of the messages author, in this case `developer`. */
	pub role: ChatCompletionRequestDeveloperMessageRole,
	/** An optional name for the participant. Provides the model information to differentiate between participants of the same role. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestFunctionMessageRole {
	#[serde(rename="function")]
	Function,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestFunctionMessage {
	/** The role of the messages author, in this case `function`. */
	pub role: ChatCompletionRequestFunctionMessageRole,
	/** The contents of the function message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<String>,
	/** The name of the function to call. */
	pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestMessage {
	ChatCompletionRequestDeveloperMessage(ChatCompletionRequestDeveloperMessage),
	ChatCompletionRequestSystemMessage(ChatCompletionRequestSystemMessage),
	ChatCompletionRequestUserMessage(ChatCompletionRequestUserMessage),
	ChatCompletionRequestAssistantMessage(ChatCompletionRequestAssistantMessage),
	ChatCompletionRequestToolMessage(ChatCompletionRequestToolMessage),
	ChatCompletionRequestFunctionMessage(ChatCompletionRequestFunctionMessage),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestMessageContentPartAudioType {
	#[serde(rename="input_audio")]
	InputAudio,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestMessageContentPartAudioInputAudioFormat {
	#[serde(rename="wav")]
	Wav,
	#[serde(rename="mp3")]
	Mp3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartAudioInputAudio {
	/** Base64 encoded audio data. */
	pub data: String,
	/** The format of the encoded audio data. Currently supports "wav" and "mp3". */
	pub format: ChatCompletionRequestMessageContentPartAudioInputAudioFormat,
}

/** Learn about [audio inputs](/docs/guides/audio). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartAudio {
	#[serde(rename="type")]
	/** The type of the content part. Always `input_audio`. */
	pub r#type: ChatCompletionRequestMessageContentPartAudioType,
	pub input_audio: ChatCompletionRequestMessageContentPartAudioInputAudio,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestMessageContentPartFileType {
	#[serde(rename="file")]
	File,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartFileFile {
	/** The name of the file, used when passing the file to the model as a 
string. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
	/** The base64 encoded file data, used when passing the file to the model 
as a string. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_data: Option<String>,
	/** The ID of an uploaded file to use as input. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}

/** Learn about [file inputs](/docs/guides/text) for text generation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartFile {
	#[serde(rename="type")]
	/** The type of the content part. Always `file`. */
	pub r#type: ChatCompletionRequestMessageContentPartFileType,
	pub file: ChatCompletionRequestMessageContentPartFileFile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestMessageContentPartImageType {
	#[serde(rename="image_url")]
	ImageUrl,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestMessageContentPartImageImageUrlDetail {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="low")]
	Low,
	#[serde(rename="high")]
	High,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartImageImageUrl {
	/** Either a URL of the image or the base64 encoded image data. */
	pub url: String,
	/** Specifies the detail level of the image. Learn more in the [Vision guide](/docs/guides/vision#low-or-high-fidelity-image-understanding). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<ChatCompletionRequestMessageContentPartImageImageUrlDetail>,
}

/** Learn about [image inputs](/docs/guides/vision). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartImage {
	#[serde(rename="type")]
	/** The type of the content part. */
	pub r#type: ChatCompletionRequestMessageContentPartImageType,
	pub image_url: ChatCompletionRequestMessageContentPartImageImageUrl,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestMessageContentPartRefusalType {
	#[serde(rename="refusal")]
	Refusal,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartRefusal {
	#[serde(rename="type")]
	/** The type of the content part. */
	pub r#type: ChatCompletionRequestMessageContentPartRefusalType,
	/** The refusal message generated by the model. */
	pub refusal: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestMessageContentPartTextType {
	#[serde(rename="text")]
	Text,
}

/** Learn about [text inputs](/docs/guides/text-generation). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestMessageContentPartText {
	#[serde(rename="type")]
	/** The type of the content part. */
	pub r#type: ChatCompletionRequestMessageContentPartTextType,
	/** The text content. */
	pub text: String,
}

/** The contents of the system message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContent {
	/** The contents of the system message. */
	String(String),
	/** An array of content parts with a defined type. For system messages, only type `text` is supported. */
	ArrayList(Vec<ChatCompletionRequestSystemMessageContentPart>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestSystemMessageRole {
	#[serde(rename="system")]
	System,
}

/** Developer-provided instructions that the model should follow, regardless of
messages sent by the user. With o1 models and newer, use `developer` messages
for this purpose instead. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestSystemMessage {
	/** The contents of the system message. */
	pub content: ChatCompletionRequestSystemMessageContent,
	/** The role of the messages author, in this case `system`. */
	pub role: ChatCompletionRequestSystemMessageRole,
	/** An optional name for the participant. Provides the model information to differentiate between participants of the same role. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestSystemMessageContentPart {
	ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestToolMessageRole {
	#[serde(rename="tool")]
	Tool,
}

/** The contents of the tool message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContent {
	/** The contents of the tool message. */
	String(String),
	/** An array of content parts with a defined type. For tool messages, only type `text` is supported. */
	ArrayList(Vec<ChatCompletionRequestToolMessageContentPart>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestToolMessage {
	/** The role of the messages author, in this case `tool`. */
	pub role: ChatCompletionRequestToolMessageRole,
	/** The contents of the tool message. */
	pub content: ChatCompletionRequestToolMessageContent,
	/** Tool call that this message is responding to. */
	pub tool_call_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestToolMessageContentPart {
	ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
}

/** The contents of the user message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContent {
	/** The text contents of the message. */
	String(String),
	/** An array of content parts with a defined type. Supported options differ based on the [model](/docs/models) being used to generate the response. Can contain text, image, or audio inputs. */
	ArrayList(Vec<ChatCompletionRequestUserMessageContentPart>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionRequestUserMessageRole {
	#[serde(rename="user")]
	User,
}

/** Messages sent by an end user, containing prompts or additional context
information. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionRequestUserMessage {
	/** The contents of the user message. */
	pub content: ChatCompletionRequestUserMessageContent,
	/** The role of the messages author, in this case `user`. */
	pub role: ChatCompletionRequestUserMessageRole,
	/** An optional name for the participant. Provides the model information to differentiate between participants of the same role. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionRequestUserMessageContentPart {
	ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
	ChatCompletionRequestMessageContentPartImage(ChatCompletionRequestMessageContentPartImage),
	ChatCompletionRequestMessageContentPartAudio(ChatCompletionRequestMessageContentPartAudio),
	ChatCompletionRequestMessageContentPartFile(ChatCompletionRequestMessageContentPartFile),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionResponseMessageAnnotationsType {
	#[serde(rename="url_citation")]
	UrlCitation,
}

/** A URL citation when using web search. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseMessageAnnotationsUrlCitation {
	/** The index of the last character of the URL citation in the message. */
	pub end_index: u64,
	/** The index of the first character of the URL citation in the message. */
	pub start_index: u64,
	/** The URL of the web resource. */
	pub url: String,
	/** The title of the web resource. */
	pub title: String,
}

/** A URL citation when using web search. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseMessageAnnotations {
	#[serde(rename="type")]
	/** The type of the URL citation. Always `url_citation`. */
	pub r#type: ChatCompletionResponseMessageAnnotationsType,
	/** A URL citation when using web search. */
	pub url_citation: ChatCompletionResponseMessageAnnotationsUrlCitation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionResponseMessageRole {
	#[serde(rename="assistant")]
	Assistant,
}

/** Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseMessageFunctionCall {
	/** The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function. */
	pub arguments: String,
	/** The name of the function to call. */
	pub name: String,
}

/** If the audio output modality is requested, this object contains data
about the audio response from the model. [Learn more](/docs/guides/audio). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseMessageAudio {
	/** Unique identifier for this audio response. */
	pub id: String,
	/** The Unix timestamp (in seconds) for when this audio response will
no longer be accessible on the server for use in multi-turn
conversations. */
	pub expires_at: u64,
	/** Base64 encoded audio bytes generated by the model, in the format
specified in the request. */
	pub data: String,
	/** Transcript of the audio generated by the model. */
	pub transcript: String,
}

/** A chat completion message generated by the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseMessage {
	/** The contents of the message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<String>,
	/** The refusal message generated by the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<ChatCompletionMessageToolCalls>,
	/** Annotations for the message, when applicable, as when using the
[web search tool](/docs/guides/tools-web-search?api-mode=chat). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<ChatCompletionResponseMessageAnnotations>>,
	/** The role of the author of this message. */
	pub role: ChatCompletionResponseMessageRole,
	/** Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<ChatCompletionResponseMessageFunctionCall>,
	/** If the audio output modality is requested, this object contains data
about the audio response from the model. [Learn more](/docs/guides/audio). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<ChatCompletionResponseMessageAudio>,
}

/** The role of the author of a message */
pub type ChatCompletionRole = String;

/** Options for streaming response. Only set this when you set `stream: true`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionStreamOptions {
	/** If set, an additional chunk will be streamed before the `data: [DONE]`
message. The `usage` field on this chunk shows the token usage statistics
for the entire request, and the `choices` field will always be an empty
array. 

All other chunks will also include a `usage` field, but with a null
value. **NOTE:** If the stream is interrupted, you may not receive the
final usage chunk which contains the total token usage for the request. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_usage: Option<bool>,
}

/** Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionStreamResponseDeltaFunctionCall {
	/** The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/** The name of the function to call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionStreamResponseDeltaRole {
	#[serde(rename="developer")]
	Developer,
	#[serde(rename="system")]
	System,
	#[serde(rename="user")]
	User,
	#[serde(rename="assistant")]
	Assistant,
	#[serde(rename="tool")]
	Tool,
}

/** A chat completion delta generated by streamed model responses. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionStreamResponseDelta {
	/** The contents of the chunk message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<String>,
	/** Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<ChatCompletionStreamResponseDeltaFunctionCall>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<Vec<ChatCompletionMessageToolCallChunk>>,
	/** The role of the author of this message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<ChatCompletionStreamResponseDeltaRole>,
	/** The refusal message generated by the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionTokenLogprobTopLogprobs {
	/** The token. */
	pub token: String,
	/** The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely. */
	pub logprob: f32,
	/** A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bytes: Option<Vec<u64>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionTokenLogprob {
	/** The token. */
	pub token: String,
	/** The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely. */
	pub logprob: f32,
	/** A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bytes: Option<Vec<u64>>,
	/** List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned. */
	pub top_logprobs: Vec<ChatCompletionTokenLogprobTopLogprobs>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ChatCompletionToolType {
	#[serde(rename="function")]
	Function,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionTool {
	#[serde(rename="type")]
	/** The type of the tool. Currently, only `function` is supported. */
	pub r#type: ChatCompletionToolType,
	pub function: FunctionObject,
}

/** Controls which (if any) tool is called by the model.
`none` means the model will not call any tool and instead generates a message.
`auto` means the model can pick between generating a message or calling one or more tools.
`required` means the model must call one or more tools.
Specifying a particular tool via `{"type": "function", "function": {"name": "my_function"}}` forces the model to call that tool.

`none` is the default when no tools are present. `auto` is the default if tools are present. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
	/** `none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools. */
	#[serde(rename="none")]
	None,
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="required")]
	Required,
	ChatCompletionNamedToolChoice(ChatCompletionNamedToolChoice),
}

/** The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChunkingStrategyRequestParam {
	AutoChunkingStrategyRequestParam(AutoChunkingStrategyRequestParam),
	StaticChunkingStrategyRequestParam(StaticChunkingStrategyRequestParam),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ClickType {
	#[serde(rename="click")]
	Click,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ClickButton {
	#[serde(rename="left")]
	Left,
	#[serde(rename="right")]
	Right,
	#[serde(rename="wheel")]
	Wheel,
	#[serde(rename="back")]
	Back,
	#[serde(rename="forward")]
	Forward,
}

/** A click action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Click {
	#[serde(rename="type")]
	/** Specifies the event type. For a click action, this property is 
always set to `click`. */
	pub r#type: ClickType,
	/** Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`. */
	pub button: ClickButton,
	/** The x-coordinate where the click occurred. */
	pub x: u64,
	/** The y-coordinate where the click occurred. */
	pub y: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeInterpreterFileOutputType {
	#[serde(rename="files")]
	Files,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterFileOutputFiles {
	/** The MIME type of the file. */
	pub mime_type: String,
	/** The ID of the file. */
	pub file_id: String,
}

/** The output of a code interpreter tool call that is a file. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterFileOutput {
	#[serde(rename="type")]
	/** The type of the code interpreter file output. Always `files`. */
	pub r#type: CodeInterpreterFileOutputType,
	pub files: Vec<CodeInterpreterFileOutputFiles>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeInterpreterOutputImageType {
	#[serde(rename="image")]
	Image,
}

/** The image output from the code interpreter. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterOutputImage {
	#[serde(rename="type")]
	/** The type of the output. Always 'image'. */
	pub r#type: CodeInterpreterOutputImageType,
	/** The URL of the image output from the code interpreter. */
	pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeInterpreterOutputLogsType {
	#[serde(rename="logs")]
	Logs,
}

/** The logs output from the code interpreter. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterOutputLogs {
	#[serde(rename="type")]
	/** The type of the output. Always 'logs'. */
	pub r#type: CodeInterpreterOutputLogsType,
	/** The logs output from the code interpreter. */
	pub logs: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeInterpreterTextOutputType {
	#[serde(rename="logs")]
	Logs,
}

/** The output of a code interpreter tool call that is text. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterTextOutput {
	#[serde(rename="type")]
	/** The type of the code interpreter text output. Always `logs`. */
	pub r#type: CodeInterpreterTextOutputType,
	/** The logs of the code interpreter tool call. */
	pub logs: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeInterpreterToolType {
	#[serde(rename="code_interpreter")]
	CodeInterpreter,
}

/** The code interpreter container. Can be a container ID or an object that
specifies uploaded file IDs to make available to your code. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeInterpreterToolContainer {
	/** The container ID. */
	String(String),
	CodeInterpreterToolAuto(CodeInterpreterToolAuto),
}

/** A tool that runs Python code to help generate a response to a prompt. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterTool {
	#[serde(rename="type")]
	/** The type of the code interpreter tool. Always `code_interpreter`. */
	pub r#type: CodeInterpreterToolType,
	/** The code interpreter container. Can be a container ID or an object that
specifies uploaded file IDs to make available to your code. */
	pub container: CodeInterpreterToolContainer,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeInterpreterToolAutoType {
	#[serde(rename="auto")]
	Auto,
}

/** Configuration for a code interpreter container. Optionally specify the IDs
of the files to run the code on. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterToolAuto {
	#[serde(rename="type")]
	/** Always `auto`. */
	pub r#type: CodeInterpreterToolAutoType,
	/** An optional list of uploaded files to make available to your code. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeInterpreterToolCallType {
	#[serde(rename="code_interpreter_call")]
	CodeInterpreterCall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CodeInterpreterToolCallStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
	#[serde(rename="interpreting")]
	Interpreting,
	#[serde(rename="failed")]
	Failed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeInterpreterToolCallOutputs {
	CodeInterpreterOutputLogs(CodeInterpreterOutputLogs),
	CodeInterpreterOutputImage(CodeInterpreterOutputImage),
}

/** A tool call to run code. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CodeInterpreterToolCall {
	#[serde(rename="type")]
	/** The type of the code interpreter tool call. Always `code_interpreter_call`. */
	pub r#type: CodeInterpreterToolCallType,
	/** The unique ID of the code interpreter tool call. */
	pub id: String,
	/** The status of the code interpreter tool call. */
	pub status: CodeInterpreterToolCallStatus,
	/** The ID of the container used to run the code. */
	pub container_id: String,
	/** The code to run, or null if not available. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	/** The outputs generated by the code interpreter, such as logs or images. 
Can be null if no outputs are available. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outputs: Option<Vec<CodeInterpreterToolCallOutputs>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComparisonFilterType {
	#[serde(rename="eq")]
	Eq,
	#[serde(rename="ne")]
	Ne,
	#[serde(rename="gt")]
	Gt,
	#[serde(rename="gte")]
	Gte,
	#[serde(rename="lt")]
	Lt,
	#[serde(rename="lte")]
	Lte,
}

/** The value to compare against the attribute key; supports string, number, or boolean types. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComparisonFilterValue {
	String(String),
	Number(f32),
	Boolean(bool),
}

/** A filter used to compare a specified attribute key to a given value using a defined comparison operation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComparisonFilter {
	#[serde(rename="type")]
	/** Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.
- `eq`: equals
- `ne`: not equal
- `gt`: greater than
- `gte`: greater than or equal
- `lt`: less than
- `lte`: less than or equal */
	pub r#type: ComparisonFilterType,
	/** The key to compare against the value. */
	pub key: String,
	/** The value to compare against the attribute key; supports string, number, or boolean types. */
	pub value: ComparisonFilterValue,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompleteUploadRequest {
	/** The ordered list of Part IDs. */
	pub part_ids: Vec<String>,
	/** The optional md5 checksum for the file contents to verify if the bytes uploaded matches what you expect. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub md5: Option<String>,
}

/** Breakdown of tokens used in a completion. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompletionUsageCompletionTokensDetails {
	/** When using Predicted Outputs, the number of tokens in the
prediction that appeared in the completion. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accepted_prediction_tokens: Option<u64>,
	/** Audio input tokens generated by the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<u64>,
	/** Tokens generated by the model for reasoning. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_tokens: Option<u64>,
	/** When using Predicted Outputs, the number of tokens in the
prediction that did not appear in the completion. However, like
reasoning tokens, these tokens are still counted in the total
completion tokens for purposes of billing, output, and context window
limits. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rejected_prediction_tokens: Option<u64>,
}

/** Breakdown of tokens used in the prompt. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompletionUsagePromptTokensDetails {
	/** Audio input tokens present in the prompt. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<u64>,
	/** Cached tokens present in the prompt. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cached_tokens: Option<u64>,
}

/** Usage statistics for the completion request. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompletionUsage {
	/** Number of tokens in the generated completion. */
	pub completion_tokens: u64,
	/** Number of tokens in the prompt. */
	pub prompt_tokens: u64,
	/** Total number of tokens used in the request (prompt + completion). */
	pub total_tokens: u64,
	/** Breakdown of tokens used in a completion. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completion_tokens_details: Option<CompletionUsageCompletionTokensDetails>,
	/** Breakdown of tokens used in the prompt. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt_tokens_details: Option<CompletionUsagePromptTokensDetails>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CompoundFilterType {
	#[serde(rename="and")]
	And,
	#[serde(rename="or")]
	Or,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompoundFilterFilters {
	ComparisonFilter(ComparisonFilter),
	Recursive(CompoundFilter),
}

/** Combine multiple filters using `and` or `or`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CompoundFilter {
	#[serde(rename="type")]
	/** Type of operation: `and` or `or`. */
	pub r#type: CompoundFilterType,
	/** Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`. */
	pub filters: Vec<CompoundFilterFilters>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputerAction {
	Click(Click),
	DoubleClick(DoubleClick),
	Drag(Drag),
	KeyPress(KeyPress),
	Move(Move),
	Screenshot(Screenshot),
	Scroll(Scroll),
	Type(Type),
	Wait(Wait),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputerScreenshotImageType {
	#[serde(rename="computer_screenshot")]
	ComputerScreenshot,
}

/** A computer screenshot image used with the computer use tool. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerScreenshotImage {
	#[serde(rename="type")]
	/** Specifies the event type. For a computer screenshot, this property is 
always set to `computer_screenshot`. */
	pub r#type: ComputerScreenshotImageType,
	/** The URL of the screenshot image. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
	/** The identifier of an uploaded file that contains the screenshot. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputerToolCallType {
	#[serde(rename="computer_call")]
	ComputerCall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputerToolCallStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** A tool call to a computer use tool. See the 
[computer use guide](/docs/guides/tools-computer-use) for more information. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCall {
	#[serde(rename="type")]
	/** The type of the computer call. Always `computer_call`. */
	pub r#type: ComputerToolCallType,
	/** The unique ID of the computer call. */
	pub id: String,
	/** An identifier used when responding to the tool call with output. */
	pub call_id: String,
	pub action: ComputerAction,
	/** The pending safety checks for the computer call. */
	pub pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,
	/** The status of the item. One of `in_progress`, `completed`, or
`incomplete`. Populated when items are returned via API. */
	pub status: ComputerToolCallStatus,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputerToolCallOutputType {
	#[serde(rename="computer_call_output")]
	ComputerCallOutput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputerToolCallOutputStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** The output of a computer tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCallOutput {
	#[serde(rename="type")]
	/** The type of the computer tool call output. Always `computer_call_output`. */
	pub r#type: ComputerToolCallOutputType,
	/** The ID of the computer tool call output. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The ID of the computer tool call that produced the output. */
	pub call_id: String,
	/** The safety checks reported by the API that have been acknowledged by the 
developer. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub acknowledged_safety_checks: Option<Vec<ComputerToolCallSafetyCheck>>,
	pub output: ComputerScreenshotImage,
	/** The status of the message input. One of `in_progress`, `completed`, or
`incomplete`. Populated when input items are returned via API. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<ComputerToolCallOutputStatus>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCallOutputResourceObject {
	/** The unique ID of the computer call tool output. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCallOutputResource {
	#[serde(flatten)]
	pub computer_tool_call_output: ComputerToolCallOutput,
	#[serde(flatten)]
	pub object: ComputerToolCallOutputResourceObject,
}

/** A pending safety check for the computer call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerToolCallSafetyCheck {
	/** The ID of the pending safety check. */
	pub id: String,
	/** The type of the pending safety check. */
	pub code: String,
	/** Details about the pending safety check. */
	pub message: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerFileListResourceObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerFileListResource {
	/** The type of object returned, must be 'list'. */
	pub object: ContainerFileListResourceObject,
	/** A list of container files. */
	pub data: Vec<ContainerFileResource>,
	/** The ID of the first file in the list. */
	pub first_id: String,
	/** The ID of the last file in the list. */
	pub last_id: String,
	/** Whether there are more files available. */
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerFileResource {
	/** Unique identifier for the file. */
	pub id: String,
	/** The type of this object (`container.file`). */
	pub object: String,
	/** The container this file belongs to. */
	pub container_id: String,
	/** Unix timestamp (in seconds) when the file was created. */
	pub created_at: u64,
	/** Size of the file in bytes. */
	pub bytes: u64,
	/** Path of the file in the container. */
	pub path: String,
	/** Source of the file (e.g., `user`, `assistant`). */
	pub source: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerListResourceObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerListResource {
	/** The type of object returned, must be 'list'. */
	pub object: ContainerListResourceObject,
	/** A list of containers. */
	pub data: Vec<ContainerResource>,
	/** The ID of the first container in the list. */
	pub first_id: String,
	/** The ID of the last container in the list. */
	pub last_id: String,
	/** Whether there are more containers available. */
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerResourceExpiresAfterAnchor {
	#[serde(rename="last_active_at")]
	LastActiveAt,
}

/** The container will expire after this time period.
The anchor is the reference point for the expiration.
The minutes is the number of minutes after the anchor before the container expires. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerResourceExpiresAfter {
	/** The reference point for the expiration. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub anchor: Option<ContainerResourceExpiresAfterAnchor>,
	/** The number of minutes after the anchor before the container expires. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minutes: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerResource {
	/** Unique identifier for the container. */
	pub id: String,
	/** The type of this object. */
	pub object: String,
	/** Name of the container. */
	pub name: String,
	/** Unix timestamp (in seconds) when the container was created. */
	pub created_at: u64,
	/** Status of the container (e.g., active, deleted). */
	pub status: String,
	/** The container will expire after this time period.
The anchor is the reference point for the expiration.
The minutes is the number of minutes after the anchor before the container expires. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_after: Option<ContainerResourceExpiresAfter>,
}

/** Multi-modal input and output contents. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
	InputContent(InputContent),
	OutputContent(OutputContent),
}

/** An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Coordinate {
	/** The x-coordinate. */
	pub x: u64,
	/** The y-coordinate. */
	pub y: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CostsResultObject {
	#[serde(rename="organization.costs.result")]
	OrganizationCostsResult,
}

/** The monetary value in its associated currency. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CostsResultAmount {
	/** The numeric value of the cost. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<f32>,
	/** Lowercase ISO-4217 currency e.g. "usd" */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency: Option<String>,
}

/** The aggregated costs details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CostsResult {
	pub object: CostsResultObject,
	/** The monetary value in its associated currency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub amount: Option<CostsResultAmount>,
	/** When `group_by=line_item`, this field provides the line item of the grouped costs result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub line_item: Option<String>,
	/** When `group_by=project_id`, this field provides the project ID of the grouped costs result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
}

/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestModel {
	String(String),
	AssistantSupportedModels(AssistantSupportedModels),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAssistantRequestTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAssistantRequestToolResourcesCodeInterpreter {
	/** A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

	/** The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAssistantRequestToolResourcesFileSearchVectorStoresChunkingStrategy(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAssistantRequestToolResourcesFileSearchVectorStores {
	/** A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
	/** The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<CreateAssistantRequestToolResourcesFileSearchVectorStoresChunkingStrategy>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAssistantRequestToolResourcesFileSearch {
	/** The [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
	/** A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this assistant. There can be a maximum of 1 vector store attached to the assistant. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_stores: Option<Vec<CreateAssistantRequestToolResourcesFileSearchVectorStores>>,
}

/** A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAssistantRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<CreateAssistantRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<CreateAssistantRequestToolResourcesFileSearch>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAssistantRequest {
	/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
	pub model: CreateAssistantRequestModel,
	/** The name of the assistant. The maximum length is 256 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The description of the assistant. The maximum length is 512 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** The system instructions that the assistant uses. The maximum length is 256,000 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	/** A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<CreateAssistantRequestTools>>,
	/** A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<CreateAssistantRequestToolResources>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or temperature but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChatCompletionRequestObjectWebSearchOptionsUserLocationType {
	#[serde(rename="approximate")]
	Approximate,
}

/** Approximate location parameters for the search. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionRequestObjectWebSearchOptionsUserLocation {
	#[serde(rename="type")]
	/** The type of location approximation. Always `approximate`. */
	pub r#type: CreateChatCompletionRequestObjectWebSearchOptionsUserLocationType,
	pub approximate: WebSearchLocation,
}

/** This tool searches the web for relevant results to use in a response.
Learn more about the [web search tool](/docs/guides/tools-web-search?api-mode=chat). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionRequestObjectWebSearchOptions {
	/** Approximate location parameters for the search. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_location: Option<CreateChatCompletionRequestObjectWebSearchOptionsUserLocation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub search_context_size: Option<WebSearchContextSize>,
}

/** An object specifying the format that the model must output.

Setting to `{ "type": "json_schema", "json_schema": {...} }` enables
Structured Outputs which ensures the model will match your supplied JSON
schema. Learn more in the [Structured Outputs
guide](/docs/guides/structured-outputs).

Setting to `{ "type": "json_object" }` enables the older JSON mode, which
ensures the message the model generates is valid JSON. Using `json_schema`
is preferred for models that support it. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestObjectResponseFormat {
	ResponseFormatText(ResponseFormatText),
	ResponseFormatJsonSchema(ResponseFormatJsonSchema),
	ResponseFormatJsonObject(ResponseFormatJsonObject),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChatCompletionRequestObjectAudioFormat {
	#[serde(rename="wav")]
	Wav,
	#[serde(rename="aac")]
	Aac,
	#[serde(rename="mp3")]
	Mp3,
	#[serde(rename="flac")]
	Flac,
	#[serde(rename="opus")]
	Opus,
	#[serde(rename="pcm16")]
	Pcm16,
}

/** Parameters for audio output. Required when audio output is requested with
`modalities: ["audio"]`. [Learn more](/docs/guides/audio). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionRequestObjectAudio {
	/** The voice the model uses to respond. Supported voices are 
`alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `nova`, `onyx`, `sage`, and `shimmer`. */
	pub voice: VoiceIdsShared,
	/** Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,
`opus`, or `pcm16`. */
	pub format: CreateChatCompletionRequestObjectAudioFormat,
}

	/** Modify the likelihood of specified tokens appearing in the completion.

Accepts a JSON object that maps tokens (specified by their token ID in the
tokenizer) to an associated bias value from -100 to 100. Mathematically,
the bias is added to the logits generated by the model prior to sampling.
The exact effect will vary per model, but values between -1 and 1 should
decrease or increase likelihood of selection; values like -100 or 100
should result in a ban or exclusive selection of the relevant token. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionRequestObjectLogitBias(pub HashMap<String,String>);

/** Configuration for a [Predicted Output](/docs/guides/predicted-outputs),
which can greatly improve response times when large parts of the model
response are known ahead of time. This is most common when you are
regenerating a file with only minor changes to most of the content. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestObjectPrediction {
	PredictionContent(PredictionContent),
}

/** Deprecated in favor of `tool_choice`.

Controls which (if any) function is called by the model.

`none` means the model will not call a function and instead generates a
message.

`auto` means the model can pick between generating a message or calling a
function.

Specifying a particular function via `{"name": "my_function"}` forces the
model to call that function.

`none` is the default when no functions are present. `auto` is the default
if functions are present. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateChatCompletionRequestObjectFunctionCall {
	/** `none` means the model will not call a function and instead generates a message. `auto` means the model can pick between generating a message or calling a function. */
	#[serde(rename="none")]
	None,
	#[serde(rename="auto")]
	Auto,
	ChatCompletionFunctionCallOption(ChatCompletionFunctionCallOption),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionRequestObject {
	/** A list of messages comprising the conversation so far. Depending on the
[model](/docs/models) you use, different message types (modalities) are
supported, like [text](/docs/guides/text-generation),
[images](/docs/guides/vision), and [audio](/docs/guides/audio). */
	pub messages: Vec<ChatCompletionRequestMessage>,
	/** Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI
offers a wide range of models with different capabilities, performance
characteristics, and price points. Refer to the [model guide](/docs/models)
to browse and compare available models. */
	pub model: ModelIdsShared,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<ResponseModalities>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	/** An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and [reasoning tokens](/docs/guides/reasoning). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<u64>,
	/** Number between -2.0 and 2.0. Positive values penalize new tokens based on
their existing frequency in the text so far, decreasing the model's
likelihood to repeat the same line verbatim. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub frequency_penalty: Option<f32>,
	/** Number between -2.0 and 2.0. Positive values penalize new tokens based on
whether they appear in the text so far, increasing the model's likelihood
to talk about new topics. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub presence_penalty: Option<f32>,
	/** This tool searches the web for relevant results to use in a response.
Learn more about the [web search tool](/docs/guides/tools-web-search?api-mode=chat). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub web_search_options: Option<CreateChatCompletionRequestObjectWebSearchOptions>,
	/** An integer between 0 and 20 specifying the number of most likely tokens to
return at each token position, each with an associated log probability.
`logprobs` must be set to `true` if this parameter is used. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_logprobs: Option<u64>,
	/** An object specifying the format that the model must output.

Setting to `{ "type": "json_schema", "json_schema": {...} }` enables
Structured Outputs which ensures the model will match your supplied JSON
schema. Learn more in the [Structured Outputs
guide](/docs/guides/structured-outputs).

Setting to `{ "type": "json_object" }` enables the older JSON mode, which
ensures the message the model generates is valid JSON. Using `json_schema`
is preferred for models that support it. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateChatCompletionRequestObjectResponseFormat>,
	/** Parameters for audio output. Required when audio output is requested with
`modalities: ["audio"]`. [Learn more](/docs/guides/audio). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<CreateChatCompletionRequestObjectAudio>,
	/** Whether or not to store the output of this chat completion request for 
use in our [model distillation](/docs/guides/distillation) or
[evals](/docs/guides/evals) products. 

Supports text and image inputs. Note: image inputs over 10MB will be dropped. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub store: Option<bool>,
	/** If set to true, the model response data will be streamed to the client
as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
See the [Streaming section below](/docs/api-reference/chat/streaming)
for more information, along with the [streaming responses](/docs/guides/streaming-responses)
guide for more information on how to handle the streaming events. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop: Option<StopConfiguration>,
	/** Modify the likelihood of specified tokens appearing in the completion.

Accepts a JSON object that maps tokens (specified by their token ID in the
tokenizer) to an associated bias value from -100 to 100. Mathematically,
the bias is added to the logits generated by the model prior to sampling.
The exact effect will vary per model, but values between -1 and 1 should
decrease or increase likelihood of selection; values like -100 or 100
should result in a ban or exclusive selection of the relevant token. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logit_bias: Option<CreateChatCompletionRequestObjectLogitBias>,
	/** Whether to return log probabilities of the output tokens or not. If true,
returns the log probabilities of each output token returned in the
`content` of `message`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<bool>,
	/** The maximum number of [tokens](/tokenizer) that can be generated in the
chat completion. This value can be used to control
[costs](https://openai.com/api/pricing/) for text generated via API.

This value is now deprecated in favor of `max_completion_tokens`, and is
not compatible with [o-series models](/docs/guides/reasoning). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tokens: Option<u64>,
	/** How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep `n` as `1` to minimize costs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<u64>,
	/** Configuration for a [Predicted Output](/docs/guides/predicted-outputs),
which can greatly improve response times when large parts of the model
response are known ahead of time. This is most common when you are
regenerating a file with only minor changes to most of the content. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prediction: Option<CreateChatCompletionRequestObjectPrediction>,
	/** This feature is in Beta.
If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.
Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream_options: Option<ChatCompletionStreamOptions>,
	/** A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<ChatCompletionTool>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<ChatCompletionToolChoiceOption>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<ParallelToolCalls>,
	/** Deprecated in favor of `tool_choice`.

Controls which (if any) function is called by the model.

`none` means the model will not call a function and instead generates a
message.

`auto` means the model can pick between generating a message or calling a
function.

Specifying a particular function via `{"name": "my_function"}` forces the
model to call that function.

`none` is the default when no functions are present. `auto` is the default
if functions are present. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function_call: Option<CreateChatCompletionRequestObjectFunctionCall>,
	/** Deprecated in favor of `tools`.

A list of functions the model may generate JSON inputs for. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub functions: Option<Vec<ChatCompletionFunctions>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionRequest {
	#[serde(flatten)]
	pub create_model_response_properties: CreateModelResponseProperties,
	#[serde(flatten)]
	pub object: CreateChatCompletionRequestObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChatCompletionResponseChoicesFinishReason {
	#[serde(rename="stop")]
	Stop,
	#[serde(rename="length")]
	Length,
	#[serde(rename="tool_calls")]
	ToolCalls,
	#[serde(rename="content_filter")]
	ContentFilter,
	#[serde(rename="function_call")]
	FunctionCall,
}

/** Log probability information for the choice. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionResponseChoicesLogprobs {
	/** A list of message content tokens with log probability information. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<ChatCompletionTokenLogprob>>,
	/** A list of message refusal tokens with log probability information. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionResponseChoices {
	/** The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
`length` if the maximum number of tokens specified in the request was reached,
`content_filter` if content was omitted due to a flag from our content filters,
`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function. */
	pub finish_reason: CreateChatCompletionResponseChoicesFinishReason,
	/** The index of the choice in the list of choices. */
	pub index: u64,
	pub message: ChatCompletionResponseMessage,
	/** Log probability information for the choice. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<CreateChatCompletionResponseChoicesLogprobs>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChatCompletionResponseObject {
	#[serde(rename="chat.completion")]
	ChatCompletion,
}

/** Represents a chat completion response returned by model, based on the provided input. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionResponse {
	/** A unique identifier for the chat completion. */
	pub id: String,
	/** A list of chat completion choices. Can be more than one if `n` is greater than 1. */
	pub choices: Vec<CreateChatCompletionResponseChoices>,
	/** The Unix timestamp (in seconds) of when the chat completion was created. */
	pub created: u64,
	/** The model used for the chat completion. */
	pub model: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_tier: Option<ServiceTier>,
	/** This fingerprint represents the backend configuration that the model runs with.

Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_fingerprint: Option<String>,
	/** The object type, which is always `chat.completion`. */
	pub object: CreateChatCompletionResponseObject,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<CompletionUsage>,
}

/** Log probability information for the choice. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionStreamResponseChoicesLogprobs {
	/** A list of message content tokens with log probability information. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<ChatCompletionTokenLogprob>>,
	/** A list of message refusal tokens with log probability information. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChatCompletionStreamResponseChoicesFinishReason {
	#[serde(rename="stop")]
	Stop,
	#[serde(rename="length")]
	Length,
	#[serde(rename="tool_calls")]
	ToolCalls,
	#[serde(rename="content_filter")]
	ContentFilter,
	#[serde(rename="function_call")]
	FunctionCall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionStreamResponseChoices {
	pub delta: ChatCompletionStreamResponseDelta,
	/** Log probability information for the choice. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<CreateChatCompletionStreamResponseChoicesLogprobs>,
	/** The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
`length` if the maximum number of tokens specified in the request was reached,
`content_filter` if content was omitted due to a flag from our content filters,
`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub finish_reason: Option<CreateChatCompletionStreamResponseChoicesFinishReason>,
	/** The index of the choice in the list of choices. */
	pub index: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateChatCompletionStreamResponseObject {
	#[serde(rename="chat.completion.chunk")]
	ChatCompletionChunk,
}

/** Represents a streamed chunk of a chat completion response returned
by the model, based on the provided input. 
[Learn more](/docs/guides/streaming-responses). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateChatCompletionStreamResponse {
	/** A unique identifier for the chat completion. Each chunk has the same ID. */
	pub id: String,
	/** A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the
last chunk if you set `stream_options: {"include_usage": true}`. */
	pub choices: Vec<CreateChatCompletionStreamResponseChoices>,
	/** The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp. */
	pub created: u64,
	/** The model to generate the completion. */
	pub model: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_tier: Option<ServiceTier>,
	/** This fingerprint represents the backend configuration that the model runs with.
Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_fingerprint: Option<String>,
	/** The object type, which is always `chat.completion.chunk`. */
	pub object: CreateChatCompletionStreamResponseObject,
	/** An optional field that will only be present when you set
`stream_options: {"include_usage": true}` in your request. When present, it
contains a null value **except for the last chunk** which contains the
token usage statistics for the entire request.

**NOTE:** If the stream is interrupted or cancelled, you may not
receive the final usage chunk which contains the total token usage for
the request. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<CompletionUsage>,
}

/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCompletionRequestModel {
	String(String),
	#[serde(rename="gpt-3.5-turbo-instruct")]
	Gpt35TurboInstruct,
	#[serde(rename="davinci-002")]
	Davinci002,
	#[serde(rename="babbage-002")]
	Babbage002,
}

/** The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.

Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCompletionRequestPrompt {
	String(String),
	ArrayString(Vec<String>),
	ArrayNumber(Vec<u64>),
	ArrayListNumber(Vec<Vec<u64>>),
}

	/** Modify the likelihood of specified tokens appearing in the completion.

Accepts a JSON object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCompletionRequestLogitBias(pub HashMap<String,String>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCompletionRequest {
	/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
	pub model: CreateCompletionRequestModel,
	/** The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.

Note that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<CreateCompletionRequestPrompt>,
	/** Generates `best_of` completions server-side and returns the "best" (the one with the highest log probability per token). Results cannot be streamed.

When used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.

**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub best_of: Option<u64>,
	/** Echo back the prompt in addition to the completion */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub echo: Option<bool>,
	/** Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.

[See more information about frequency and presence penalties.](/docs/guides/text-generation) */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub frequency_penalty: Option<f32>,
	/** Modify the likelihood of specified tokens appearing in the completion.

Accepts a JSON object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.

As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logit_bias: Option<CreateCompletionRequestLogitBias>,
	/** Include the log probabilities on the `logprobs` most likely output tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.

The maximum value for `logprobs` is 5. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<u64>,
	/** The maximum number of [tokens](/tokenizer) that can be generated in the completion.

The token count of your prompt plus `max_tokens` cannot exceed the model's context length. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tokens: Option<u64>,
	/** How many completions to generate for each prompt.

**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<u64>,
	/** Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.

[See more information about frequency and presence penalties.](/docs/guides/text-generation) */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub presence_penalty: Option<f32>,
	/** If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.

Determinism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stop: Option<StopConfiguration>,
	/** Whether to stream back partial progress. If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message. [Example Python code](https://cookbook.openai.com/examples/how_to_stream_completions). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream_options: Option<ChatCompletionStreamOptions>,
	/** The suffix that comes after a completion of inserted text.

This parameter is only supported for `gpt-3.5-turbo-instruct`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suffix: Option<String>,
	/** What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.

We generally recommend altering this or `top_p` but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or `temperature` but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	/** A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateCompletionResponseChoicesFinishReason {
	#[serde(rename="stop")]
	Stop,
	#[serde(rename="length")]
	Length,
	#[serde(rename="content_filter")]
	ContentFilter,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCompletionResponseChoicesLogprobsTopLogprobs(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCompletionResponseChoicesLogprobs {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text_offset: Option<Vec<u64>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token_logprobs: Option<Vec<f32>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tokens: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_logprobs: Option<Vec<CreateCompletionResponseChoicesLogprobsTopLogprobs>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCompletionResponseChoices {
	/** The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,
`length` if the maximum number of tokens specified in the request was reached,
or `content_filter` if content was omitted due to a flag from our content filters. */
	pub finish_reason: CreateCompletionResponseChoicesFinishReason,
	pub index: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<CreateCompletionResponseChoicesLogprobs>,
	pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateCompletionResponseObject {
	#[serde(rename="text_completion")]
	TextCompletion,
}

/** Represents a completion response from the API. Note: both the streamed and non-streamed response objects share the same shape (unlike the chat endpoint). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateCompletionResponse {
	/** A unique identifier for the completion. */
	pub id: String,
	/** The list of completion choices the model generated for the input prompt. */
	pub choices: Vec<CreateCompletionResponseChoices>,
	/** The Unix timestamp (in seconds) of when the completion was created. */
	pub created: u64,
	/** The model used for completion. */
	pub model: String,
	/** This fingerprint represents the backend configuration that the model runs with.

Can be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub system_fingerprint: Option<String>,
	/** The object type, which is always "text_completion" */
	pub object: CreateCompletionResponseObject,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<CompletionUsage>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateContainerBodyExpiresAfterAnchor {
	#[serde(rename="last_active_at")]
	LastActiveAt,
}

/** Container expiration time in seconds relative to the 'anchor' time. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateContainerBodyExpiresAfter {
	/** Time anchor for the expiration time. Currently only 'last_active_at' is supported. */
	pub anchor: CreateContainerBodyExpiresAfterAnchor,
	pub minutes: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateContainerBody {
	/** Name of the container to create. */
	pub name: String,
	/** IDs of files to copy to the container. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
	/** Container expiration time in seconds relative to the 'anchor' time. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_after: Option<CreateContainerBodyExpiresAfter>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateContainerFileBody {
	/** Name of the file to create. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/** The File object (not file name) to be uploaded. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file: Option<crate::multipart::File>,
}

/** Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for all embedding models), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. In addition to the per-input token limit, all embedding  models enforce a maximum of 300,000 tokens summed across all inputs in a  single request. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmbeddingRequestInput {
	/** The string that will be turned into an embedding. */
	String(String),
	/** The array of strings that will be turned into an embedding. */
	ArrayString(Vec<String>),
	/** The array of integers that will be turned into an embedding. */
	ArrayNumber(Vec<u64>),
	/** The array of arrays containing integers that will be turned into an embedding. */
	ArrayListNumber(Vec<Vec<u64>>),
}

/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEmbeddingRequestModel {
	String(String),
	#[serde(rename="text-embedding-ada-002")]
	TextEmbeddingAda002,
	#[serde(rename="text-embedding-3-small")]
	TextEmbedding3Small,
	#[serde(rename="text-embedding-3-large")]
	TextEmbedding3Large,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEmbeddingRequestEncodingFormat {
	#[serde(rename="float")]
	Float,
	#[serde(rename="base64")]
	Base64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEmbeddingRequest {
	/** Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for all embedding models), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. In addition to the per-input token limit, all embedding  models enforce a maximum of 300,000 tokens summed across all inputs in a  single request. */
	pub input: CreateEmbeddingRequestInput,
	/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
	pub model: CreateEmbeddingRequestModel,
	/** The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encoding_format: Option<CreateEmbeddingRequestEncodingFormat>,
	/** The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dimensions: Option<u64>,
	/** A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEmbeddingResponseObject {
	#[serde(rename="list")]
	List,
}

/** The usage information for the request. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEmbeddingResponseUsage {
	/** The number of tokens used by the prompt. */
	pub prompt_tokens: u64,
	/** The total number of tokens used by the request. */
	pub total_tokens: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEmbeddingResponse {
	/** The list of embeddings generated by the model. */
	pub data: Vec<Embedding>,
	/** The name of the model used to generate the embedding. */
	pub model: String,
	/** The object type, which is always "list". */
	pub object: CreateEmbeddingResponseObject,
	/** The usage information for the request. */
	pub usage: CreateEmbeddingResponseUsage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEvalCompletionsRunDataSourceType {
	#[serde(rename="completions")]
	Completions,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalCompletionsRunDataSourceInputMessagesTemplate {
	EasyInputMessage(EasyInputMessage),
	EvalItem(EvalItem),
}

/** Used when sampling from a model. Dictates the structure of the messages passed into the model. Can either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with variable references to the `item` namespace. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalCompletionsRunDataSourceInputMessages {
	TemplateInputMessages {
		#[serde(rename="type")]
		r#type: String,
		template: Vec<CreateEvalCompletionsRunDataSourceInputMessagesTemplate>,
	},
	ItemReferenceInputMessages {
		#[serde(rename="type")]
		r#type: String,
		item_reference: String,
	},
}

/** An object specifying the format that the model must output.

Setting to `{ "type": "json_schema", "json_schema": {...} }` enables
Structured Outputs which ensures the model will match your supplied JSON
schema. Learn more in the [Structured Outputs
guide](/docs/guides/structured-outputs).

Setting to `{ "type": "json_object" }` enables the older JSON mode, which
ensures the message the model generates is valid JSON. Using `json_schema`
is preferred for models that support it. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat {
	ResponseFormatText(ResponseFormatText),
	ResponseFormatJsonSchema(ResponseFormatJsonSchema),
	ResponseFormatJsonObject(ResponseFormatJsonObject),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalCompletionsRunDataSourceSamplingParams {
	/** A higher temperature increases randomness in the outputs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** The maximum number of tokens in the generated output. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<u64>,
	/** An alternative to temperature for nucleus sampling; 1.0 includes all tokens. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	/** A seed value to initialize the randomness, during sampling. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<u64>,
	/** An object specifying the format that the model must output.

Setting to `{ "type": "json_schema", "json_schema": {...} }` enables
Structured Outputs which ensures the model will match your supplied JSON
schema. Learn more in the [Structured Outputs
guide](/docs/guides/structured-outputs).

Setting to `{ "type": "json_object" }` enables the older JSON mode, which
ensures the message the model generates is valid JSON. Using `json_schema`
is preferred for models that support it. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateEvalCompletionsRunDataSourceSamplingParamsResponseFormat>,
	/** A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<ChatCompletionTool>>,
}

/** Determines what populates the `item` namespace in this run's data source. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalCompletionsRunDataSourceSource {
	EvalJsonlFileContentSource(EvalJsonlFileContentSource),
	EvalJsonlFileIdSource(EvalJsonlFileIdSource),
	EvalStoredCompletionsSource(EvalStoredCompletionsSource),
}

/** A CompletionsRunDataSource object describing a model sampling configuration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalCompletionsRunDataSource {
	#[serde(rename="type")]
	/** The type of run data source. Always `completions`. */
	pub r#type: CreateEvalCompletionsRunDataSourceType,
	/** Used when sampling from a model. Dictates the structure of the messages passed into the model. Can either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with variable references to the `item` namespace. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_messages: Option<CreateEvalCompletionsRunDataSourceInputMessages>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sampling_params: Option<CreateEvalCompletionsRunDataSourceSamplingParams>,
	/** The name of the model to use for generating completions (e.g. "o3-mini"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/** Determines what populates the `item` namespace in this run's data source. */
	pub source: CreateEvalCompletionsRunDataSourceSource,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEvalCustomDataSourceConfigType {
	#[serde(rename="custom")]
	Custom,
}

	/** The json schema for each row in the data source. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalCustomDataSourceConfigItemSchema(pub String);

/** A CustomDataSourceConfig object that defines the schema for the data source used for the evaluation runs.
This schema is used to define the shape of the data that will be:
- Used to define your testing criteria and
- What data is required when creating a run */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalCustomDataSourceConfig {
	#[serde(rename="type")]
	/** The type of data source. Always `custom`. */
	pub r#type: CreateEvalCustomDataSourceConfigType,
	/** The json schema for each row in the data source. */
	pub item_schema: CreateEvalCustomDataSourceConfigItemSchema,
	/** Whether the eval should expect you to populate the sample namespace (ie, by generating responses off of your data source) */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include_sample_schema: Option<bool>,
}

/** A chat message that makes up the prompt or context. May include variable references to the `item` namespace, ie {{item.name}}. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalItem {
	SimpleInputMessage {
		role: String,
		content: String,
	},
	EvalItem(EvalItem),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEvalJsonlRunDataSourceType {
	#[serde(rename="jsonl")]
	Jsonl,
}

/** Determines what populates the `item` namespace in the data source. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalJsonlRunDataSourceSource {
	EvalJsonlFileContentSource(EvalJsonlFileContentSource),
	EvalJsonlFileIdSource(EvalJsonlFileIdSource),
}

/** A JsonlRunDataSource object with that specifies a JSONL file that matches the eval */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalJsonlRunDataSource {
	#[serde(rename="type")]
	/** The type of data source. Always `jsonl`. */
	pub r#type: CreateEvalJsonlRunDataSourceType,
	/** Determines what populates the `item` namespace in the data source. */
	pub source: CreateEvalJsonlRunDataSourceSource,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEvalLabelModelGraderType {
	#[serde(rename="label_model")]
	LabelModel,
}

/** A LabelModelGrader object which uses a model to assign labels to each item
in the evaluation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalLabelModelGrader {
	#[serde(rename="type")]
	/** The object type, which is always `label_model`. */
	pub r#type: CreateEvalLabelModelGraderType,
	/** The name of the grader. */
	pub name: String,
	/** The model to use for the evaluation. Must support structured outputs. */
	pub model: String,
	/** A list of chat messages forming the prompt or context. May include variable references to the `item` namespace, ie {{item.name}}. */
	pub input: Vec<CreateEvalItem>,
	/** The labels to classify to each item in the evaluation. */
	pub labels: Vec<String>,
	/** The labels that indicate a passing result. Must be a subset of labels. */
	pub passing_labels: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEvalLogsDataSourceConfigType {
	#[serde(rename="logs")]
	Logs,
}

	/** Metadata filters for the logs data source. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalLogsDataSourceConfigMetadata(pub String);

/** A data source config which specifies the metadata property of your logs query.
This is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalLogsDataSourceConfig {
	#[serde(rename="type")]
	/** The type of data source. Always `logs`. */
	pub r#type: CreateEvalLogsDataSourceConfigType,
	/** Metadata filters for the logs data source. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<CreateEvalLogsDataSourceConfigMetadata>,
}

	/** The configuration for the data source used for the evaluation runs. Dictates the schema of the data used in the evaluation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalRequestDataSourceConfig(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalRequestTestingCriteria {
	CreateEvalLabelModelGrader(CreateEvalLabelModelGrader),
	EvalGraderStringCheck(EvalGraderStringCheck),
	EvalGraderTextSimilarity(EvalGraderTextSimilarity),
	EvalGraderPython(EvalGraderPython),
	EvalGraderScoreModel(EvalGraderScoreModel),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalRequest {
	/** The name of the evaluation. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** The configuration for the data source used for the evaluation runs. Dictates the schema of the data used in the evaluation. */
	pub data_source_config: CreateEvalRequestDataSourceConfig,
	/** A list of graders for all eval runs in this group. Graders can reference variables in the data source using double curly braces notation, like `{{item.variable_name}}`. To reference the model's output, use the `sample` namespace (ie, `{{sample.output_text}}`). */
	pub testing_criteria: Vec<CreateEvalRequestTestingCriteria>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEvalResponsesRunDataSourceType {
	#[serde(rename="responses")]
	Responses,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalResponsesRunDataSourceInputMessagesTemplate {
	ChatMessage {
		role: String,
		content: String,
	},
	EvalItem(EvalItem),
}

/** Used when sampling from a model. Dictates the structure of the messages passed into the model. Can either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with variable references to the `item` namespace. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalResponsesRunDataSourceInputMessages {
	InputMessagesTemplate {
		#[serde(rename="type")]
		r#type: String,
		template: Vec<CreateEvalResponsesRunDataSourceInputMessagesTemplate>,
	},
	InputMessagesItemReference {
		#[serde(rename="type")]
		r#type: String,
		item_reference: String,
	},
}

/** Configuration options for a text response from the model. Can be plain
text or structured JSON data. Learn more:
- [Text inputs and outputs](/docs/guides/text)
- [Structured Outputs](/docs/guides/structured-outputs) */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalResponsesRunDataSourceSamplingParamsText {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<TextResponseFormatConfiguration>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalResponsesRunDataSourceSamplingParams {
	/** A higher temperature increases randomness in the outputs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** The maximum number of tokens in the generated output. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<u64>,
	/** An alternative to temperature for nucleus sampling; 1.0 includes all tokens. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	/** A seed value to initialize the randomness, during sampling. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<u64>,
	/** An array of tools the model may call while generating a response. You
can specify which tool to use by setting the `tool_choice` parameter.

The two categories of tools you can provide the model are:

- **Built-in tools**: Tools that are provided by OpenAI that extend the
  model's capabilities, like [web search](/docs/guides/tools-web-search)
  or [file search](/docs/guides/tools-file-search). Learn more about
  [built-in tools](/docs/guides/tools).
- **Function calls (custom tools)**: Functions that are defined by you,
  enabling the model to call your own code. Learn more about
  [function calling](/docs/guides/function-calling). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<Tool>>,
	/** Configuration options for a text response from the model. Can be plain
text or structured JSON data. Learn more:
- [Text inputs and outputs](/docs/guides/text)
- [Structured Outputs](/docs/guides/structured-outputs) */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<CreateEvalResponsesRunDataSourceSamplingParamsText>,
}

/** Determines what populates the `item` namespace in this run's data source. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEvalResponsesRunDataSourceSource {
	EvalJsonlFileContentSource(EvalJsonlFileContentSource),
	EvalJsonlFileIdSource(EvalJsonlFileIdSource),
	EvalResponsesSource(EvalResponsesSource),
}

/** A ResponsesRunDataSource object describing a model sampling configuration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalResponsesRunDataSource {
	#[serde(rename="type")]
	/** The type of run data source. Always `responses`. */
	pub r#type: CreateEvalResponsesRunDataSourceType,
	/** Used when sampling from a model. Dictates the structure of the messages passed into the model. Can either be a reference to a prebuilt trajectory (ie, `item.input_trajectory`), or a template with variable references to the `item` namespace. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_messages: Option<CreateEvalResponsesRunDataSourceInputMessages>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sampling_params: Option<CreateEvalResponsesRunDataSourceSamplingParams>,
	/** The name of the model to use for generating completions (e.g. "o3-mini"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/** Determines what populates the `item` namespace in this run's data source. */
	pub source: CreateEvalResponsesRunDataSourceSource,
}

	/** Details about the run's data source. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalRunRequestDataSource(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalRunRequest {
	/** The name of the run. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** Details about the run's data source. */
	pub data_source: CreateEvalRunRequestDataSource,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateEvalStoredCompletionsDataSourceConfigType {
	#[serde(rename="stored_completions")]
	StoredCompletions,
}

	/** Metadata filters for the stored completions data source. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalStoredCompletionsDataSourceConfigMetadata(pub String);

/** Deprecated in favor of LogsDataSourceConfig. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateEvalStoredCompletionsDataSourceConfig {
	#[serde(rename="type")]
	/** The type of data source. Always `stored_completions`. */
	pub r#type: CreateEvalStoredCompletionsDataSourceConfigType,
	/** Metadata filters for the stored completions data source. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<CreateEvalStoredCompletionsDataSourceConfigMetadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateFileRequestPurpose {
	#[serde(rename="assistants")]
	Assistants,
	#[serde(rename="batch")]
	Batch,
	#[serde(rename="fine-tune")]
	FineTune,
	#[serde(rename="vision")]
	Vision,
	#[serde(rename="user_data")]
	UserData,
	#[serde(rename="evals")]
	Evals,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFileRequest {
	/** The File object (not file name) to be uploaded. */
	pub file: crate::multipart::File,
	/** The intended purpose of the uploaded file. One of: - `assistants`: Used in the Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`: Flexible file type for any purpose - `evals`: Used for eval data sets */
	pub purpose: CreateFileRequestPurpose,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFineTuningCheckpointPermissionRequest {
	/** The project identifiers to grant access to. */
	pub project_ids: Vec<String>,
}

/** The name of the model to fine-tune. You can select one of the
[supported models](/docs/guides/fine-tuning#which-models-can-be-fine-tuned). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestModel {
	String(String),
	#[serde(rename="babbage-002")]
	Babbage002,
	#[serde(rename="davinci-002")]
	Davinci002,
	#[serde(rename="gpt-3.5-turbo")]
	Gpt35Turbo,
	#[serde(rename="gpt-4o-mini")]
	Gpt4oMini,
}

/** Number of examples in each batch. A larger batch size means that model parameters
are updated less frequently, but with lower variance. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersBatchSize {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
overfitting. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
	#[serde(rename="auto")]
	Auto,
	Number(f32),
}

/** The number of epochs to train the model for. An epoch refers to one full cycle
through the training dataset. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestHyperparametersNEpochs {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** The hyperparameters used for the fine-tuning job.
This value is now deprecated in favor of `method`, and should be passed in under the `method` parameter. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFineTuningJobRequestHyperparameters {
	/** Number of examples in each batch. A larger batch size means that model parameters
are updated less frequently, but with lower variance. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<CreateFineTuningJobRequestHyperparametersBatchSize>,
	/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
overfitting. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<CreateFineTuningJobRequestHyperparametersLearningRateMultiplier>,
	/** The number of epochs to train the model for. An epoch refers to one full cycle
through the training dataset. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<CreateFineTuningJobRequestHyperparametersNEpochs>,
}

/** The type of integration to enable. Currently, only "wandb" (Weights and Biases) is supported. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFineTuningJobRequestIntegrationsType {
	#[serde(rename="wandb")]
	Wandb,
}

/** The settings for your integration with Weights and Biases. This payload specifies the project that
metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
to your run, and set a default entity (team, username, etc) to be associated with your run. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFineTuningJobRequestIntegrationsWandb {
	/** The name of the project that the new run will be created under. */
	pub project: String,
	/** A display name to set for the run. If not set, we will use the Job ID as the name. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The entity to use for the run. This allows you to set the team or username of the WandB user that you would
like associated with the run. If not set, the default entity for the registered WandB API key is used. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity: Option<String>,
	/** A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some
default tags are generated by OpenAI: "openai/finetune", "openai/{base-model}", "openai/{ftjob-abcdef}". */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFineTuningJobRequestIntegrations {
	#[serde(rename="type")]
	/** The type of integration to enable. Currently, only "wandb" (Weights and Biases) is supported. */
	pub r#type: CreateFineTuningJobRequestIntegrationsType,
	/** The settings for your integration with Weights and Biases. This payload specifies the project that
metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
to your run, and set a default entity (team, username, etc) to be associated with your run. */
	pub wandb: CreateFineTuningJobRequestIntegrationsWandb,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateFineTuningJobRequest {
	/** The name of the model to fine-tune. You can select one of the
[supported models](/docs/guides/fine-tuning#which-models-can-be-fine-tuned). */
	pub model: CreateFineTuningJobRequestModel,
	/** The ID of an uploaded file that contains training data.

See [upload file](/docs/api-reference/files/create) for how to upload a file.

Your dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.

The contents of the file should differ depending on if the model uses the [chat](/docs/api-reference/fine-tuning/chat-input), [completions](/docs/api-reference/fine-tuning/completions-input) format, or if the fine-tuning method uses the [preference](/docs/api-reference/fine-tuning/preference-input) format.

See the [fine-tuning guide](/docs/guides/model-optimization) for more details. */
	pub training_file: String,
	/** The hyperparameters used for the fine-tuning job.
This value is now deprecated in favor of `method`, and should be passed in under the `method` parameter. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<CreateFineTuningJobRequestHyperparameters>,
	/** A string of up to 64 characters that will be added to your fine-tuned model name.

For example, a `suffix` of "custom-model-name" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suffix: Option<String>,
	/** The ID of an uploaded file that contains validation data.

If you provide this file, the data is used to generate validation
metrics periodically during fine-tuning. These metrics can be viewed in
the fine-tuning results file.
The same data should not be present in both train and validation files.

Your dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.

See the [fine-tuning guide](/docs/guides/model-optimization) for more details. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub validation_file: Option<String>,
	/** A list of integrations to enable for your fine-tuning job. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integrations: Option<Vec<CreateFineTuningJobRequestIntegrations>>,
	/** The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases.
If a seed is not specified, one will be generated for you. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seed: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub method: Option<FineTuneMethod>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

/** The image(s) to edit. Must be a supported image file or an array of images.

For `gpt-image-1`, each image should be a `png`, `webp`, or `jpg` file less 
than 50MB. You can provide up to 16 images.

For `dall-e-2`, you can only provide one image, and it should be a square 
`png` file less than 4MB. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateImageEditRequestImage {
	String(String),
	ArrayString(Vec<String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageEditRequestBackground {
	#[serde(rename="transparent")]
	Transparent,
	#[serde(rename="opaque")]
	Opaque,
	#[serde(rename="auto")]
	Auto,
}

/** The model to use for image generation. Only `dall-e-2` and `gpt-image-1` are supported. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateImageEditRequestModel {
	String(String),
	#[serde(rename="dall-e-2")]
	DallE2,
	#[serde(rename="gpt-image-1")]
	GptImage1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageEditRequestSize {
	#[serde(rename="256x256")]
	Size256x256,
	#[serde(rename="512x512")]
	Size512x512,
	#[serde(rename="1024x1024")]
	Size1024x1024,
	#[serde(rename="1536x1024")]
	Size1536x1024,
	#[serde(rename="1024x1536")]
	Size1024x1536,
	#[serde(rename="auto")]
	Auto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageEditRequestResponseFormat {
	#[serde(rename="url")]
	Url,
	#[serde(rename="b64_json")]
	B64Json,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageEditRequestOutputFormat {
	#[serde(rename="png")]
	Png,
	#[serde(rename="jpeg")]
	Jpeg,
	#[serde(rename="webp")]
	Webp,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageEditRequestQuality {
	#[serde(rename="standard")]
	Standard,
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
	#[serde(rename="auto")]
	Auto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateImageEditRequest {
	/** The image(s) to edit. Must be a supported image file or an array of images.

For `gpt-image-1`, each image should be a `png`, `webp`, or `jpg` file less 
than 50MB. You can provide up to 16 images.

For `dall-e-2`, you can only provide one image, and it should be a square 
`png` file less than 4MB. */
	pub image: CreateImageEditRequestImage,
	/** A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2`, and 32000 characters for `gpt-image-1`. */
	pub prompt: String,
	/** An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. If there are multiple images provided, the mask will be applied on the first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mask: Option<Vec<u8>>,
	/** Allows to set transparency for the background of the generated image(s). 
This parameter is only supported for `gpt-image-1`. Must be one of 
`transparent`, `opaque` or `auto` (default value). When `auto` is used, the 
model will automatically determine the best background for the image.

If `transparent`, the output format needs to support transparency, so it 
should be set to either `png` (default value) or `webp`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub background: Option<CreateImageEditRequestBackground>,
	/** The model to use for image generation. Only `dall-e-2` and `gpt-image-1` are supported. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateImageEditRequestModel>,
	/** The number of images to generate. Must be between 1 and 10. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<u64>,
	/** The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, and one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<CreateImageEditRequestSize>,
	/** The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter is only supported for `dall-e-2`, as `gpt-image-1` will always return base64-encoded images. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateImageEditRequestResponseFormat>,
	/** The format in which the generated images are returned. This parameter is
only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`.
The default value is `png`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_format: Option<CreateImageEditRequestOutputFormat>,
	/** The compression level (0-100%) for the generated images. This parameter 
is only supported for `gpt-image-1` with the `webp` or `jpeg` output 
formats, and defaults to 100. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_compression: Option<u64>,
	/** A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
	/** The quality of the image that will be generated. `high`, `medium` and `low` are only supported for `gpt-image-1`. `dall-e-2` only supports `standard` quality. Defaults to `auto`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quality: Option<CreateImageEditRequestQuality>,
}

/** The model to use for image generation. One of `dall-e-2`, `dall-e-3`, or `gpt-image-1`. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateImageRequestModel {
	String(String),
	#[serde(rename="dall-e-2")]
	DallE2,
	#[serde(rename="dall-e-3")]
	DallE3,
	#[serde(rename="gpt-image-1")]
	GptImage1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageRequestQuality {
	#[serde(rename="standard")]
	Standard,
	#[serde(rename="hd")]
	Hd,
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
	#[serde(rename="auto")]
	Auto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageRequestResponseFormat {
	#[serde(rename="url")]
	Url,
	#[serde(rename="b64_json")]
	B64Json,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageRequestOutputFormat {
	#[serde(rename="png")]
	Png,
	#[serde(rename="jpeg")]
	Jpeg,
	#[serde(rename="webp")]
	Webp,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageRequestSize {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="1024x1024")]
	Size1024x1024,
	#[serde(rename="1536x1024")]
	Size1536x1024,
	#[serde(rename="1024x1536")]
	Size1024x1536,
	#[serde(rename="256x256")]
	Size256x256,
	#[serde(rename="512x512")]
	Size512x512,
	#[serde(rename="1792x1024")]
	Size1792x1024,
	#[serde(rename="1024x1792")]
	Size1024x1792,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageRequestModeration {
	#[serde(rename="low")]
	Low,
	#[serde(rename="auto")]
	Auto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageRequestBackground {
	#[serde(rename="transparent")]
	Transparent,
	#[serde(rename="opaque")]
	Opaque,
	#[serde(rename="auto")]
	Auto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageRequestStyle {
	#[serde(rename="vivid")]
	Vivid,
	#[serde(rename="natural")]
	Natural,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateImageRequest {
	/** A text description of the desired image(s). The maximum length is 32000 characters for `gpt-image-1`, 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`. */
	pub prompt: String,
	/** The model to use for image generation. One of `dall-e-2`, `dall-e-3`, or `gpt-image-1`. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateImageRequestModel>,
	/** The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<u64>,
	/** The quality of the image that will be generated. 

- `auto` (default value) will automatically select the best quality for the given model.
- `high`, `medium` and `low` are supported for `gpt-image-1`.
- `hd` and `standard` are supported for `dall-e-3`.
- `standard` is the only option for `dall-e-2`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quality: Option<CreateImageRequestQuality>,
	/** The format in which generated images with `dall-e-2` and `dall-e-3` are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter isn't supported for `gpt-image-1` which will always return base64-encoded images. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateImageRequestResponseFormat>,
	/** The format in which the generated images are returned. This parameter is only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_format: Option<CreateImageRequestOutputFormat>,
	/** The compression level (0-100%) for the generated images. This parameter is only supported for `gpt-image-1` with the `webp` or `jpeg` output formats, and defaults to 100. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_compression: Option<u64>,
	/** The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`, and one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<CreateImageRequestSize>,
	/** Control the content-moderation level for images generated by `gpt-image-1`. Must be either `low` for less restrictive filtering or `auto` (default value). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub moderation: Option<CreateImageRequestModeration>,
	/** Allows to set transparency for the background of the generated image(s). 
This parameter is only supported for `gpt-image-1`. Must be one of 
`transparent`, `opaque` or `auto` (default value). When `auto` is used, the 
model will automatically determine the best background for the image.

If `transparent`, the output format needs to support transparency, so it 
should be set to either `png` (default value) or `webp`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub background: Option<CreateImageRequestBackground>,
	/** The style of the generated images. This parameter is only supported for `dall-e-3`. Must be one of `vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub style: Option<CreateImageRequestStyle>,
	/** A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}

/** The model to use for image generation. Only `dall-e-2` is supported at this time. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateImageVariationRequestModel {
	String(String),
	#[serde(rename="dall-e-2")]
	DallE2,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageVariationRequestResponseFormat {
	#[serde(rename="url")]
	Url,
	#[serde(rename="b64_json")]
	B64Json,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateImageVariationRequestSize {
	#[serde(rename="256x256")]
	Size256x256,
	#[serde(rename="512x512")]
	Size512x512,
	#[serde(rename="1024x1024")]
	Size1024x1024,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateImageVariationRequest {
	/** The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square. */
	pub image: Vec<u8>,
	/** The model to use for image generation. Only `dall-e-2` is supported at this time. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateImageVariationRequestModel>,
	/** The number of images to generate. Must be between 1 and 10. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n: Option<u64>,
	/** The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateImageVariationRequestResponseFormat>,
	/** The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<CreateImageVariationRequestSize>,
	/** A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateMessageRequestRole {
	#[serde(rename="user")]
	User,
	#[serde(rename="assistant")]
	Assistant,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageRequestContentArray {
	MessageContentImageFileObject(MessageContentImageFileObject),
	MessageContentImageUrlObject(MessageContentImageUrlObject),
	MessageRequestContentTextObject(MessageRequestContentTextObject),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageRequestContent {
	/** The text contents of the message. */
	String(String),
	/** An array of content parts with a defined type, each can be of type `text` or images can be passed with `image_url` or `image_file`. Image types are only supported on [Vision-compatible models](/docs/models). */
	ArrayList(Vec<CreateMessageRequestContentArray>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageRequestAttachmentsTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearchTypeOnly(AssistantToolsFileSearchTypeOnly),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateMessageRequestAttachments {
	/** The ID of the file to attach to the message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/** The tools to add this file to. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<CreateMessageRequestAttachmentsTools>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateMessageRequest {
	/** The role of the entity that is creating the message. Allowed values include:
- `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.
- `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation. */
	pub role: CreateMessageRequestRole,
	pub content: CreateMessageRequestContent,
	/** A list of files attached to the message, and the tools they should be added to. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attachments: Option<Vec<CreateMessageRequestAttachments>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModelResponsePropertiesObject {
	/** An integer between 0 and 20 specifying the number of most likely tokens to
return at each token position, each with an associated log probability. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_logprobs: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModelResponseProperties {
	#[serde(flatten)]
	pub model_response_properties: ModelResponseProperties,
	#[serde(flatten)]
	pub object: CreateModelResponsePropertiesObject,
}

/** Contains either an image URL or a data URL for a base64 encoded image. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModerationRequestInputArrayImageUrl {
	/** Either a URL of the image or the base64 encoded image data. */
	pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateModerationRequestInputArray {
	/** An object describing an image to classify. */
	A {
		#[serde(rename="type")]
		r#type: String,
		image_url: CreateModerationRequestInputArrayImageUrl,
	},
	/** An object describing text to classify. */
	B {
		#[serde(rename="type")]
		r#type: String,
		text: String,
	},
}

/** Input (or inputs) to classify. Can be a single string, an array of strings, or
an array of multi-modal input objects similar to other models. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateModerationRequestInput {
	/** A string of text to classify for moderation. */
	String(String),
	/** An array of strings to classify for moderation. */
	ArrayString(Vec<String>),
	/** An array of multi-modal inputs to the moderation model. */
	ArrayList(Vec<CreateModerationRequestInputArray>),
}

/** The content moderation model you would like to use. Learn more in
[the moderation guide](/docs/guides/moderation), and learn about
available models [here](/docs/models#moderation). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateModerationRequestModel {
	String(String),
	#[serde(rename="omni-moderation-latest")]
	OmniModerationLatest,
	#[serde(rename="omni-moderation-2024-09-26")]
	OmniModeration20240926,
	#[serde(rename="text-moderation-latest")]
	TextModerationLatest,
	#[serde(rename="text-moderation-stable")]
	TextModerationStable,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModerationRequest {
	/** Input (or inputs) to classify. Can be a single string, an array of strings, or
an array of multi-modal input objects similar to other models. */
	pub input: CreateModerationRequestInput,
	/** The content moderation model you would like to use. Learn more in
[the moderation guide](/docs/guides/moderation), and learn about
available models [here](/docs/models#moderation). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateModerationRequestModel>,
}

/** A list of the categories, and whether they are flagged or not. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModerationResponseResultsCategories {
	/** Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste. Hateful content aimed at non-protected groups (e.g., chess players) is harassment. */
	pub hate: bool,
	/** Hateful content that also includes violence or serious harm towards the targeted group based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste. */
	#[serde(rename = "hate/threatening")]
	pub hate_threatening: bool,
	/** Content that expresses, incites, or promotes harassing language towards any target. */
	pub harassment: bool,
	/** Harassment content that also includes violence or serious harm towards any target. */
	#[serde(rename = "harassment/threatening")]
	pub harassment_threatening: bool,
	/** Content that includes instructions or advice that facilitate the planning or execution of wrongdoing, or that gives advice or instruction on how to commit illicit acts. For example, "how to shoplift" would fit this category. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub illicit: Option<bool>,
	/** Content that includes instructions or advice that facilitate the planning or execution of wrongdoing that also includes violence, or that gives advice or instruction on the procurement of any weapon. */
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "illicit/violent")]
	pub illicit_violent: Option<bool>,
	/** Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders. */
	#[serde(rename = "self-harm")]
	pub self_harm: bool,
	/** Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders. */
	#[serde(rename = "self-harm/intent")]
	pub self_harm_intent: bool,
	/** Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders, or that gives instructions or advice on how to commit such acts. */
	#[serde(rename = "self-harm/instructions")]
	pub self_harm_instructions: bool,
	/** Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness). */
	pub sexual: bool,
	/** Sexual content that includes an individual who is under 18 years old. */
	#[serde(rename = "sexual/minors")]
	pub sexual_minors: bool,
	/** Content that depicts death, violence, or physical injury. */
	pub violence: bool,
	/** Content that depicts death, violence, or physical injury in graphic detail. */
	#[serde(rename = "violence/graphic")]
	pub violence_graphic: bool,
}

/** A list of the categories along with their scores as predicted by model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModerationResponseResultsCategoryScores {
	/** The score for the category 'hate'. */
	pub hate: f32,
	/** The score for the category 'hate/threatening'. */
	#[serde(rename = "hate/threatening")]
	pub hate_threatening: f32,
	/** The score for the category 'harassment'. */
	pub harassment: f32,
	/** The score for the category 'harassment/threatening'. */
	#[serde(rename = "harassment/threatening")]
	pub harassment_threatening: f32,
	/** The score for the category 'illicit'. */
	pub illicit: f32,
	/** The score for the category 'illicit/violent'. */
	#[serde(rename = "illicit/violent")]
	pub illicit_violent: f32,
	/** The score for the category 'self-harm'. */
	#[serde(rename = "self-harm")]
	pub self_harm: f32,
	/** The score for the category 'self-harm/intent'. */
	#[serde(rename = "self-harm/intent")]
	pub self_harm_intent: f32,
	/** The score for the category 'self-harm/instructions'. */
	#[serde(rename = "self-harm/instructions")]
	pub self_harm_instructions: f32,
	/** The score for the category 'sexual'. */
	pub sexual: f32,
	/** The score for the category 'sexual/minors'. */
	#[serde(rename = "sexual/minors")]
	pub sexual_minors: f32,
	/** The score for the category 'violence'. */
	pub violence: f32,
	/** The score for the category 'violence/graphic'. */
	#[serde(rename = "violence/graphic")]
	pub violence_graphic: f32,
}

/** A list of the categories along with the input type(s) that the score applies to. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModerationResponseResultsCategoryAppliedInputTypes {
	/** The applied input type(s) for the category 'hate'. */
	pub hate: Vec<String>,
	/** The applied input type(s) for the category 'hate/threatening'. */
	#[serde(rename = "hate/threatening")]
	pub hate_threatening: Vec<String>,
	/** The applied input type(s) for the category 'harassment'. */
	pub harassment: Vec<String>,
	/** The applied input type(s) for the category 'harassment/threatening'. */
	#[serde(rename = "harassment/threatening")]
	pub harassment_threatening: Vec<String>,
	/** The applied input type(s) for the category 'illicit'. */
	pub illicit: Vec<String>,
	/** The applied input type(s) for the category 'illicit/violent'. */
	#[serde(rename = "illicit/violent")]
	pub illicit_violent: Vec<String>,
	/** The applied input type(s) for the category 'self-harm'. */
	#[serde(rename = "self-harm")]
	pub self_harm: Vec<String>,
	/** The applied input type(s) for the category 'self-harm/intent'. */
	#[serde(rename = "self-harm/intent")]
	pub self_harm_intent: Vec<String>,
	/** The applied input type(s) for the category 'self-harm/instructions'. */
	#[serde(rename = "self-harm/instructions")]
	pub self_harm_instructions: Vec<String>,
	/** The applied input type(s) for the category 'sexual'. */
	pub sexual: Vec<String>,
	/** The applied input type(s) for the category 'sexual/minors'. */
	#[serde(rename = "sexual/minors")]
	pub sexual_minors: Vec<String>,
	/** The applied input type(s) for the category 'violence'. */
	pub violence: Vec<String>,
	/** The applied input type(s) for the category 'violence/graphic'. */
	#[serde(rename = "violence/graphic")]
	pub violence_graphic: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModerationResponseResults {
	/** Whether any of the below categories are flagged. */
	pub flagged: bool,
	/** A list of the categories, and whether they are flagged or not. */
	pub categories: CreateModerationResponseResultsCategories,
	/** A list of the categories along with their scores as predicted by model. */
	pub category_scores: CreateModerationResponseResultsCategoryScores,
	/** A list of the categories along with the input type(s) that the score applies to. */
	pub category_applied_input_types: CreateModerationResponseResultsCategoryAppliedInputTypes,
}

/** Represents if a given text input is potentially harmful. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateModerationResponse {
	/** The unique identifier for the moderation request. */
	pub id: String,
	/** The model used to generate the moderation results. */
	pub model: String,
	/** A list of moderation objects. */
	pub results: Vec<CreateModerationResponseResults>,
}

/** Text, image, or file inputs to the model, used to generate a response.

Learn more:
- [Text inputs and outputs](/docs/guides/text)
- [Image inputs](/docs/guides/images)
- [File inputs](/docs/guides/pdf-files)
- [Conversation state](/docs/guides/conversation-state)
- [Function calling](/docs/guides/function-calling) */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateResponseObjectInput {
	/** A text input to the model, equivalent to a text input with the
`user` role. */
	String(String),
	/** A list of one or many input items to the model, containing
different content types. */
	ArrayList(Vec<InputItem>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateResponseObject {
	/** Text, image, or file inputs to the model, used to generate a response.

Learn more:
- [Text inputs and outputs](/docs/guides/text)
- [Image inputs](/docs/guides/images)
- [File inputs](/docs/guides/pdf-files)
- [Conversation state](/docs/guides/conversation-state)
- [Function calling](/docs/guides/function-calling) */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input: Option<CreateResponseObjectInput>,
	/** Specify additional output data to include in the model response. Currently
supported values are:
- `code_interpreter_call.outputs`: Includes the outputs of python code execution
  in code interpreter tool call items.
- `computer_call_output.output.image_url`: Include image urls from the computer call output.
- `file_search_call.results`: Include the search results of
  the file search tool call.
- `message.input_image.image_url`: Include image urls from the input message.
- `message.output_text.logprobs`: Include logprobs with assistant messages.
- `reasoning.encrypted_content`: Includes an encrypted version of reasoning
  tokens in reasoning item outputs. This enables reasoning items to be used in
  multi-turn conversations when using the Responses API statelessly (like
  when the `store` parameter is set to `false`, or when an organization is
  enrolled in the zero data retention program). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<Vec<Includable>>,
	/** Whether to allow the model to run tool calls in parallel. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<bool>,
	/** Whether to store the generated model response for later retrieval via
API. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub store: Option<bool>,
	/** A system (or developer) message inserted into the model's context.

When using along with `previous_response_id`, the instructions from a previous
response will not be carried over to the next response. This makes it simple
to swap out system (or developer) messages in new responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** If set to true, the model response data will be streamed to the client
as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).
See the [Streaming section below](/docs/api-reference/responses-streaming)
for more information. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateResponse {
	#[serde(flatten)]
	pub create_model_response_properties: CreateModelResponseProperties,
	#[serde(flatten)]
	pub response_properties: ResponseProperties,
	#[serde(flatten)]
	pub object: CreateResponseObject,
}

/** The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRunRequestModel {
	String(String),
	AssistantSupportedModels(AssistantSupportedModels),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateRunRequestTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRunRequestTruncationStrategy {
	#[serde(flatten)]
	pub truncation_object: TruncationObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRunRequestToolChoice {
	#[serde(flatten)]
	pub assistants_api_tool_choice_option: AssistantsApiToolChoiceOption,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRunRequest {
	/** The ID of the [assistant](/docs/api-reference/assistants) to use to execute this run. */
	pub assistant_id: String,
	/** The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateRunRequestModel>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	/** Overrides the [instructions](/docs/api-reference/assistants/createAssistant) of the assistant. This is useful for modifying the behavior on a per-run basis. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** Appends additional instructions at the end of the instructions for the run. This is useful for modifying the behavior on a per-run basis without overriding other instructions. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub additional_instructions: Option<String>,
	/** Adds additional messages to the thread before creating the run. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub additional_messages: Option<Vec<CreateMessageRequest>>,
	/** Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<CreateRunRequestTools>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or temperature but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	/** If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	/** The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_prompt_tokens: Option<u64>,
	/** The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub truncation_strategy: Option<CreateRunRequestTruncationStrategy>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<CreateRunRequestToolChoice>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<ParallelToolCalls>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
}

/** One of the available [TTS models](/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSpeechRequestModel {
	String(String),
	#[serde(rename="tts-1")]
	Tts1,
	#[serde(rename="tts-1-hd")]
	Tts1Hd,
	#[serde(rename="gpt-4o-mini-tts")]
	Gpt4oMiniTts,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateSpeechRequestResponseFormat {
	#[serde(rename="mp3")]
	Mp3,
	#[serde(rename="opus")]
	Opus,
	#[serde(rename="aac")]
	Aac,
	#[serde(rename="flac")]
	Flac,
	#[serde(rename="wav")]
	Wav,
	#[serde(rename="pcm")]
	Pcm,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateSpeechRequestStreamFormat {
	#[serde(rename="sse")]
	Sse,
	#[serde(rename="audio")]
	Audio,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSpeechRequest {
	/** One of the available [TTS models](/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`. */
	pub model: CreateSpeechRequestModel,
	/** The text to generate audio for. The maximum length is 4096 characters. */
	pub input: String,
	/** Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, and `verse`. Previews of the voices are available in the [Text to speech guide](/docs/guides/text-to-speech#voice-options). */
	pub voice: VoiceIdsShared,
	/** The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateSpeechRequestResponseFormat>,
	/** The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<f32>,
	/** The format to stream the audio in. Supported formats are `sse` and `audio`. `sse` is not supported for `tts-1` or `tts-1-hd`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream_format: Option<CreateSpeechRequestStreamFormat>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateSpeechResponseStreamEvent {
	SpeechAudioDeltaEvent(SpeechAudioDeltaEvent),
	SpeechAudioDoneEvent(SpeechAudioDoneEvent),
}

/** The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadAndRunRequestModel {
	String(String),
	#[serde(rename="gpt-4.1")]
	Gpt41,
	#[serde(rename="gpt-4.1-mini")]
	Gpt41Mini,
	#[serde(rename="gpt-4.1-nano")]
	Gpt41Nano,
	#[serde(rename="gpt-4.1-2025-04-14")]
	Gpt4120250414,
	#[serde(rename="gpt-4.1-mini-2025-04-14")]
	Gpt41Mini20250414,
	#[serde(rename="gpt-4.1-nano-2025-04-14")]
	Gpt41Nano20250414,
	#[serde(rename="gpt-4o")]
	Gpt4o,
	#[serde(rename="gpt-4o-2024-11-20")]
	Gpt4o20241120,
	#[serde(rename="gpt-4o-2024-08-06")]
	Gpt4o20240806,
	#[serde(rename="gpt-4o-2024-05-13")]
	Gpt4o20240513,
	#[serde(rename="gpt-4o-mini")]
	Gpt4oMini,
	#[serde(rename="gpt-4o-mini-2024-07-18")]
	Gpt4oMini20240718,
	#[serde(rename="gpt-4.5-preview")]
	Gpt45Preview,
	#[serde(rename="gpt-4.5-preview-2025-02-27")]
	Gpt45Preview20250227,
	#[serde(rename="gpt-4-turbo")]
	Gpt4Turbo,
	#[serde(rename="gpt-4-turbo-2024-04-09")]
	Gpt4Turbo20240409,
	#[serde(rename="gpt-4-0125-preview")]
	Gpt40125Preview,
	#[serde(rename="gpt-4-turbo-preview")]
	Gpt4TurboPreview,
	#[serde(rename="gpt-4-1106-preview")]
	Gpt41106Preview,
	#[serde(rename="gpt-4-vision-preview")]
	Gpt4VisionPreview,
	#[serde(rename="gpt-4")]
	Gpt4,
	#[serde(rename="gpt-4-0314")]
	Gpt40314,
	#[serde(rename="gpt-4-0613")]
	Gpt40613,
	#[serde(rename="gpt-4-32k")]
	Gpt432k,
	#[serde(rename="gpt-4-32k-0314")]
	Gpt432k0314,
	#[serde(rename="gpt-4-32k-0613")]
	Gpt432k0613,
	#[serde(rename="gpt-3.5-turbo")]
	Gpt35Turbo,
	#[serde(rename="gpt-3.5-turbo-16k")]
	Gpt35Turbo16k,
	#[serde(rename="gpt-3.5-turbo-0613")]
	Gpt35Turbo0613,
	#[serde(rename="gpt-3.5-turbo-1106")]
	Gpt35Turbo1106,
	#[serde(rename="gpt-3.5-turbo-0125")]
	Gpt35Turbo0125,
	#[serde(rename="gpt-3.5-turbo-16k-0613")]
	Gpt35Turbo16k0613,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateThreadAndRunRequestTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadAndRunRequestToolResourcesCodeInterpreter {
	/** A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadAndRunRequestToolResourcesFileSearch {
	/** The ID of the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
}

/** A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadAndRunRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<CreateThreadAndRunRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<CreateThreadAndRunRequestToolResourcesFileSearch>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadAndRunRequestTruncationStrategy {
	#[serde(flatten)]
	pub truncation_object: TruncationObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadAndRunRequestToolChoice {
	#[serde(flatten)]
	pub assistants_api_tool_choice_option: AssistantsApiToolChoiceOption,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadAndRunRequest {
	/** The ID of the [assistant](/docs/api-reference/assistants) to use to execute this run. */
	pub assistant_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub thread: Option<CreateThreadRequest>,
	/** The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<CreateThreadAndRunRequestModel>,
	/** Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<CreateThreadAndRunRequestTools>>,
	/** A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<CreateThreadAndRunRequestToolResources>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or temperature but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	/** If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	/** The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_prompt_tokens: Option<u64>,
	/** The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub truncation_strategy: Option<CreateThreadAndRunRequestTruncationStrategy>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<CreateThreadAndRunRequestToolChoice>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<ParallelToolCalls>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadRequestToolResourcesCodeInterpreter {
	/** A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

	/** The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadRequestToolResourcesFileSearchVectorStoresChunkingStrategy(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadRequestToolResourcesFileSearchVectorStores {
	/** A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
	/** The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<CreateThreadRequestToolResourcesFileSearchVectorStoresChunkingStrategy>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadRequestToolResourcesFileSearch {
	/** The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
	/** A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this thread. There can be a maximum of 1 vector store attached to the thread. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_stores: Option<Vec<CreateThreadRequestToolResourcesFileSearchVectorStores>>,
}

/** A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<CreateThreadRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<CreateThreadRequestToolResourcesFileSearch>,
}

/** Options to create a new thread. If no thread is provided when running a 
request, an empty thread will be created. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateThreadRequest {
	/** A list of [messages](/docs/api-reference/messages) to start the thread with. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub messages: Option<Vec<CreateMessageRequest>>,
	/** A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<CreateThreadRequestToolResources>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

/** ID of the model to use. The options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1` (which is powered by our open source Whisper V2 model). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranscriptionRequestModel {
	String(String),
	#[serde(rename="whisper-1")]
	Whisper1,
	#[serde(rename="gpt-4o-transcribe")]
	Gpt4oTranscribe,
	#[serde(rename="gpt-4o-mini-transcribe")]
	Gpt4oMiniTranscribe,
}

/** Controls how the audio is cut into chunks. When set to `"auto"`, the server first normalizes loudness and then uses voice activity detection (VAD) to choose boundaries. `server_vad` object can be provided to tweak VAD detection parameters manually. If unset, the audio is transcribed as a single block. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranscriptionRequestChunkingStrategy {
	/** Automatically set chunking parameters based on the audio. Must be set to `"auto"`. */
	#[serde(rename="auto")]
	Auto,
	VadConfig(VadConfig),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTranscriptionRequest {
	/** The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm. */
	pub file: crate::multipart::File,
	/** ID of the model to use. The options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1` (which is powered by our open source Whisper V2 model). */
	pub model: CreateTranscriptionRequestModel,
	/** The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/** An optional text to guide the model's style or continue a previous audio segment. The [prompt](/docs/guides/speech-to-text#prompting) should match the audio language. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AudioResponseFormat>,
	/** The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	#[serde(rename="include[]")]
	/** Additional information to include in the transcription response. 
`logprobs` will return the log probabilities of the tokens in the 
response to understand the model's confidence in the transcription. 
`logprobs` only works with response_format set to `json` and only with 
the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<Vec<TranscriptionInclude>>,
	#[serde(rename="timestamp_granularities[]")]
	/** The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamp_granularities: Option<Vec<String>>,
	/** If set to true, the model response data will be streamed to the client
as it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). 
See the [Streaming section of the Speech-to-Text guide](/docs/guides/speech-to-text?lang=curl#streaming-transcriptions)
for more information.

Note: Streaming is not supported for the `whisper-1` model and will be ignored. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
	/** Controls how the audio is cut into chunks. When set to `"auto"`, the server first normalizes loudness and then uses voice activity detection (VAD) to choose boundaries. `server_vad` object can be provided to tweak VAD detection parameters manually. If unset, the audio is transcribed as a single block. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<CreateTranscriptionRequestChunkingStrategy>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTranscriptionResponseJsonLogprobs {
	/** The token in the transcription. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
	/** The log probability of the token. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprob: Option<f32>,
	/** The bytes of the token. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bytes: Option<Vec<f32>>,
}

	/** Token usage statistics for the request. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTranscriptionResponseJsonUsage(pub String);

/** Represents a transcription response returned by model, based on the provided input. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTranscriptionResponseJson {
	/** The transcribed text. */
	pub text: String,
	/** The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<Vec<CreateTranscriptionResponseJsonLogprobs>>,
	/** Token usage statistics for the request. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<CreateTranscriptionResponseJsonUsage>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranscriptionResponseStreamEvent {
	TranscriptTextDeltaEvent(TranscriptTextDeltaEvent),
	TranscriptTextDoneEvent(TranscriptTextDoneEvent),
}

/** Represents a verbose json transcription response returned by model, based on the provided input. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTranscriptionResponseVerboseJson {
	/** The language of the input audio. */
	pub language: String,
	/** The duration of the input audio. */
	pub duration: f32,
	/** The transcribed text. */
	pub text: String,
	/** Extracted words and their corresponding timestamps. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub words: Option<Vec<TranscriptionWord>>,
	/** Segments of the transcribed text and their corresponding details. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub segments: Option<Vec<TranscriptionSegment>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<TranscriptTextUsageDuration>,
}

/** ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranslationRequestModel {
	String(String),
	#[serde(rename="whisper-1")]
	Whisper1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateTranslationRequestResponseFormat {
	#[serde(rename="json")]
	Json,
	#[serde(rename="text")]
	Text,
	#[serde(rename="srt")]
	Srt,
	#[serde(rename="verbose_json")]
	VerboseJson,
	#[serde(rename="vtt")]
	Vtt,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTranslationRequest {
	/** The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm. */
	pub file: crate::multipart::File,
	/** ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available. */
	pub model: CreateTranslationRequestModel,
	/** An optional text to guide the model's style or continue a previous audio segment. The [prompt](/docs/guides/speech-to-text#prompting) should be in English. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
	/** The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<CreateTranslationRequestResponseFormat>,
	/** The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTranslationResponseJson {
	pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTranslationResponseVerboseJson {
	/** The language of the output translation (always `english`). */
	pub language: String,
	/** The duration of the input audio. */
	pub duration: f32,
	/** The translated text. */
	pub text: String,
	/** Segments of the translated text and their corresponding details. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub segments: Option<Vec<TranscriptionSegment>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum CreateUploadRequestPurpose {
	#[serde(rename="assistants")]
	Assistants,
	#[serde(rename="batch")]
	Batch,
	#[serde(rename="fine-tune")]
	FineTune,
	#[serde(rename="vision")]
	Vision,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateUploadRequest {
	/** The name of the file to upload. */
	pub filename: String,
	/** The intended purpose of the uploaded file.

See the [documentation on File purposes](/docs/api-reference/files/create#files-create-purpose). */
	pub purpose: CreateUploadRequestPurpose,
	/** The number of bytes in the file you are uploading. */
	pub bytes: u64,
	/** The MIME type of the file.

This must fall within the supported MIME types for your file purpose. See the supported MIME types for assistants and vision. */
	pub mime_type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVectorStoreFileBatchRequest {
	/** A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files. */
	pub file_ids: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<ChunkingStrategyRequestParam>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<VectorStoreFileAttributes>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVectorStoreFileRequest {
	/** A [File](/docs/api-reference/files) ID that the vector store should use. Useful for tools like `file_search` that can access files. */
	pub file_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<ChunkingStrategyRequestParam>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<VectorStoreFileAttributes>,
}

	/** The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVectorStoreRequestChunkingStrategy(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateVectorStoreRequest {
	/** A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
	/** The name of the vector store. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_after: Option<VectorStoreExpirationAfter>,
	/** The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<CreateVectorStoreRequestChunkingStrategy>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteAssistantResponseObject {
	#[serde(rename="assistant.deleted")]
	AssistantDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteAssistantResponse {
	pub id: String,
	pub deleted: bool,
	pub object: DeleteAssistantResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteCertificateResponseObject {
	#[serde(rename="certificate.deleted")]
	CertificateDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteCertificateResponse {
	/** The object type, must be `certificate.deleted`. */
	pub object: DeleteCertificateResponseObject,
	/** The ID of the certificate that was deleted. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteFileResponseObject {
	#[serde(rename="file")]
	File,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteFileResponse {
	pub id: String,
	pub object: DeleteFileResponseObject,
	pub deleted: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteFineTuningCheckpointPermissionResponseObject {
	#[serde(rename="checkpoint.permission")]
	CheckpointPermission,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteFineTuningCheckpointPermissionResponse {
	/** The ID of the fine-tuned model checkpoint permission that was deleted. */
	pub id: String,
	/** The object type, which is always "checkpoint.permission". */
	pub object: DeleteFineTuningCheckpointPermissionResponseObject,
	/** Whether the fine-tuned model checkpoint permission was successfully deleted. */
	pub deleted: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteMessageResponseObject {
	#[serde(rename="thread.message.deleted")]
	ThreadMessageDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteMessageResponse {
	pub id: String,
	pub deleted: bool,
	pub object: DeleteMessageResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteModelResponse {
	pub id: String,
	pub deleted: bool,
	pub object: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteThreadResponseObject {
	#[serde(rename="thread.deleted")]
	ThreadDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteThreadResponse {
	pub id: String,
	pub deleted: bool,
	pub object: DeleteThreadResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteVectorStoreFileResponseObject {
	#[serde(rename="vector_store.file.deleted")]
	VectorStoreFileDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteVectorStoreFileResponse {
	pub id: String,
	pub deleted: bool,
	pub object: DeleteVectorStoreFileResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DeleteVectorStoreResponseObject {
	#[serde(rename="vector_store.deleted")]
	VectorStoreDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteVectorStoreResponse {
	pub id: String,
	pub deleted: bool,
	pub object: DeleteVectorStoreResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DoneEventEvent {
	#[serde(rename="done")]
	Done,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DoneEventData {
	#[serde(rename="[DONE]")]
	DONE,
}

/** Occurs when a stream ends. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DoneEvent {
	pub event: DoneEventEvent,
	pub data: DoneEventData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DoubleClickType {
	#[serde(rename="double_click")]
	DoubleClick,
}

/** A double click action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DoubleClick {
	#[serde(rename="type")]
	/** Specifies the event type. For a double click action, this property is 
always set to `double_click`. */
	pub r#type: DoubleClickType,
	/** The x-coordinate where the double click occurred. */
	pub x: u64,
	/** The y-coordinate where the double click occurred. */
	pub y: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum DragType {
	#[serde(rename="drag")]
	Drag,
}

/** A drag action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Drag {
	#[serde(rename="type")]
	/** Specifies the event type. For a drag action, this property is 
always set to `drag`. */
	pub r#type: DragType,
	/** An array of coordinates representing the path of the drag action. Coordinates will appear as an array
of objects, eg
***
[
  { x: 100, y: 200 },
  { x: 200, y: 300 }
]
*** */
	pub path: Vec<Coordinate>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EasyInputMessageRole {
	#[serde(rename="user")]
	User,
	#[serde(rename="assistant")]
	Assistant,
	#[serde(rename="system")]
	System,
	#[serde(rename="developer")]
	Developer,
}

/** Text, image, or audio input to the model, used to generate a response.
Can also contain previous assistant responses. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EasyInputMessageContent {
	/** A text input to the model. */
	String(String),
	InputMessageContentList(InputMessageContentList),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EasyInputMessageType {
	#[serde(rename="message")]
	Message,
}

/** A message input to the model with a role indicating instruction following
hierarchy. Instructions given with the `developer` or `system` role take
precedence over instructions given with the `user` role. Messages with the
`assistant` role are presumed to have been generated by the model in previous
interactions. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EasyInputMessage {
	/** The role of the message input. One of `user`, `assistant`, `system`, or
`developer`. */
	pub role: EasyInputMessageRole,
	/** Text, image, or audio input to the model, used to generate a response.
Can also contain previous assistant responses. */
	pub content: EasyInputMessageContent,
	#[serde(rename="type")]
	/** The type of the message input. Always `message`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<EasyInputMessageType>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EmbeddingObject {
	#[serde(rename="embedding")]
	Embedding,
}

/** Represents an embedding vector returned by embedding endpoint. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Embedding {
	/** The index of the embedding in the list of embeddings. */
	pub index: u64,
	/** The embedding vector, which is a list of floats. The length of vector depends on the model as listed in the [embedding guide](/docs/guides/embeddings). */
	pub embedding: Vec<f32>,
	/** The object type, which is always "embedding". */
	pub object: EmbeddingObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Error {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	pub message: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub param: Option<String>,
	#[serde(rename="type")]
	pub r#type: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ErrorEventEvent {
	#[serde(rename="error")]
	Error,
}

/** Occurs when an [error](/docs/guides/error-codes#api-errors) occurs. This can happen due to an internal server error or a timeout. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorEvent {
	pub event: ErrorEventEvent,
	pub data: Error,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
	pub error: Error,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalObject {
	#[serde(rename="eval")]
	Eval,
}

	/** Configuration of data sources used in runs of the evaluation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalDataSourceConfig(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvalTestingCriteria {
	EvalGraderLabelModel(EvalGraderLabelModel),
	EvalGraderStringCheck(EvalGraderStringCheck),
	EvalGraderTextSimilarity(EvalGraderTextSimilarity),
	EvalGraderPython(EvalGraderPython),
	EvalGraderScoreModel(EvalGraderScoreModel),
}

/** An Eval object with a data source config and testing criteria.
An Eval represents a task to be done for your LLM integration.
Like:
 - Improve the quality of my chatbot
 - See how well my chatbot handles customer support
 - Check if o4-mini is better at my usecase than gpt-4o */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Eval {
	/** The object type. */
	pub object: EvalObject,
	/** Unique identifier for the evaluation. */
	pub id: String,
	/** The name of the evaluation. */
	pub name: String,
	/** Configuration of data sources used in runs of the evaluation. */
	pub data_source_config: EvalDataSourceConfig,
	/** A list of testing criteria. */
	pub testing_criteria: Vec<EvalTestingCriteria>,
	/** The Unix timestamp (in seconds) for when the eval was created. */
	pub created_at: u64,
	pub metadata: Metadata,
}

/** An object representing an error response from the Eval API. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalApiError {
	/** The error code. */
	pub code: String,
	/** The error message. */
	pub message: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalCustomDataSourceConfigType {
	#[serde(rename="custom")]
	Custom,
}

	/** The json schema for the run data source items.
Learn how to build JSON schemas [here](https://json-schema.org/). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalCustomDataSourceConfigSchema(pub String);

/** A CustomDataSourceConfig which specifies the schema of your `item` and optionally `sample` namespaces.
The response schema defines the shape of the data that will be:
- Used to define your testing criteria and
- What data is required when creating a run */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalCustomDataSourceConfig {
	#[serde(rename="type")]
	/** The type of data source. Always `custom`. */
	pub r#type: EvalCustomDataSourceConfigType,
	/** The json schema for the run data source items.
Learn how to build JSON schemas [here](https://json-schema.org/). */
	pub schema: EvalCustomDataSourceConfigSchema,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalGraderLabelModel {
	#[serde(flatten)]
	pub grader_label_model: GraderLabelModel,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalGraderPythonObject {
	/** The threshold for the score. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pass_threshold: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalGraderPython {
	#[serde(flatten)]
	pub grader_python: GraderPython,
	#[serde(flatten)]
	pub object: EvalGraderPythonObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalGraderScoreModelObject {
	/** The threshold for the score. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub pass_threshold: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalGraderScoreModel {
	#[serde(flatten)]
	pub grader_score_model: GraderScoreModel,
	#[serde(flatten)]
	pub object: EvalGraderScoreModelObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalGraderStringCheck {
	#[serde(flatten)]
	pub grader_string_check: GraderStringCheck,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalGraderTextSimilarityObject {
	/** The threshold for the score. */
	pub pass_threshold: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalGraderTextSimilarity {
	#[serde(flatten)]
	pub grader_text_similarity: GraderTextSimilarity,
	#[serde(flatten)]
	pub object: EvalGraderTextSimilarityObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalItemRole {
	#[serde(rename="user")]
	User,
	#[serde(rename="assistant")]
	Assistant,
	#[serde(rename="system")]
	System,
	#[serde(rename="developer")]
	Developer,
}

/** Text inputs to the model - can contain template strings. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvalItemContent {
	/** A text input to the model. */
	String(String),
	InputTextContent(InputTextContent),
	/** A text output from the model. */
	Outputtext {
		#[serde(rename="type")]
		r#type: String,
		text: String,
	},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalItemType {
	#[serde(rename="message")]
	Message,
}

/** A message input to the model with a role indicating instruction following
hierarchy. Instructions given with the `developer` or `system` role take
precedence over instructions given with the `user` role. Messages with the
`assistant` role are presumed to have been generated by the model in previous
interactions. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalItem {
	/** The role of the message input. One of `user`, `assistant`, `system`, or
`developer`. */
	pub role: EvalItemRole,
	/** Text inputs to the model - can contain template strings. */
	pub content: EvalItemContent,
	#[serde(rename="type")]
	/** The type of the message input. Always `message`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<EvalItemType>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalJsonlFileContentSourceType {
	#[serde(rename="file_content")]
	FileContent,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalJsonlFileContentSourceContentItem(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalJsonlFileContentSourceContentSample(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalJsonlFileContentSourceContent {
	pub item: EvalJsonlFileContentSourceContentItem,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sample: Option<EvalJsonlFileContentSourceContentSample>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalJsonlFileContentSource {
	#[serde(rename="type")]
	/** The type of jsonl source. Always `file_content`. */
	pub r#type: EvalJsonlFileContentSourceType,
	/** The content of the jsonl file. */
	pub content: Vec<EvalJsonlFileContentSourceContent>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalJsonlFileIdSourceType {
	#[serde(rename="file_id")]
	FileId,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalJsonlFileIdSource {
	#[serde(rename="type")]
	/** The type of jsonl source. Always `file_id`. */
	pub r#type: EvalJsonlFileIdSourceType,
	/** The identifier of the file. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalListObject {
	#[serde(rename="list")]
	List,
}

/** An object representing a list of evals. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalList {
	/** The type of this object. It is always set to "list". */
	pub object: EvalListObject,
	/** An array of eval objects. */
	pub data: Vec<Eval>,
	/** The identifier of the first eval in the data array. */
	pub first_id: String,
	/** The identifier of the last eval in the data array. */
	pub last_id: String,
	/** Indicates whether there are more evals available. */
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalLogsDataSourceConfigType {
	#[serde(rename="logs")]
	Logs,
}

	/** The json schema for the run data source items.
Learn how to build JSON schemas [here](https://json-schema.org/). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalLogsDataSourceConfigSchema(pub String);

/** A LogsDataSourceConfig which specifies the metadata property of your logs query.
This is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc.
The schema returned by this data source config is used to defined what variables are available in your evals.
`item` and `sample` are both defined when using this data source config. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalLogsDataSourceConfig {
	#[serde(rename="type")]
	/** The type of data source. Always `logs`. */
	pub r#type: EvalLogsDataSourceConfigType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** The json schema for the run data source items.
Learn how to build JSON schemas [here](https://json-schema.org/). */
	pub schema: EvalLogsDataSourceConfigSchema,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalResponsesSourceType {
	#[serde(rename="responses")]
	Responses,
}

	/** Metadata filter for the responses. This is a query parameter used to select responses. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalResponsesSourceMetadata(pub String);

/** A EvalResponsesSource object describing a run data source configuration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalResponsesSource {
	#[serde(rename="type")]
	/** The type of run data source. Always `responses`. */
	pub r#type: EvalResponsesSourceType,
	/** Metadata filter for the responses. This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<EvalResponsesSourceMetadata>,
	/** The name of the model to find responses for. This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/** Optional string to search the 'instructions' field. This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions_search: Option<String>,
	/** Only include items created after this timestamp (inclusive). This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_after: Option<u64>,
	/** Only include items created before this timestamp (inclusive). This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_before: Option<u64>,
	/** Optional reasoning effort parameter. This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	/** Sampling temperature. This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** Nucleus sampling parameter. This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	/** List of user identifiers. This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub users: Option<Vec<String>>,
	/** List of tool names. This is a query parameter used to select responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalRunObject {
	#[serde(rename="eval.run")]
	EvalRun,
}

/** Counters summarizing the outcomes of the evaluation run. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunResultCounts {
	/** Total number of executed output items. */
	pub total: u64,
	/** Number of output items that resulted in an error. */
	pub errored: u64,
	/** Number of output items that failed to pass the evaluation. */
	pub failed: u64,
	/** Number of output items that passed the evaluation. */
	pub passed: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunPerModelUsage {
	/** The name of the model. */
	pub model_name: String,
	/** The number of invocations. */
	pub invocation_count: u64,
	/** The number of prompt tokens used. */
	pub prompt_tokens: u64,
	/** The number of completion tokens generated. */
	pub completion_tokens: u64,
	/** The total number of tokens used. */
	pub total_tokens: u64,
	/** The number of tokens retrieved from cache. */
	pub cached_tokens: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunPerTestingCriteriaResults {
	/** A description of the testing criteria. */
	pub testing_criteria: String,
	/** Number of tests passed for this criteria. */
	pub passed: u64,
	/** Number of tests failed for this criteria. */
	pub failed: u64,
}

	/** Information about the run's data source. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunDataSource(pub String);

/** A schema representing an evaluation run. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRun {
	/** The type of the object. Always "eval.run". */
	pub object: EvalRunObject,
	/** Unique identifier for the evaluation run. */
	pub id: String,
	/** The identifier of the associated evaluation. */
	pub eval_id: String,
	/** The status of the evaluation run. */
	pub status: String,
	/** The model that is evaluated, if applicable. */
	pub model: String,
	/** The name of the evaluation run. */
	pub name: String,
	/** Unix timestamp (in seconds) when the evaluation run was created. */
	pub created_at: u64,
	/** The URL to the rendered evaluation run report on the UI dashboard. */
	pub report_url: String,
	/** Counters summarizing the outcomes of the evaluation run. */
	pub result_counts: EvalRunResultCounts,
	/** Usage statistics for each model during the evaluation run. */
	pub per_model_usage: Vec<EvalRunPerModelUsage>,
	/** Results per testing criteria applied during the evaluation run. */
	pub per_testing_criteria_results: Vec<EvalRunPerTestingCriteriaResults>,
	/** Information about the run's data source. */
	pub data_source: EvalRunDataSource,
	pub metadata: Metadata,
	pub error: EvalApiError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalRunListObject {
	#[serde(rename="list")]
	List,
}

/** An object representing a list of runs for an evaluation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunList {
	/** The type of this object. It is always set to "list". */
	pub object: EvalRunListObject,
	/** An array of eval run objects. */
	pub data: Vec<EvalRun>,
	/** The identifier of the first eval run in the data array. */
	pub first_id: String,
	/** The identifier of the last eval run in the data array. */
	pub last_id: String,
	/** Indicates whether there are more evals available. */
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalRunOutputItemObject {
	#[serde(rename="eval.run.output_item")]
	EvalRunOutputItem,
}

	/** Details of the input data source item. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunOutputItemDatasourceItem(pub String);

	/** A result object. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunOutputItemResults(pub String);

/** An input message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunOutputItemSampleInput {
	/** The role of the message sender (e.g., system, user, developer). */
	pub role: String,
	/** The content of the message. */
	pub content: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunOutputItemSampleOutput {
	/** The role of the message (e.g. "system", "assistant", "user"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<String>,
	/** The content of the message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<String>,
}

/** Token usage details for the sample. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunOutputItemSampleUsage {
	/** The total number of tokens used. */
	pub total_tokens: u64,
	/** The number of completion tokens generated. */
	pub completion_tokens: u64,
	/** The number of prompt tokens used. */
	pub prompt_tokens: u64,
	/** The number of tokens retrieved from cache. */
	pub cached_tokens: u64,
}

/** A sample containing the input and output of the evaluation run. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunOutputItemSample {
	/** An array of input messages. */
	pub input: Vec<EvalRunOutputItemSampleInput>,
	/** An array of output messages. */
	pub output: Vec<EvalRunOutputItemSampleOutput>,
	/** The reason why the sample generation was finished. */
	pub finish_reason: String,
	/** The model used for generating the sample. */
	pub model: String,
	/** Token usage details for the sample. */
	pub usage: EvalRunOutputItemSampleUsage,
	pub error: EvalApiError,
	/** The sampling temperature used. */
	pub temperature: f32,
	/** The maximum number of tokens allowed for completion. */
	pub max_completion_tokens: u64,
	/** The top_p value used for sampling. */
	pub top_p: f32,
	/** The seed used for generating the sample. */
	pub seed: u64,
}

/** A schema representing an evaluation run output item. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunOutputItem {
	/** The type of the object. Always "eval.run.output_item". */
	pub object: EvalRunOutputItemObject,
	/** Unique identifier for the evaluation run output item. */
	pub id: String,
	/** The identifier of the evaluation run associated with this output item. */
	pub run_id: String,
	/** The identifier of the evaluation group. */
	pub eval_id: String,
	/** Unix timestamp (in seconds) when the evaluation run was created. */
	pub created_at: u64,
	/** The status of the evaluation run. */
	pub status: String,
	/** The identifier for the data source item. */
	pub datasource_item_id: u64,
	/** Details of the input data source item. */
	pub datasource_item: EvalRunOutputItemDatasourceItem,
	/** A list of results from the evaluation run. */
	pub results: Vec<EvalRunOutputItemResults>,
	/** A sample containing the input and output of the evaluation run. */
	pub sample: EvalRunOutputItemSample,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalRunOutputItemListObject {
	#[serde(rename="list")]
	List,
}

/** An object representing a list of output items for an evaluation run. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalRunOutputItemList {
	/** The type of this object. It is always set to "list". */
	pub object: EvalRunOutputItemListObject,
	/** An array of eval run output item objects. */
	pub data: Vec<EvalRunOutputItem>,
	/** The identifier of the first eval run output item in the data array. */
	pub first_id: String,
	/** The identifier of the last eval run output item in the data array. */
	pub last_id: String,
	/** Indicates whether there are more eval run output items available. */
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalStoredCompletionsDataSourceConfigType {
	#[serde(rename="stored_completions")]
	StoredCompletions,
}

	/** The json schema for the run data source items.
Learn how to build JSON schemas [here](https://json-schema.org/). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalStoredCompletionsDataSourceConfigSchema(pub String);

/** Deprecated in favor of LogsDataSourceConfig. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalStoredCompletionsDataSourceConfig {
	#[serde(rename="type")]
	/** The type of data source. Always `stored_completions`. */
	pub r#type: EvalStoredCompletionsDataSourceConfigType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** The json schema for the run data source items.
Learn how to build JSON schemas [here](https://json-schema.org/). */
	pub schema: EvalStoredCompletionsDataSourceConfigSchema,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EvalStoredCompletionsSourceType {
	#[serde(rename="stored_completions")]
	StoredCompletions,
}

/** A StoredCompletionsRunDataSource configuration describing a set of filters */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct EvalStoredCompletionsSource {
	#[serde(rename="type")]
	/** The type of source. Always `stored_completions`. */
	pub r#type: EvalStoredCompletionsSourceType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** An optional model to filter by (e.g., 'gpt-4o'). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/** An optional Unix timestamp to filter items created after this time. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_after: Option<u64>,
	/** An optional Unix timestamp to filter items created before this time. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub created_before: Option<u64>,
	/** An optional maximum number of items to return. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FilePathType {
	#[serde(rename="file_path")]
	FilePath,
}

/** A path to a file. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FilePath {
	#[serde(rename="type")]
	/** The type of the file path. Always `file_path`. */
	pub r#type: FilePathType,
	/** The ID of the file. */
	pub file_id: String,
	/** The index of the file in the list of files. */
	pub index: u64,
}

/** The ranker to use for the file search. If not specified will use the `auto` ranker. */
pub type FileSearchRanker = String;

/** The ranking options for the file search. If not specified, the file search tool will use the `auto` ranker and a score_threshold of 0.

See the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSearchRankingOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranker: Option<FileSearchRanker>,
	/** The score threshold for the file search. All values must be a floating point number between 0 and 1. */
	pub score_threshold: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FileSearchToolCallType {
	#[serde(rename="file_search_call")]
	FileSearchCall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FileSearchToolCallStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="searching")]
	Searching,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
	#[serde(rename="failed")]
	Failed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSearchToolCallResults {
	/** The unique ID of the file. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/** The text that was retrieved from the file. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/** The name of the file. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<VectorStoreFileAttributes>,
	/** The relevance score of the file - a value between 0 and 1. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub score: Option<f32>,
}

/** The results of a file search tool call. See the 
[file search guide](/docs/guides/tools-file-search) for more information. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSearchToolCall {
	/** The unique ID of the file search tool call. */
	pub id: String,
	#[serde(rename="type")]
	/** The type of the file search tool call. Always `file_search_call`. */
	pub r#type: FileSearchToolCallType,
	/** The status of the file search tool call. One of `in_progress`, 
`searching`, `incomplete` or `failed`, */
	pub status: FileSearchToolCallStatus,
	/** The queries used to search for files. */
	pub queries: Vec<String>,
	/** The results of the file search tool call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub results: Option<Vec<FileSearchToolCallResults>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneChatCompletionRequestAssistantMessageObject {
	/** Controls whether the assistant message is trained against (0 or 1) */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub weight: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneChatCompletionRequestAssistantMessage {
	#[serde(flatten)]
	pub object: FineTuneChatCompletionRequestAssistantMessageObject,
	#[serde(flatten)]
	pub chat_completion_request_assistant_message: ChatCompletionRequestAssistantMessage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneChatRequestInputMessages {
	ChatCompletionRequestSystemMessage(ChatCompletionRequestSystemMessage),
	ChatCompletionRequestUserMessage(ChatCompletionRequestUserMessage),
	FineTuneChatCompletionRequestAssistantMessage(FineTuneChatCompletionRequestAssistantMessage),
	ChatCompletionRequestToolMessage(ChatCompletionRequestToolMessage),
	ChatCompletionRequestFunctionMessage(ChatCompletionRequestFunctionMessage),
}

/** The per-line training example of a fine-tuning input file for chat models using the supervised method.
Input messages may contain text or image content only. Audio and file input messages
are not currently supported for fine-tuning. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneChatRequestInput {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub messages: Option<Vec<FineTuneChatRequestInputMessages>>,
	/** A list of tools the model may generate JSON inputs for. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<ChatCompletionTool>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<ParallelToolCalls>,
	/** A list of functions the model may generate JSON inputs for. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub functions: Option<Vec<ChatCompletionFunctions>>,
}

/** The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneDPOHyperparametersBeta {
	#[serde(rename="auto")]
	Auto,
	Number(f32),
}

/** Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneDPOHyperparametersBatchSize {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneDPOHyperparametersLearningRateMultiplier {
	#[serde(rename="auto")]
	Auto,
	Number(f32),
}

/** The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneDPOHyperparametersNEpochs {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** The hyperparameters used for the DPO fine-tuning job. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneDPOHyperparameters {
	/** The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub beta: Option<FineTuneDPOHyperparametersBeta>,
	/** Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<FineTuneDPOHyperparametersBatchSize>,
	/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<FineTuneDPOHyperparametersLearningRateMultiplier>,
	/** The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<FineTuneDPOHyperparametersNEpochs>,
}

/** Configuration for the DPO fine-tuning method. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneDPOMethod {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<FineTuneDPOHyperparameters>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuneMethodType {
	#[serde(rename="supervised")]
	Supervised,
	#[serde(rename="dpo")]
	Dpo,
	#[serde(rename="reinforcement")]
	Reinforcement,
}

/** The method used for fine-tuning. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneMethod {
	#[serde(rename="type")]
	/** The type of method. Is either `supervised`, `dpo`, or `reinforcement`. */
	pub r#type: FineTuneMethodType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supervised: Option<FineTuneSupervisedMethod>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub dpo: Option<FineTuneDPOMethod>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reinforcement: Option<FineTuneReinforcementMethod>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTunePreferenceRequestInputInputMessages {
	ChatCompletionRequestSystemMessage(ChatCompletionRequestSystemMessage),
	ChatCompletionRequestUserMessage(ChatCompletionRequestUserMessage),
	FineTuneChatCompletionRequestAssistantMessage(FineTuneChatCompletionRequestAssistantMessage),
	ChatCompletionRequestToolMessage(ChatCompletionRequestToolMessage),
	ChatCompletionRequestFunctionMessage(ChatCompletionRequestFunctionMessage),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTunePreferenceRequestInputInput {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub messages: Option<Vec<FineTunePreferenceRequestInputInputMessages>>,
	/** A list of tools the model may generate JSON inputs for. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<ChatCompletionTool>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parallel_tool_calls: Option<ParallelToolCalls>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTunePreferenceRequestInputPreferredOutput {
	ChatCompletionRequestAssistantMessage(ChatCompletionRequestAssistantMessage),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTunePreferenceRequestInputNonPreferredOutput {
	ChatCompletionRequestAssistantMessage(ChatCompletionRequestAssistantMessage),
}

/** The per-line training example of a fine-tuning input file for chat models using the dpo method.
Input messages may contain text or image content only. Audio and file input messages
are not currently supported for fine-tuning. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTunePreferenceRequestInput {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input: Option<FineTunePreferenceRequestInputInput>,
	/** The preferred completion message for the output. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub preferred_output: Option<Vec<FineTunePreferenceRequestInputPreferredOutput>>,
	/** The non-preferred completion message for the output. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub non_preferred_output: Option<Vec<FineTunePreferenceRequestInputNonPreferredOutput>>,
}

/** Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersBatchSize {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersLearningRateMultiplier {
	#[serde(rename="auto")]
	Auto,
	Number(f32),
}

/** The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersNEpochs {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuneReinforcementHyperparametersReasoningEffort {
	#[serde(rename="default")]
	Default,
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
}

/** Multiplier on amount of compute used for exploring search space during training. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersComputeMultiplier {
	#[serde(rename="auto")]
	Auto,
	Number(f32),
}

/** The number of training steps between evaluation runs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEvalInterval {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** Number of evaluation samples to generate per training step. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementHyperparametersEvalSamples {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** The hyperparameters used for the reinforcement fine-tuning job. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneReinforcementHyperparameters {
	/** Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<FineTuneReinforcementHyperparametersBatchSize>,
	/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<FineTuneReinforcementHyperparametersLearningRateMultiplier>,
	/** The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<FineTuneReinforcementHyperparametersNEpochs>,
	/** Level of reasoning effort. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<FineTuneReinforcementHyperparametersReasoningEffort>,
	/** Multiplier on amount of compute used for exploring search space during training. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compute_multiplier: Option<FineTuneReinforcementHyperparametersComputeMultiplier>,
	/** The number of training steps between evaluation runs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eval_interval: Option<FineTuneReinforcementHyperparametersEvalInterval>,
	/** Number of evaluation samples to generate per training step. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eval_samples: Option<FineTuneReinforcementHyperparametersEvalSamples>,
}

	/** The grader used for the fine-tuning job. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneReinforcementMethodGrader(pub String);

/** Configuration for the reinforcement fine-tuning method. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneReinforcementMethod {
	/** The grader used for the fine-tuning job. */
	pub grader: FineTuneReinforcementMethodGrader,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<FineTuneReinforcementHyperparameters>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneReinforcementRequestInputMessages {
	ChatCompletionRequestDeveloperMessage(ChatCompletionRequestDeveloperMessage),
	ChatCompletionRequestUserMessage(ChatCompletionRequestUserMessage),
	FineTuneChatCompletionRequestAssistantMessage(FineTuneChatCompletionRequestAssistantMessage),
	ChatCompletionRequestToolMessage(ChatCompletionRequestToolMessage),
}

/** Per-line training example for reinforcement fine-tuning. Note that `messages` and `tools` are the only reserved keywords.
Any other arbitrary key-value data can be included on training datapoints and will be available to reference during grading under the `{{ item.XXX }}` template variable.
Input messages may contain text or image content only. Audio and file input messages
are not currently supported for fine-tuning. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneReinforcementRequestInput {
	pub messages: Vec<FineTuneReinforcementRequestInputMessages>,
	/** A list of tools the model may generate JSON inputs for. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<ChatCompletionTool>>,
}

/** Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneSupervisedHyperparametersBatchSize {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneSupervisedHyperparametersLearningRateMultiplier {
	#[serde(rename="auto")]
	Auto,
	Number(f32),
}

/** The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuneSupervisedHyperparametersNEpochs {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** The hyperparameters used for the fine-tuning job. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneSupervisedHyperparameters {
	/** Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<FineTuneSupervisedHyperparametersBatchSize>,
	/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<FineTuneSupervisedHyperparametersLearningRateMultiplier>,
	/** The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<FineTuneSupervisedHyperparametersNEpochs>,
}

/** Configuration for the supervised fine-tuning method. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuneSupervisedMethod {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hyperparameters: Option<FineTuneSupervisedHyperparameters>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuningCheckpointPermissionObject {
	#[serde(rename="checkpoint.permission")]
	CheckpointPermission,
}

/** The `checkpoint.permission` object represents a permission for a fine-tuned model checkpoint. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningCheckpointPermission {
	/** The permission identifier, which can be referenced in the API endpoints. */
	pub id: String,
	/** The Unix timestamp (in seconds) for when the permission was created. */
	pub created_at: u64,
	/** The project identifier that the permission is for. */
	pub project_id: String,
	/** The object type, which is always "checkpoint.permission". */
	pub object: FineTuningCheckpointPermissionObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuningIntegrationType {
	#[serde(rename="wandb")]
	Wandb,
}

/** The settings for your integration with Weights and Biases. This payload specifies the project that
metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
to your run, and set a default entity (team, username, etc) to be associated with your run. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningIntegrationWandb {
	/** The name of the project that the new run will be created under. */
	pub project: String,
	/** A display name to set for the run. If not set, we will use the Job ID as the name. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The entity to use for the run. This allows you to set the team or username of the WandB user that you would
like associated with the run. If not set, the default entity for the registered WandB API key is used. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity: Option<String>,
	/** A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some
default tags are generated by OpenAI: "openai/finetune", "openai/{base-model}", "openai/{ftjob-abcdef}". */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tags: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningIntegration {
	#[serde(rename="type")]
	/** The type of the integration being enabled for the fine-tuning job */
	pub r#type: FineTuningIntegrationType,
	/** The settings for your integration with Weights and Biases. This payload specifies the project that
metrics will be sent to. Optionally, you can set an explicit display name for your run, add tags
to your run, and set a default entity (team, username, etc) to be associated with your run. */
	pub wandb: FineTuningIntegrationWandb,
}

/** For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobError {
	/** A machine-readable error code. */
	pub code: String,
	/** A human-readable error message. */
	pub message: String,
	/** The parameter that was invalid, usually `training_file` or `validation_file`. This field will be null if the failure was not parameter-specific. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub param: Option<String>,
}

/** Number of examples in each batch. A larger batch size means that model parameters
are updated less frequently, but with lower variance. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuningJobHyperparametersBatchSize {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
overfitting. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuningJobHyperparametersLearningRateMultiplier {
	#[serde(rename="auto")]
	Auto,
	Number(f32),
}

/** The number of epochs to train the model for. An epoch refers to one full cycle
through the training dataset. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuningJobHyperparametersNEpochs {
	#[serde(rename="auto")]
	Auto,
	Integer(u64),
}

/** The hyperparameters used for the fine-tuning job. This value will only be returned when running `supervised` jobs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobHyperparameters {
	/** Number of examples in each batch. A larger batch size means that model parameters
are updated less frequently, but with lower variance. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_size: Option<FineTuningJobHyperparametersBatchSize>,
	/** Scaling factor for the learning rate. A smaller learning rate may be useful to avoid
overfitting. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub learning_rate_multiplier: Option<FineTuningJobHyperparametersLearningRateMultiplier>,
	/** The number of epochs to train the model for. An epoch refers to one full cycle
through the training dataset. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub n_epochs: Option<FineTuningJobHyperparametersNEpochs>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuningJobObject {
	#[serde(rename="fine_tuning.job")]
	FineTuningJob,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuningJobStatus {
	#[serde(rename="validating_files")]
	ValidatingFiles,
	#[serde(rename="queued")]
	Queued,
	#[serde(rename="running")]
	Running,
	#[serde(rename="succeeded")]
	Succeeded,
	#[serde(rename="failed")]
	Failed,
	#[serde(rename="cancelled")]
	Cancelled,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FineTuningJobIntegrations {
	FineTuningIntegration(FineTuningIntegration),
}

/** The `fine_tuning.job` object represents a fine-tuning job that has been created through the API. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJob {
	/** The object identifier, which can be referenced in the API endpoints. */
	pub id: String,
	/** The Unix timestamp (in seconds) for when the fine-tuning job was created. */
	pub created_at: u64,
	/** For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<FineTuningJobError>,
	/** The name of the fine-tuned model that is being created. The value will be null if the fine-tuning job is still running. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub fine_tuned_model: Option<String>,
	/** The Unix timestamp (in seconds) for when the fine-tuning job was finished. The value will be null if the fine-tuning job is still running. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub finished_at: Option<u64>,
	/** The hyperparameters used for the fine-tuning job. This value will only be returned when running `supervised` jobs. */
	pub hyperparameters: FineTuningJobHyperparameters,
	/** The base model that is being fine-tuned. */
	pub model: String,
	/** The object type, which is always "fine_tuning.job". */
	pub object: FineTuningJobObject,
	/** The organization that owns the fine-tuning job. */
	pub organization_id: String,
	/** The compiled results file ID(s) for the fine-tuning job. You can retrieve the results with the [Files API](/docs/api-reference/files/retrieve-contents). */
	pub result_files: Vec<String>,
	/** The current status of the fine-tuning job, which can be either `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`. */
	pub status: FineTuningJobStatus,
	/** The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub trained_tokens: Option<u64>,
	/** The file ID used for training. You can retrieve the training data with the [Files API](/docs/api-reference/files/retrieve-contents). */
	pub training_file: String,
	/** The file ID used for validation. You can retrieve the validation results with the [Files API](/docs/api-reference/files/retrieve-contents). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub validation_file: Option<String>,
	/** A list of integrations to enable for this fine-tuning job. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub integrations: Option<Vec<FineTuningJobIntegrations>>,
	/** The seed used for the fine-tuning job. */
	pub seed: u64,
	/** The Unix timestamp (in seconds) for when the fine-tuning job is estimated to finish. The value will be null if the fine-tuning job is not running. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub estimated_finish: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub method: Option<FineTuneMethod>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

/** Metrics at the step number during the fine-tuning job. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobCheckpointMetrics {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub step: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub train_loss: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub train_mean_token_accuracy: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_loss: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_mean_token_accuracy: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub full_valid_loss: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub full_valid_mean_token_accuracy: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuningJobCheckpointObject {
	#[serde(rename="fine_tuning.job.checkpoint")]
	FineTuningJobCheckpoint,
}

/** The `fine_tuning.job.checkpoint` object represents a model checkpoint for a fine-tuning job that is ready to use. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobCheckpoint {
	/** The checkpoint identifier, which can be referenced in the API endpoints. */
	pub id: String,
	/** The Unix timestamp (in seconds) for when the checkpoint was created. */
	pub created_at: u64,
	/** The name of the fine-tuned checkpoint model that is created. */
	pub fine_tuned_model_checkpoint: String,
	/** The step number that the checkpoint was created at. */
	pub step_number: u64,
	/** Metrics at the step number during the fine-tuning job. */
	pub metrics: FineTuningJobCheckpointMetrics,
	/** The name of the fine-tuning job that this checkpoint was created from. */
	pub fine_tuning_job_id: String,
	/** The object type, which is always "fine_tuning.job.checkpoint". */
	pub object: FineTuningJobCheckpointObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuningJobEventObject {
	#[serde(rename="fine_tuning.job.event")]
	FineTuningJobEvent,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuningJobEventLevel {
	#[serde(rename="info")]
	Info,
	#[serde(rename="warn")]
	Warn,
	#[serde(rename="error")]
	Error,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FineTuningJobEventType {
	#[serde(rename="message")]
	Message,
	#[serde(rename="metrics")]
	Metrics,
}

	/** The data associated with the event. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobEventData(pub String);

/** Fine-tuning job event object */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FineTuningJobEvent {
	/** The object type, which is always "fine_tuning.job.event". */
	pub object: FineTuningJobEventObject,
	/** The object identifier. */
	pub id: String,
	/** The Unix timestamp (in seconds) for when the fine-tuning job was created. */
	pub created_at: u64,
	/** The log level of the event. */
	pub level: FineTuningJobEventLevel,
	/** The message of the event. */
	pub message: String,
	#[serde(rename="type")]
	/** The type of event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<FineTuningJobEventType>,
	/** The data associated with the event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<FineTuningJobEventData>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionObject {
	/** A description of what the function does, used by the model to choose when and how to call the function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64. */
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<FunctionParameters>,
	/** Whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the `parameters` field. Only a subset of JSON Schema is supported when `strict` is `true`. Learn more about Structured Outputs in the [function calling guide](docs/guides/function-calling). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strict: Option<bool>,
}

/** The parameters the functions accepts, described as a JSON Schema object. See the [guide](/docs/guides/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format. 

Omitting `parameters` defines a function with an empty parameter list. */
pub type FunctionParameters = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FunctionToolCallType {
	#[serde(rename="function_call")]
	FunctionCall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FunctionToolCallStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** A tool call to run a function. See the 
[function calling guide](/docs/guides/function-calling) for more information. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCall {
	/** The unique ID of the function tool call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of the function tool call. Always `function_call`. */
	pub r#type: FunctionToolCallType,
	/** The unique ID of the function tool call generated by the model. */
	pub call_id: String,
	/** The name of the function to run. */
	pub name: String,
	/** A JSON string of the arguments to pass to the function. */
	pub arguments: String,
	/** The status of the item. One of `in_progress`, `completed`, or
`incomplete`. Populated when items are returned via API. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<FunctionToolCallStatus>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FunctionToolCallOutputType {
	#[serde(rename="function_call_output")]
	FunctionCallOutput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FunctionToolCallOutputStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** The output of a function tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallOutput {
	/** The unique ID of the function tool call output. Populated when this item
is returned via API. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of the function tool call output. Always `function_call_output`. */
	pub r#type: FunctionToolCallOutputType,
	/** The unique ID of the function tool call generated by the model. */
	pub call_id: String,
	/** A JSON string of the output of the function tool call. */
	pub output: String,
	/** The status of the item. One of `in_progress`, `completed`, or
`incomplete`. Populated when items are returned via API. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<FunctionToolCallOutputStatus>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallOutputResourceObject {
	/** The unique ID of the function call tool output. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallOutputResource {
	#[serde(flatten)]
	pub function_tool_call_output: FunctionToolCallOutput,
	#[serde(flatten)]
	pub object: FunctionToolCallOutputResourceObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallResourceObject {
	/** The unique ID of the function tool call. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionToolCallResource {
	#[serde(flatten)]
	pub function_tool_call: FunctionToolCall,
	#[serde(flatten)]
	pub object: FunctionToolCallResourceObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraderLabelModelType {
	#[serde(rename="label_model")]
	LabelModel,
}

/** A LabelModelGrader object which uses a model to assign labels to each item
in the evaluation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraderLabelModel {
	#[serde(rename="type")]
	/** The object type, which is always `label_model`. */
	pub r#type: GraderLabelModelType,
	/** The name of the grader. */
	pub name: String,
	/** The model to use for the evaluation. Must support structured outputs. */
	pub model: String,
	pub input: Vec<EvalItem>,
	/** The labels to assign to each item in the evaluation. */
	pub labels: Vec<String>,
	/** The labels that indicate a passing result. Must be a subset of labels. */
	pub passing_labels: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraderMultiType {
	#[serde(rename="multi")]
	Multi,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GraderMultiGraders {
	GraderStringCheck(GraderStringCheck),
	GraderTextSimilarity(GraderTextSimilarity),
	GraderPython(GraderPython),
	GraderScoreModel(GraderScoreModel),
	GraderLabelModel(GraderLabelModel),
}

/** A MultiGrader object combines the output of multiple graders to produce a single score. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraderMulti {
	#[serde(rename="type")]
	/** The object type, which is always `multi`. */
	pub r#type: GraderMultiType,
	/** The name of the grader. */
	pub name: String,
	pub graders: GraderMultiGraders,
	/** A formula to calculate the output based on grader results. */
	pub calculate_output: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraderPythonType {
	#[serde(rename="python")]
	Python,
}

/** A PythonGrader object that runs a python script on the input. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraderPython {
	#[serde(rename="type")]
	/** The object type, which is always `python`. */
	pub r#type: GraderPythonType,
	/** The name of the grader. */
	pub name: String,
	/** The source code of the python script. */
	pub source: String,
	/** The image tag to use for the python script. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_tag: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraderScoreModelType {
	#[serde(rename="score_model")]
	ScoreModel,
}

	/** The sampling parameters for the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraderScoreModelSamplingParams(pub String);

/** A ScoreModelGrader object that uses a model to assign a score to the input. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraderScoreModel {
	#[serde(rename="type")]
	/** The object type, which is always `score_model`. */
	pub r#type: GraderScoreModelType,
	/** The name of the grader. */
	pub name: String,
	/** The model to use for the evaluation. */
	pub model: String,
	/** The sampling parameters for the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sampling_params: Option<GraderScoreModelSamplingParams>,
	/** The input text. This may include template strings. */
	pub input: Vec<EvalItem>,
	/** The range of the score. Defaults to `[0, 1]`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub range: Option<Vec<f32>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraderStringCheckType {
	#[serde(rename="string_check")]
	StringCheck,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraderStringCheckOperation {
	#[serde(rename="eq")]
	Eq,
	#[serde(rename="ne")]
	Ne,
	#[serde(rename="like")]
	Like,
	#[serde(rename="ilike")]
	Ilike,
}

/** A StringCheckGrader object that performs a string comparison between input and reference using a specified operation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraderStringCheck {
	#[serde(rename="type")]
	/** The object type, which is always `string_check`. */
	pub r#type: GraderStringCheckType,
	/** The name of the grader. */
	pub name: String,
	/** The input text. This may include template strings. */
	pub input: String,
	/** The reference text. This may include template strings. */
	pub reference: String,
	/** The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`. */
	pub operation: GraderStringCheckOperation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraderTextSimilarityType {
	#[serde(rename="text_similarity")]
	TextSimilarity,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GraderTextSimilarityEvaluationMetric {
	#[serde(rename="fuzzy_match")]
	FuzzyMatch,
	#[serde(rename="bleu")]
	Bleu,
	#[serde(rename="gleu")]
	Gleu,
	#[serde(rename="meteor")]
	Meteor,
	#[serde(rename="rouge_1")]
	Rouge1,
	#[serde(rename="rouge_2")]
	Rouge2,
	#[serde(rename="rouge_3")]
	Rouge3,
	#[serde(rename="rouge_4")]
	Rouge4,
	#[serde(rename="rouge_5")]
	Rouge5,
	#[serde(rename="rouge_l")]
	RougeL,
}

/** A TextSimilarityGrader object which grades text based on similarity metrics. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GraderTextSimilarity {
	#[serde(rename="type")]
	/** The type of grader. */
	pub r#type: GraderTextSimilarityType,
	/** The name of the grader. */
	pub name: String,
	/** The text being graded. */
	pub input: String,
	/** The text being graded against. */
	pub reference: String,
	/** The evaluation metric to use. One of `fuzzy_match`, `bleu`, `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`, or `rouge_l`. */
	pub evaluation_metric: GraderTextSimilarityEvaluationMetric,
}

/** Represents the content or the URL of an image generated by the OpenAI API. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Image {
	/** The base64-encoded JSON of the generated image. Default value for `gpt-image-1`, and only present if `response_format` is set to `b64_json` for `dall-e-2` and `dall-e-3`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub b64_json: Option<String>,
	/** When using `dall-e-2` or `dall-e-3`, the URL of the generated image if `response_format` is set to `url` (default value). Unsupported for `gpt-image-1`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/** For `dall-e-3` only, the revised prompt that was used to generate the image. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub revised_prompt: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolType {
	#[serde(rename="image_generation")]
	ImageGeneration,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolModel {
	#[serde(rename="gpt-image-1")]
	GptImage1,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolQuality {
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
	#[serde(rename="auto")]
	Auto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolSize {
	#[serde(rename="1024x1024")]
	Size1024x1024,
	#[serde(rename="1024x1536")]
	Size1024x1536,
	#[serde(rename="1536x1024")]
	Size1536x1024,
	#[serde(rename="auto")]
	Auto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolOutputFormat {
	#[serde(rename="png")]
	Png,
	#[serde(rename="webp")]
	Webp,
	#[serde(rename="jpeg")]
	Jpeg,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolModeration {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="low")]
	Low,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolBackground {
	#[serde(rename="transparent")]
	Transparent,
	#[serde(rename="opaque")]
	Opaque,
	#[serde(rename="auto")]
	Auto,
}

/** Optional mask for inpainting. Contains `image_url` 
(string, optional) and `file_id` (string, optional). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageGenToolInputImageMask {
	/** Base64-encoded mask image. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
	/** File ID for the mask image. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}

/** A tool that generates images using a model like `gpt-image-1`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageGenTool {
	#[serde(rename="type")]
	/** The type of the image generation tool. Always `image_generation`. */
	pub r#type: ImageGenToolType,
	/** The image generation model to use. Default: `gpt-image-1`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<ImageGenToolModel>,
	/** The quality of the generated image. One of `low`, `medium`, `high`, 
or `auto`. Default: `auto`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quality: Option<ImageGenToolQuality>,
	/** The size of the generated image. One of `1024x1024`, `1024x1536`, 
`1536x1024`, or `auto`. Default: `auto`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<ImageGenToolSize>,
	/** The output format of the generated image. One of `png`, `webp`, or 
`jpeg`. Default: `png`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_format: Option<ImageGenToolOutputFormat>,
	/** Compression level for the output image. Default: 100. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_compression: Option<u64>,
	/** Moderation level for the generated image. Default: `auto`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub moderation: Option<ImageGenToolModeration>,
	/** Background type for the generated image. One of `transparent`, 
`opaque`, or `auto`. Default: `auto`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub background: Option<ImageGenToolBackground>,
	/** Optional mask for inpainting. Contains `image_url` 
(string, optional) and `file_id` (string, optional). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_image_mask: Option<ImageGenToolInputImageMask>,
	/** Number of partial images to generate in streaming mode, from 0 (default value) to 3. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub partial_images: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolCallType {
	#[serde(rename="image_generation_call")]
	ImageGenerationCall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImageGenToolCallStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="generating")]
	Generating,
	#[serde(rename="failed")]
	Failed,
}

/** An image generation request made by the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageGenToolCall {
	#[serde(rename="type")]
	/** The type of the image generation call. Always `image_generation_call`. */
	pub r#type: ImageGenToolCallType,
	/** The unique ID of the image generation call. */
	pub id: String,
	/** The status of the image generation call. */
	pub status: ImageGenToolCallStatus,
	/** The generated image encoded in base64. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImagesResponseBackground {
	#[serde(rename="transparent")]
	Transparent,
	#[serde(rename="opaque")]
	Opaque,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImagesResponseOutputFormat {
	#[serde(rename="png")]
	Png,
	#[serde(rename="webp")]
	Webp,
	#[serde(rename="jpeg")]
	Jpeg,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImagesResponseSize {
	#[serde(rename="1024x1024")]
	Size1024x1024,
	#[serde(rename="1024x1536")]
	Size1024x1536,
	#[serde(rename="1536x1024")]
	Size1536x1024,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ImagesResponseQuality {
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
}

/** The input tokens detailed information for the image generation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImagesResponseUsageInputTokensDetails {
	/** The number of text tokens in the input prompt. */
	pub text_tokens: u64,
	/** The number of image tokens in the input prompt. */
	pub image_tokens: u64,
}

/** For `gpt-image-1` only, the token usage information for the image generation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImagesResponseUsage {
	/** The total number of tokens (images and text) used for the image generation. */
	pub total_tokens: u64,
	/** The number of tokens (images and text) in the input prompt. */
	pub input_tokens: u64,
	/** The number of image tokens in the output image. */
	pub output_tokens: u64,
	/** The input tokens detailed information for the image generation. */
	pub input_tokens_details: ImagesResponseUsageInputTokensDetails,
}

/** The response from the image generation endpoint. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ImagesResponse {
	/** The Unix timestamp (in seconds) of when the image was created. */
	pub created: u64,
	/** The list of generated images. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<Vec<Image>>,
	/** The background parameter used for the image generation. Either `transparent` or `opaque`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub background: Option<ImagesResponseBackground>,
	/** The output format of the image generation. Either `png`, `webp`, or `jpeg`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_format: Option<ImagesResponseOutputFormat>,
	/** The size of the image generated. Either `1024x1024`, `1024x1536`, or `1536x1024`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<ImagesResponseSize>,
	/** The quality of the image generated. Either `low`, `medium`, or `high`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quality: Option<ImagesResponseQuality>,
	/** For `gpt-image-1` only, the token usage information for the image generation. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<ImagesResponseUsage>,
}

/** Specify additional output data to include in the model response. Currently
supported values are:
- `code_interpreter_call.outputs`: Includes the outputs of python code execution
  in code interpreter tool call items.
- `computer_call_output.output.image_url`: Include image urls from the computer call output.
- `file_search_call.results`: Include the search results of
  the file search tool call.
- `message.input_image.image_url`: Include image urls from the input message.
- `message.output_text.logprobs`: Include logprobs with assistant messages.
- `reasoning.encrypted_content`: Includes an encrypted version of reasoning
  tokens in reasoning item outputs. This enables reasoning items to be used in
  multi-turn conversations when using the Responses API statelessly (like
  when the `store` parameter is set to `false`, or when an organization is
  enrolled in the zero data retention program). */
pub type Includable = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputAudioType {
	#[serde(rename="input_audio")]
	InputAudio,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputAudioFormat {
	#[serde(rename="mp3")]
	Mp3,
	#[serde(rename="wav")]
	Wav,
}

/** An audio input to the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputAudio {
	#[serde(rename="type")]
	/** The type of the input item. Always `input_audio`. */
	pub r#type: InputAudioType,
	/** Base64-encoded audio data. */
	pub data: String,
	/** The format of the audio data. Currently supported formats are `mp3` and
`wav`. */
	pub format: InputAudioFormat,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputContent {
	InputTextContent(InputTextContent),
	InputImageContent(InputImageContent),
	InputFileContent(InputFileContent),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputItem {
	EasyInputMessage(EasyInputMessage),
	/** An item representing part of the context for the response to be 
generated by the model. Can contain text, images, and audio inputs,
as well as previous assistant responses and tool call outputs. */
	Item(Item),
	ItemReferenceParam(ItemReferenceParam),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputMessageType {
	#[serde(rename="message")]
	Message,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputMessageRole {
	#[serde(rename="user")]
	User,
	#[serde(rename="system")]
	System,
	#[serde(rename="developer")]
	Developer,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputMessageStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** A message input to the model with a role indicating instruction following
hierarchy. Instructions given with the `developer` or `system` role take
precedence over instructions given with the `user` role. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessage {
	#[serde(rename="type")]
	/** The type of the message input. Always set to `message`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<InputMessageType>,
	/** The role of the message input. One of `user`, `system`, or `developer`. */
	pub role: InputMessageRole,
	/** The status of item. One of `in_progress`, `completed`, or
`incomplete`. Populated when items are returned via API. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<InputMessageStatus>,
	pub content: InputMessageContentList,
}

/** A list of one or many input items to the model, containing different content 
types. */
pub type InputMessageContentList = Vec<InputContent>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageResourceObject {
	/** The unique ID of the message input. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageResource {
	#[serde(flatten)]
	pub input_message: InputMessage,
	#[serde(flatten)]
	pub object: InputMessageResourceObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InviteObject {
	#[serde(rename="organization.invite")]
	OrganizationInvite,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InviteRole {
	#[serde(rename="owner")]
	Owner,
	#[serde(rename="reader")]
	Reader,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InviteStatus {
	#[serde(rename="accepted")]
	Accepted,
	#[serde(rename="expired")]
	Expired,
	#[serde(rename="pending")]
	Pending,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InviteProjectsRole {
	#[serde(rename="member")]
	Member,
	#[serde(rename="owner")]
	Owner,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteProjects {
	/** Project's public ID */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** Project membership role */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<InviteProjectsRole>,
}

/** Represents an individual `invite` to the organization. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Invite {
	/** The object type, which is always `organization.invite` */
	pub object: InviteObject,
	/** The identifier, which can be referenced in API endpoints */
	pub id: String,
	/** The email address of the individual to whom the invite was sent */
	pub email: String,
	/** `owner` or `reader` */
	pub role: InviteRole,
	/** `accepted`,`expired`, or `pending` */
	pub status: InviteStatus,
	/** The Unix timestamp (in seconds) of when the invite was sent. */
	pub invited_at: u64,
	/** The Unix timestamp (in seconds) of when the invite expires. */
	pub expires_at: u64,
	/** The Unix timestamp (in seconds) of when the invite was accepted. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub accepted_at: Option<u64>,
	/** The projects that were granted membership upon acceptance of the invite. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub projects: Option<Vec<InviteProjects>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InviteDeleteResponseObject {
	#[serde(rename="organization.invite.deleted")]
	OrganizationInviteDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteDeleteResponse {
	/** The object type, which is always `organization.invite.deleted` */
	pub object: InviteDeleteResponseObject,
	pub id: String,
	pub deleted: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InviteListResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteListResponse {
	/** The object type, which is always `list` */
	pub object: InviteListResponseObject,
	pub data: Vec<Invite>,
	/** The first `invite_id` in the retrieved `list` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	/** The last `invite_id` in the retrieved `list` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	/** The `has_more` property is used for pagination to indicate there are additional results. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub has_more: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InviteRequestRole {
	#[serde(rename="reader")]
	Reader,
	#[serde(rename="owner")]
	Owner,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InviteRequestProjectsRole {
	#[serde(rename="member")]
	Member,
	#[serde(rename="owner")]
	Owner,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteRequestProjects {
	/** Project's public ID */
	pub id: String,
	/** Project membership role */
	pub role: InviteRequestProjectsRole,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InviteRequest {
	/** Send an email to this address */
	pub email: String,
	/** `owner` or `reader` */
	pub role: InviteRequestRole,
	/** An array of projects to which membership is granted at the same time the org invite is accepted. If omitted, the user will be invited to the default project for compatibility with legacy behavior. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub projects: Option<Vec<InviteRequestProjects>>,
}

/** Content item used to generate a response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Item {
	InputMessage(InputMessage),
	OutputMessage(OutputMessage),
	FileSearchToolCall(FileSearchToolCall),
	ComputerToolCall(ComputerToolCall),
	ComputerCallOutputItemParam(ComputerCallOutputItemParam),
	WebSearchToolCall(WebSearchToolCall),
	FunctionToolCall(FunctionToolCall),
	FunctionCallOutputItemParam(FunctionCallOutputItemParam),
	ReasoningItem(ReasoningItem),
	ImageGenToolCall(ImageGenToolCall),
	CodeInterpreterToolCall(CodeInterpreterToolCall),
	LocalShellToolCall(LocalShellToolCall),
	LocalShellToolCallOutput(LocalShellToolCallOutput),
	MCPListTools(MCPListTools),
	MCPApprovalRequest(MCPApprovalRequest),
	MCPApprovalResponse(MCPApprovalResponse),
	MCPToolCall(MCPToolCall),
}

/** Content item used to generate a response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemResource {
	InputMessageResource(InputMessageResource),
	OutputMessage(OutputMessage),
	FileSearchToolCall(FileSearchToolCall),
	ComputerToolCall(ComputerToolCall),
	ComputerToolCallOutputResource(ComputerToolCallOutputResource),
	WebSearchToolCall(WebSearchToolCall),
	FunctionToolCallResource(FunctionToolCallResource),
	FunctionToolCallOutputResource(FunctionToolCallOutputResource),
	ImageGenToolCall(ImageGenToolCall),
	CodeInterpreterToolCall(CodeInterpreterToolCall),
	LocalShellToolCall(LocalShellToolCall),
	LocalShellToolCallOutput(LocalShellToolCallOutput),
	MCPListTools(MCPListTools),
	MCPApprovalRequest(MCPApprovalRequest),
	MCPApprovalResponseResource(MCPApprovalResponseResource),
	MCPToolCall(MCPToolCall),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum KeyPressType {
	#[serde(rename="keypress")]
	Keypress,
}

/** A collection of keypresses the model would like to perform. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyPress {
	#[serde(rename="type")]
	/** Specifies the event type. For a keypress action, this property is 
always set to `keypress`. */
	pub r#type: KeyPressType,
	/** The combination of keys the model is requesting to be pressed. This is an
array of strings, each representing a key. */
	pub keys: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAssistantsResponse {
	pub object: String,
	pub data: Vec<AssistantObject>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListAuditLogsResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListAuditLogsResponse {
	pub object: ListAuditLogsResponseObject,
	pub data: Vec<AuditLog>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListBatchesResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListBatchesResponse {
	pub data: Vec<Batch>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	pub has_more: bool,
	pub object: ListBatchesResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListCertificatesResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListCertificatesResponse {
	pub data: Vec<Certificate>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	pub has_more: bool,
	pub object: ListCertificatesResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFilesResponse {
	pub object: String,
	pub data: Vec<OpenAIFile>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListFineTuningCheckpointPermissionResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFineTuningCheckpointPermissionResponse {
	pub data: Vec<FineTuningCheckpointPermission>,
	pub object: ListFineTuningCheckpointPermissionResponseObject,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListFineTuningJobCheckpointsResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFineTuningJobCheckpointsResponse {
	pub data: Vec<FineTuningJobCheckpoint>,
	pub object: ListFineTuningJobCheckpointsResponseObject,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_id: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_id: Option<String>,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListFineTuningJobEventsResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListFineTuningJobEventsResponse {
	pub data: Vec<FineTuningJobEvent>,
	pub object: ListFineTuningJobEventsResponseObject,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListMessagesResponse {
	pub object: String,
	pub data: Vec<MessageObject>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListModelsResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListModelsResponse {
	pub object: ListModelsResponseObject,
	pub data: Vec<Model>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ListPaginatedFineTuningJobsResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPaginatedFineTuningJobsResponse {
	pub data: Vec<FineTuningJob>,
	pub has_more: bool,
	pub object: ListPaginatedFineTuningJobsResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListRunStepsResponse {
	pub object: String,
	pub data: Vec<RunStepObject>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListRunsResponse {
	pub object: String,
	pub data: Vec<RunObject>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListVectorStoreFilesResponse {
	pub object: String,
	pub data: Vec<VectorStoreFileObject>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ListVectorStoresResponse {
	pub object: String,
	pub data: Vec<VectorStoreObject>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LocalShellExecActionType {
	#[serde(rename="exec")]
	Exec,
}

	/** Environment variables to set for the command. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalShellExecActionEnv(pub String);

/** Execute a shell command on the server. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalShellExecAction {
	#[serde(rename="type")]
	/** The type of the local shell action. Always `exec`. */
	pub r#type: LocalShellExecActionType,
	/** The command to run. */
	pub command: Vec<String>,
	/** Optional timeout in milliseconds for the command. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout_ms: Option<u64>,
	/** Optional working directory to run the command in. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub working_directory: Option<String>,
	/** Environment variables to set for the command. */
	pub env: LocalShellExecActionEnv,
	/** Optional user to run the command as. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LocalShellToolType {
	#[serde(rename="local_shell")]
	LocalShell,
}

/** A tool that allows the model to execute shell commands in a local environment. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalShellTool {
	#[serde(rename="type")]
	/** The type of the local shell tool. Always `local_shell`. */
	pub r#type: LocalShellToolType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LocalShellToolCallType {
	#[serde(rename="local_shell_call")]
	LocalShellCall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LocalShellToolCallStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** A tool call to run a command on the local shell. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalShellToolCall {
	#[serde(rename="type")]
	/** The type of the local shell call. Always `local_shell_call`. */
	pub r#type: LocalShellToolCallType,
	/** The unique ID of the local shell call. */
	pub id: String,
	/** The unique ID of the local shell tool call generated by the model. */
	pub call_id: String,
	pub action: LocalShellExecAction,
	/** The status of the local shell call. */
	pub status: LocalShellToolCallStatus,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LocalShellToolCallOutputType {
	#[serde(rename="local_shell_call_output")]
	LocalShellCallOutput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum LocalShellToolCallOutputStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** The output of a local shell tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalShellToolCallOutput {
	#[serde(rename="type")]
	/** The type of the local shell tool call output. Always `local_shell_call_output`. */
	pub r#type: LocalShellToolCallOutputType,
	/** The unique ID of the local shell tool call generated by the model. */
	pub id: String,
	/** A JSON string of the output of the local shell tool call. */
	pub output: String,
	/** The status of the item. One of `in_progress`, `completed`, or `incomplete`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<LocalShellToolCallOutputStatus>,
}

/** A log probability object. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProbProperties {
	/** The token that was used to generate the log probability. */
	pub token: String,
	/** The log probability of the token. */
	pub logprob: f32,
	/** The bytes that were used to generate the log probability. */
	pub bytes: Vec<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MCPApprovalRequestType {
	#[serde(rename="mcp_approval_request")]
	McpApprovalRequest,
}

/** A request for human approval of a tool invocation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPApprovalRequest {
	#[serde(rename="type")]
	/** The type of the item. Always `mcp_approval_request`. */
	pub r#type: MCPApprovalRequestType,
	/** The unique ID of the approval request. */
	pub id: String,
	/** The label of the MCP server making the request. */
	pub server_label: String,
	/** The name of the tool to run. */
	pub name: String,
	/** A JSON string of arguments for the tool. */
	pub arguments: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MCPApprovalResponseType {
	#[serde(rename="mcp_approval_response")]
	McpApprovalResponse,
}

/** A response to an MCP approval request. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPApprovalResponse {
	#[serde(rename="type")]
	/** The type of the item. Always `mcp_approval_response`. */
	pub r#type: MCPApprovalResponseType,
	/** The unique ID of the approval response */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The ID of the approval request being answered. */
	pub approval_request_id: String,
	/** Whether the request was approved. */
	pub approve: bool,
	/** Optional reason for the decision. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MCPApprovalResponseResourceType {
	#[serde(rename="mcp_approval_response")]
	McpApprovalResponse,
}

/** A response to an MCP approval request. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPApprovalResponseResource {
	#[serde(rename="type")]
	/** The type of the item. Always `mcp_approval_response`. */
	pub r#type: MCPApprovalResponseResourceType,
	/** The unique ID of the approval response */
	pub id: String,
	/** The ID of the approval request being answered. */
	pub approval_request_id: String,
	/** Whether the request was approved. */
	pub approve: bool,
	/** Optional reason for the decision. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MCPListToolsType {
	#[serde(rename="mcp_list_tools")]
	McpListTools,
}

/** A list of tools available on an MCP server. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPListTools {
	#[serde(rename="type")]
	/** The type of the item. Always `mcp_list_tools`. */
	pub r#type: MCPListToolsType,
	/** The unique ID of the list. */
	pub id: String,
	/** The label of the MCP server. */
	pub server_label: String,
	/** The tools available on the server. */
	pub tools: Vec<MCPListToolsTool>,
	/** Error message if the server could not list tools. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<String>,
}

	/** The JSON schema describing the tool's input. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPListToolsToolInputSchema(pub String);

	/** Additional annotations about the tool. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPListToolsToolAnnotations(pub String);

/** A tool available on an MCP server. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPListToolsTool {
	/** The name of the tool. */
	pub name: String,
	/** The description of the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** The JSON schema describing the tool's input. */
	pub input_schema: MCPListToolsToolInputSchema,
	/** Additional annotations about the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<MCPListToolsToolAnnotations>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MCPToolType {
	#[serde(rename="mcp")]
	Mcp,
}

	/** Optional HTTP headers to send to the MCP server. Use for authentication
or other purposes. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPToolHeaders(pub String);

/** List of allowed tool names or a filter object. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MCPToolAllowedTools {
	/** A string array of allowed tool names */
	ArrayString(Vec<String>),
	/** A filter object to specify which tools are allowed. */
	MCPallowedtoolsfilter {
		tool_names: Vec<String>,
	},
}

/** A list of tools that always require approval. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPToolRequireApprovalAlways {
	/** List of tools that require approval. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_names: Option<Vec<String>>,
}

/** A list of tools that never require approval. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPToolRequireApprovalNever {
	/** List of tools that do not require approval. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_names: Option<Vec<String>>,
}

/** Specify which of the MCP server's tools require approval. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MCPToolRequireApproval {
	MCPtoolapprovalfilter {
		always: MCPToolRequireApprovalAlways,
		never: MCPToolRequireApprovalNever,
	},
	/** Specify a single approval policy for all tools. One of `always` or 
`never`. When set to `always`, all tools will require approval. When 
set to `never`, all tools will not require approval. */
	#[serde(rename="always")]
	Always,
	#[serde(rename="never")]
	Never,
}

/** Give the model access to additional tools via remote Model Context Protocol 
(MCP) servers. [Learn more about MCP](/docs/guides/tools-remote-mcp). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPTool {
	#[serde(rename="type")]
	/** The type of the MCP tool. Always `mcp`. */
	pub r#type: MCPToolType,
	/** A label for this MCP server, used to identify it in tool calls. */
	pub server_label: String,
	/** The URL for the MCP server. */
	pub server_url: String,
	/** Optional HTTP headers to send to the MCP server. Use for authentication
or other purposes. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub headers: Option<MCPToolHeaders>,
	/** List of allowed tool names or a filter object. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub allowed_tools: Option<MCPToolAllowedTools>,
	/** Specify which of the MCP server's tools require approval. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub require_approval: Option<MCPToolRequireApproval>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MCPToolCallType {
	#[serde(rename="mcp_call")]
	McpCall,
}

/** An invocation of a tool on an MCP server. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MCPToolCall {
	#[serde(rename="type")]
	/** The type of the item. Always `mcp_call`. */
	pub r#type: MCPToolCallType,
	/** The unique ID of the tool call. */
	pub id: String,
	/** The label of the MCP server running the tool. */
	pub server_label: String,
	/** The name of the tool that was run. */
	pub name: String,
	/** A JSON string of the arguments passed to the tool. */
	pub arguments: String,
	/** The output from the tool call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
	/** The error from the tool call, if any. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContentImageFileObjectType {
	#[serde(rename="image_file")]
	ImageFile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContentImageFileObjectImageFileDetail {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="low")]
	Low,
	#[serde(rename="high")]
	High,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentImageFileObjectImageFile {
	/** The [File](/docs/api-reference/files) ID of the image in the message content. Set `purpose="vision"` when uploading the File if you need to later display the file content. */
	pub file_id: String,
	/** Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<MessageContentImageFileObjectImageFileDetail>,
}

/** References an image [File](/docs/api-reference/files) in the content of a message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentImageFileObject {
	#[serde(rename="type")]
	/** Always `image_file`. */
	pub r#type: MessageContentImageFileObjectType,
	pub image_file: MessageContentImageFileObjectImageFile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContentImageUrlObjectType {
	#[serde(rename="image_url")]
	ImageUrl,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContentImageUrlObjectImageUrlDetail {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="low")]
	Low,
	#[serde(rename="high")]
	High,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentImageUrlObjectImageUrl {
	/** The external URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp. */
	pub url: String,
	/** Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<MessageContentImageUrlObjectImageUrlDetail>,
}

/** References an image URL in the content of a message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentImageUrlObject {
	#[serde(rename="type")]
	/** The type of the content part. */
	pub r#type: MessageContentImageUrlObjectType,
	pub image_url: MessageContentImageUrlObjectImageUrl,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContentRefusalObjectType {
	#[serde(rename="refusal")]
	Refusal,
}

/** The refusal content generated by the assistant. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentRefusalObject {
	#[serde(rename="type")]
	/** Always `refusal`. */
	pub r#type: MessageContentRefusalObjectType,
	pub refusal: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContentTextAnnotationsFileCitationObjectType {
	#[serde(rename="file_citation")]
	FileCitation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentTextAnnotationsFileCitationObjectFileCitation {
	/** The ID of the specific File the citation is from. */
	pub file_id: String,
}

/** A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the "file_search" tool to search files. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentTextAnnotationsFileCitationObject {
	#[serde(rename="type")]
	/** Always `file_citation`. */
	pub r#type: MessageContentTextAnnotationsFileCitationObjectType,
	/** The text in the message content that needs to be replaced. */
	pub text: String,
	pub file_citation: MessageContentTextAnnotationsFileCitationObjectFileCitation,
	pub start_index: u64,
	pub end_index: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContentTextAnnotationsFilePathObjectType {
	#[serde(rename="file_path")]
	FilePath,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentTextAnnotationsFilePathObjectFilePath {
	/** The ID of the file that was generated. */
	pub file_id: String,
}

/** A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentTextAnnotationsFilePathObject {
	#[serde(rename="type")]
	/** Always `file_path`. */
	pub r#type: MessageContentTextAnnotationsFilePathObjectType,
	/** The text in the message content that needs to be replaced. */
	pub text: String,
	pub file_path: MessageContentTextAnnotationsFilePathObjectFilePath,
	pub start_index: u64,
	pub end_index: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageContentTextObjectType {
	#[serde(rename="text")]
	Text,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageContentTextObjectTextAnnotations {
	MessageContentTextAnnotationsFileCitationObject(MessageContentTextAnnotationsFileCitationObject),
	MessageContentTextAnnotationsFilePathObject(MessageContentTextAnnotationsFilePathObject),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentTextObjectText {
	/** The data that makes up the text. */
	pub value: String,
	pub annotations: Vec<MessageContentTextObjectTextAnnotations>,
}

/** The text content that is part of a message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageContentTextObject {
	#[serde(rename="type")]
	/** Always `text`. */
	pub r#type: MessageContentTextObjectType,
	pub text: MessageContentTextObjectText,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaContentImageFileObjectType {
	#[serde(rename="image_file")]
	ImageFile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaContentImageFileObjectImageFileDetail {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="low")]
	Low,
	#[serde(rename="high")]
	High,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentImageFileObjectImageFile {
	/** The [File](/docs/api-reference/files) ID of the image in the message content. Set `purpose="vision"` when uploading the File if you need to later display the file content. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/** Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<MessageDeltaContentImageFileObjectImageFileDetail>,
}

/** References an image [File](/docs/api-reference/files) in the content of a message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentImageFileObject {
	/** The index of the content part in the message. */
	pub index: u64,
	#[serde(rename="type")]
	/** Always `image_file`. */
	pub r#type: MessageDeltaContentImageFileObjectType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_file: Option<MessageDeltaContentImageFileObjectImageFile>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaContentImageUrlObjectType {
	#[serde(rename="image_url")]
	ImageUrl,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaContentImageUrlObjectImageUrlDetail {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="low")]
	Low,
	#[serde(rename="high")]
	High,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentImageUrlObjectImageUrl {
	/** The URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	/** Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub detail: Option<MessageDeltaContentImageUrlObjectImageUrlDetail>,
}

/** References an image URL in the content of a message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentImageUrlObject {
	/** The index of the content part in the message. */
	pub index: u64,
	#[serde(rename="type")]
	/** Always `image_url`. */
	pub r#type: MessageDeltaContentImageUrlObjectType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<MessageDeltaContentImageUrlObjectImageUrl>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaContentRefusalObjectType {
	#[serde(rename="refusal")]
	Refusal,
}

/** The refusal content that is part of a message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentRefusalObject {
	/** The index of the refusal part in the message. */
	pub index: u64,
	#[serde(rename="type")]
	/** Always `refusal`. */
	pub r#type: MessageDeltaContentRefusalObjectType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub refusal: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaContentTextAnnotationsFileCitationObjectType {
	#[serde(rename="file_citation")]
	FileCitation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
	/** The ID of the specific File the citation is from. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/** The specific quote in the file. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub quote: Option<String>,
}

/** A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the "file_search" tool to search files. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
	/** The index of the annotation in the text content part. */
	pub index: u64,
	#[serde(rename="type")]
	/** Always `file_citation`. */
	pub r#type: MessageDeltaContentTextAnnotationsFileCitationObjectType,
	/** The text in the message content that needs to be replaced. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_citation: Option<MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_index: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_index: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaContentTextAnnotationsFilePathObjectType {
	#[serde(rename="file_path")]
	FilePath,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
	/** The ID of the file that was generated. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}

/** A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentTextAnnotationsFilePathObject {
	/** The index of the annotation in the text content part. */
	pub index: u64,
	#[serde(rename="type")]
	/** Always `file_path`. */
	pub r#type: MessageDeltaContentTextAnnotationsFilePathObjectType,
	/** The text in the message content that needs to be replaced. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_path: Option<MessageDeltaContentTextAnnotationsFilePathObjectFilePath>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub start_index: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub end_index: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaContentTextObjectType {
	#[serde(rename="text")]
	Text,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageDeltaContentTextObjectTextAnnotations {
	MessageDeltaContentTextAnnotationsFileCitationObject(MessageDeltaContentTextAnnotationsFileCitationObject),
	MessageDeltaContentTextAnnotationsFilePathObject(MessageDeltaContentTextAnnotationsFilePathObject),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentTextObjectText {
	/** The data that makes up the text. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub annotations: Option<Vec<MessageDeltaContentTextObjectTextAnnotations>>,
}

/** The text content that is part of a message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaContentTextObject {
	/** The index of the content part in the message. */
	pub index: u64,
	#[serde(rename="type")]
	/** Always `text`. */
	pub r#type: MessageDeltaContentTextObjectType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<MessageDeltaContentTextObjectText>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaObjectObject {
	#[serde(rename="thread.message.delta")]
	ThreadMessageDelta,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageDeltaObjectDeltaRole {
	#[serde(rename="user")]
	User,
	#[serde(rename="assistant")]
	Assistant,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageDeltaObjectDeltaContent {
	MessageDeltaContentImageFileObject(MessageDeltaContentImageFileObject),
	MessageDeltaContentTextObject(MessageDeltaContentTextObject),
	MessageDeltaContentRefusalObject(MessageDeltaContentRefusalObject),
	MessageDeltaContentImageUrlObject(MessageDeltaContentImageUrlObject),
}

/** The delta containing the fields that have changed on the Message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaObjectDelta {
	/** The entity that produced the message. One of `user` or `assistant`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<MessageDeltaObjectDeltaRole>,
	/** The content of the message in array of text and/or images. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<MessageDeltaObjectDeltaContent>>,
}

/** Represents a message delta i.e. any changed fields on a message during streaming. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDeltaObject {
	/** The identifier of the message, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `thread.message.delta`. */
	pub object: MessageDeltaObjectObject,
	/** The delta containing the fields that have changed on the Message. */
	pub delta: MessageDeltaObjectDelta,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageObjectObject {
	#[serde(rename="thread.message")]
	ThreadMessage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageObjectStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="incomplete")]
	Incomplete,
	#[serde(rename="completed")]
	Completed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageObjectIncompleteDetailsReason {
	#[serde(rename="content_filter")]
	ContentFilter,
	#[serde(rename="max_tokens")]
	MaxTokens,
	#[serde(rename="run_cancelled")]
	RunCancelled,
	#[serde(rename="run_expired")]
	RunExpired,
	#[serde(rename="run_failed")]
	RunFailed,
}

/** On an incomplete message, details about why the message is incomplete. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageObjectIncompleteDetails {
	/** The reason the message is incomplete. */
	pub reason: MessageObjectIncompleteDetailsReason,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageObjectRole {
	#[serde(rename="user")]
	User,
	#[serde(rename="assistant")]
	Assistant,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageObjectContent {
	MessageContentImageFileObject(MessageContentImageFileObject),
	MessageContentImageUrlObject(MessageContentImageUrlObject),
	MessageContentTextObject(MessageContentTextObject),
	MessageContentRefusalObject(MessageContentRefusalObject),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageObjectAttachmentsTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearchTypeOnly(AssistantToolsFileSearchTypeOnly),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageObjectAttachments {
	/** The ID of the file to attach to the message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
	/** The tools to add this file to. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<MessageObjectAttachmentsTools>>,
}

/** Represents a message within a [thread](/docs/api-reference/threads). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageObject {
	/** The identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `thread.message`. */
	pub object: MessageObjectObject,
	/** The Unix timestamp (in seconds) for when the message was created. */
	pub created_at: u64,
	/** The [thread](/docs/api-reference/threads) ID that this message belongs to. */
	pub thread_id: String,
	/** The status of the message, which can be either `in_progress`, `incomplete`, or `completed`. */
	pub status: MessageObjectStatus,
	/** On an incomplete message, details about why the message is incomplete. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incomplete_details: Option<MessageObjectIncompleteDetails>,
	/** The Unix timestamp (in seconds) for when the message was completed. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completed_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the message was marked as incomplete. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incomplete_at: Option<u64>,
	/** The entity that produced the message. One of `user` or `assistant`. */
	pub role: MessageObjectRole,
	/** The content of the message in array of text and/or images. */
	pub content: Vec<MessageObjectContent>,
	/** If applicable, the ID of the [assistant](/docs/api-reference/assistants) that authored this message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub assistant_id: Option<String>,
	/** The ID of the [run](/docs/api-reference/runs) associated with the creation of this message. Value is `null` when messages are created manually using the create message or create thread endpoints. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub run_id: Option<String>,
	/** A list of files attached to the message, and the tools they were added to. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attachments: Option<Vec<MessageObjectAttachments>>,
	pub metadata: Metadata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MessageRequestContentTextObjectType {
	#[serde(rename="text")]
	Text,
}

/** The text content that is part of a message. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageRequestContentTextObject {
	#[serde(rename="type")]
	/** Always `text`. */
	pub r#type: MessageRequestContentTextObjectType,
	/** Text content to be sent to the model */
	pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageStreamEvent {
	/** Occurs when a [message](/docs/api-reference/messages/object) is created. */
	A {
		event: String,
		data: MessageObject,
	},
	/** Occurs when a [message](/docs/api-reference/messages/object) moves to an `in_progress` state. */
	B {
		event: String,
		data: MessageObject,
	},
	/** Occurs when parts of a [Message](/docs/api-reference/messages/object) are being streamed. */
	C {
		event: String,
		data: MessageDeltaObject,
	},
	/** Occurs when a [message](/docs/api-reference/messages/object) is completed. */
	D {
		event: String,
		data: MessageObject,
	},
	/** Occurs when a [message](/docs/api-reference/messages/object) ends before it is completed. */
	E {
		event: String,
		data: MessageObject,
	},
}

/** Set of 16 key-value pairs that can be attached to an object. This can be
useful for storing additional information about the object in a structured
format, and querying for objects via API or the dashboard. 

Keys are strings with a maximum length of 64 characters. Values are strings
with a maximum length of 512 characters. */
pub type Metadata = HashMap<String, String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ModelObject {
	#[serde(rename="model")]
	Model,
}

/** Describes an OpenAI model offering that can be used with the API. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Model {
	/** The model identifier, which can be referenced in the API endpoints. */
	pub id: String,
	/** The Unix timestamp (in seconds) when the model was created. */
	pub created: u64,
	/** The object type, which is always "model". */
	pub object: ModelObject,
	/** The organization that owns the model. */
	pub owned_by: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelIds {
	ModelIdsShared(ModelIdsShared),
	ModelIdsResponses(ModelIdsResponses),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelIdsResponses {
	ModelIdsShared(ModelIdsShared),
	#[serde(rename="o1-pro")]
	O1Pro,
	#[serde(rename="o1-pro-2025-03-19")]
	O1Pro20250319,
	#[serde(rename="o3-pro")]
	O3Pro,
	#[serde(rename="o3-pro-2025-06-10")]
	O3Pro20250610,
	#[serde(rename="o3-deep-research")]
	O3DeepResearch,
	#[serde(rename="o3-deep-research-2025-06-26")]
	O3DeepResearch20250626,
	#[serde(rename="o4-mini-deep-research")]
	O4MiniDeepResearch,
	#[serde(rename="o4-mini-deep-research-2025-06-26")]
	O4MiniDeepResearch20250626,
	#[serde(rename="computer-use-preview")]
	ComputerUsePreview,
	#[serde(rename="computer-use-preview-2025-03-11")]
	ComputerUsePreview20250311,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModelIdsShared {
	String(String),
	#[serde(rename="gpt-4.1")]
	Gpt41,
	#[serde(rename="gpt-4.1-mini")]
	Gpt41Mini,
	#[serde(rename="gpt-4.1-nano")]
	Gpt41Nano,
	#[serde(rename="gpt-4.1-2025-04-14")]
	Gpt4120250414,
	#[serde(rename="gpt-4.1-mini-2025-04-14")]
	Gpt41Mini20250414,
	#[serde(rename="gpt-4.1-nano-2025-04-14")]
	Gpt41Nano20250414,
	#[serde(rename="o4-mini")]
	O4Mini,
	#[serde(rename="o4-mini-2025-04-16")]
	O4Mini20250416,
	#[serde(rename="o3")]
	O3,
	#[serde(rename="o3-2025-04-16")]
	O320250416,
	#[serde(rename="o3-mini")]
	O3Mini,
	#[serde(rename="o3-mini-2025-01-31")]
	O3Mini20250131,
	#[serde(rename="o1")]
	O1,
	#[serde(rename="o1-2024-12-17")]
	O120241217,
	#[serde(rename="o1-preview")]
	O1Preview,
	#[serde(rename="o1-preview-2024-09-12")]
	O1Preview20240912,
	#[serde(rename="o1-mini")]
	O1Mini,
	#[serde(rename="o1-mini-2024-09-12")]
	O1Mini20240912,
	#[serde(rename="gpt-4o")]
	Gpt4o,
	#[serde(rename="gpt-4o-2024-11-20")]
	Gpt4o20241120,
	#[serde(rename="gpt-4o-2024-08-06")]
	Gpt4o20240806,
	#[serde(rename="gpt-4o-2024-05-13")]
	Gpt4o20240513,
	#[serde(rename="gpt-4o-audio-preview")]
	Gpt4oAudioPreview,
	#[serde(rename="gpt-4o-audio-preview-2024-10-01")]
	Gpt4oAudioPreview20241001,
	#[serde(rename="gpt-4o-audio-preview-2024-12-17")]
	Gpt4oAudioPreview20241217,
	#[serde(rename="gpt-4o-audio-preview-2025-06-03")]
	Gpt4oAudioPreview20250603,
	#[serde(rename="gpt-4o-mini-audio-preview")]
	Gpt4oMiniAudioPreview,
	#[serde(rename="gpt-4o-mini-audio-preview-2024-12-17")]
	Gpt4oMiniAudioPreview20241217,
	#[serde(rename="gpt-4o-search-preview")]
	Gpt4oSearchPreview,
	#[serde(rename="gpt-4o-mini-search-preview")]
	Gpt4oMiniSearchPreview,
	#[serde(rename="gpt-4o-search-preview-2025-03-11")]
	Gpt4oSearchPreview20250311,
	#[serde(rename="gpt-4o-mini-search-preview-2025-03-11")]
	Gpt4oMiniSearchPreview20250311,
	#[serde(rename="chatgpt-4o-latest")]
	Chatgpt4oLatest,
	#[serde(rename="codex-mini-latest")]
	CodexMiniLatest,
	#[serde(rename="gpt-4o-mini")]
	Gpt4oMini,
	#[serde(rename="gpt-4o-mini-2024-07-18")]
	Gpt4oMini20240718,
	#[serde(rename="gpt-4-turbo")]
	Gpt4Turbo,
	#[serde(rename="gpt-4-turbo-2024-04-09")]
	Gpt4Turbo20240409,
	#[serde(rename="gpt-4-0125-preview")]
	Gpt40125Preview,
	#[serde(rename="gpt-4-turbo-preview")]
	Gpt4TurboPreview,
	#[serde(rename="gpt-4-1106-preview")]
	Gpt41106Preview,
	#[serde(rename="gpt-4-vision-preview")]
	Gpt4VisionPreview,
	#[serde(rename="gpt-4")]
	Gpt4,
	#[serde(rename="gpt-4-0314")]
	Gpt40314,
	#[serde(rename="gpt-4-0613")]
	Gpt40613,
	#[serde(rename="gpt-4-32k")]
	Gpt432k,
	#[serde(rename="gpt-4-32k-0314")]
	Gpt432k0314,
	#[serde(rename="gpt-4-32k-0613")]
	Gpt432k0613,
	#[serde(rename="gpt-3.5-turbo")]
	Gpt35Turbo,
	#[serde(rename="gpt-3.5-turbo-16k")]
	Gpt35Turbo16k,
	#[serde(rename="gpt-3.5-turbo-0301")]
	Gpt35Turbo0301,
	#[serde(rename="gpt-3.5-turbo-0613")]
	Gpt35Turbo0613,
	#[serde(rename="gpt-3.5-turbo-1106")]
	Gpt35Turbo1106,
	#[serde(rename="gpt-3.5-turbo-0125")]
	Gpt35Turbo0125,
	#[serde(rename="gpt-3.5-turbo-16k-0613")]
	Gpt35Turbo16k0613,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelResponseProperties {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** An integer between 0 and 20 specifying the number of most likely tokens to
return at each token position, each with an associated log probability. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_logprobs: Option<u64>,
	/** What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
We generally recommend altering this or `top_p` but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** An alternative to sampling with temperature, called nucleus sampling,
where the model considers the results of the tokens with top_p probability
mass. So 0.1 means only the tokens comprising the top 10% probability mass
are considered.

We generally recommend altering this or `temperature` but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	/** A stable identifier for your end-users. 
Used to boost cache hit rates by better bucketing similar requests and  to help OpenAI detect and prevent abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_tier: Option<ServiceTier>,
}

/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyAssistantRequestModel {
	String(String),
	AssistantSupportedModels(AssistantSupportedModels),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ModifyAssistantRequestTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyAssistantRequestToolResourcesCodeInterpreter {
	/** Overrides the list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyAssistantRequestToolResourcesFileSearch {
	/** Overrides the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
}

/** A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyAssistantRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<ModifyAssistantRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<ModifyAssistantRequestToolResourcesFileSearch>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyAssistantRequest {
	/** ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<ModifyAssistantRequestModel>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning_effort: Option<ReasoningEffort>,
	/** The name of the assistant. The maximum length is 256 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The description of the assistant. The maximum length is 512 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** The system instructions that the assistant uses. The maximum length is 256,000 characters. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<ModifyAssistantRequestTools>>,
	/** A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<ModifyAssistantRequestToolResources>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.

We generally recommend altering this or temperature but not both. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyCertificateRequest {
	/** The updated name for the certificate */
	pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyMessageRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyRunRequest {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyThreadRequestToolResourcesCodeInterpreter {
	/** A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyThreadRequestToolResourcesFileSearch {
	/** The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
}

/** A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyThreadRequestToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<ModifyThreadRequestToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<ModifyThreadRequestToolResourcesFileSearch>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ModifyThreadRequest {
	/** A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<ModifyThreadRequestToolResources>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum MoveType {
	#[serde(rename="move")]
	Move,
}

/** A mouse move action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Move {
	#[serde(rename="type")]
	/** Specifies the event type. For a move action, this property is 
always set to `move`. */
	pub r#type: MoveType,
	/** The x-coordinate to move to. */
	pub x: u64,
	/** The y-coordinate to move to. */
	pub y: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OpenAIFileObject {
	#[serde(rename="file")]
	File,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OpenAIFilePurpose {
	#[serde(rename="assistants")]
	Assistants,
	#[serde(rename="assistants_output")]
	AssistantsOutput,
	#[serde(rename="batch")]
	Batch,
	#[serde(rename="batch_output")]
	BatchOutput,
	#[serde(rename="fine-tune")]
	FineTune,
	#[serde(rename="fine-tune-results")]
	FineTuneResults,
	#[serde(rename="vision")]
	Vision,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OpenAIFileStatus {
	#[serde(rename="uploaded")]
	Uploaded,
	#[serde(rename="processed")]
	Processed,
	#[serde(rename="error")]
	Error,
}

/** The `File` object represents a document that has been uploaded to OpenAI. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenAIFile {
	/** The file identifier, which can be referenced in the API endpoints. */
	pub id: String,
	/** The size of the file, in bytes. */
	pub bytes: u64,
	/** The Unix timestamp (in seconds) for when the file was created. */
	pub created_at: u64,
	/** The Unix timestamp (in seconds) for when the file will expire. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<u64>,
	/** The name of the file. */
	pub filename: String,
	/** The object type, which is always `file`. */
	pub object: OpenAIFileObject,
	/** The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results` and `vision`. */
	pub purpose: OpenAIFilePurpose,
	/** Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`. */
	pub status: OpenAIFileStatus,
	/** Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status_details: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OtherChunkingStrategyResponseParamType {
	#[serde(rename="other")]
	Other,
}

/** This is returned when the chunking strategy is unknown. Typically, this is because the file was indexed before the `chunking_strategy` concept was introduced in the API. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OtherChunkingStrategyResponseParam {
	#[serde(rename="type")]
	/** Always `other`. */
	pub r#type: OtherChunkingStrategyResponseParamType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputAudioType {
	#[serde(rename="output_audio")]
	OutputAudio,
}

/** An audio output from the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputAudio {
	#[serde(rename="type")]
	/** The type of the output audio. Always `output_audio`. */
	pub r#type: OutputAudioType,
	/** Base64-encoded audio data from the model. */
	pub data: String,
	/** The transcript of the audio data from the model. */
	pub transcript: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutputContent {
	OutputTextContent(OutputTextContent),
	RefusalContent(RefusalContent),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutputItem {
	OutputMessage(OutputMessage),
	FileSearchToolCall(FileSearchToolCall),
	FunctionToolCall(FunctionToolCall),
	WebSearchToolCall(WebSearchToolCall),
	ComputerToolCall(ComputerToolCall),
	ReasoningItem(ReasoningItem),
	ImageGenToolCall(ImageGenToolCall),
	CodeInterpreterToolCall(CodeInterpreterToolCall),
	LocalShellToolCall(LocalShellToolCall),
	MCPToolCall(MCPToolCall),
	MCPListTools(MCPListTools),
	MCPApprovalRequest(MCPApprovalRequest),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputMessageType {
	#[serde(rename="message")]
	Message,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputMessageRole {
	#[serde(rename="assistant")]
	Assistant,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputMessageStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** An output message from the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputMessage {
	/** The unique ID of the output message. */
	pub id: String,
	#[serde(rename="type")]
	/** The type of the output message. Always `message`. */
	pub r#type: OutputMessageType,
	/** The role of the output message. Always `assistant`. */
	pub role: OutputMessageRole,
	/** The content of the output message. */
	pub content: Vec<OutputContent>,
	/** The status of the message input. One of `in_progress`, `completed`, or
`incomplete`. Populated when input items are returned via API. */
	pub status: OutputMessageStatus,
}

/** Whether to enable [parallel function calling](/docs/guides/function-calling#configuring-parallel-function-calling) during tool use. */
pub type ParallelToolCalls = bool;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PredictionContentType {
	#[serde(rename="content")]
	Content,
}

/** The content that should be matched when generating a model response.
If generated tokens would match this content, the entire model response
can be returned much more quickly. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PredictionContentContent {
	/** The content used for a Predicted Output. This is often the
text of a file you are regenerating with minor changes. */
	String(String),
	/** An array of content parts with a defined type. Supported options differ based on the [model](/docs/models) being used to generate the response. Can contain text inputs. */
	ArrayList(Vec<ChatCompletionRequestMessageContentPartText>),
}

/** Static predicted output content, such as the content of a text file that is
being regenerated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PredictionContent {
	#[serde(rename="type")]
	/** The type of the predicted content you want to provide. This type is
currently always `content`. */
	pub r#type: PredictionContentType,
	/** The content that should be matched when generating a model response.
If generated tokens would match this content, the entire model response
can be returned much more quickly. */
	pub content: PredictionContentContent,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectObject {
	#[serde(rename="organization.project")]
	OrganizationProject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectStatus {
	#[serde(rename="active")]
	Active,
	#[serde(rename="archived")]
	Archived,
}

/** Represents an individual project. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
	/** The identifier, which can be referenced in API endpoints */
	pub id: String,
	/** The object type, which is always `organization.project` */
	pub object: ProjectObject,
	/** The name of the project. This appears in reporting. */
	pub name: String,
	/** The Unix timestamp (in seconds) of when the project was created. */
	pub created_at: u64,
	/** The Unix timestamp (in seconds) of when the project was archived or `null`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub archived_at: Option<u64>,
	/** `active` or `archived` */
	pub status: ProjectStatus,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectApiKeyObject {
	#[serde(rename="organization.project.api_key")]
	OrganizationProjectApiKey,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectApiKeyOwnerType {
	#[serde(rename="user")]
	User,
	#[serde(rename="service_account")]
	ServiceAccount,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectApiKeyOwner {
	#[serde(rename="type")]
	/** `user` or `service_account` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<ProjectApiKeyOwnerType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user: Option<ProjectUser>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub service_account: Option<ProjectServiceAccount>,
}

/** Represents an individual API key in a project. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectApiKey {
	/** The object type, which is always `organization.project.api_key` */
	pub object: ProjectApiKeyObject,
	/** The redacted value of the API key */
	pub redacted_value: String,
	/** The name of the API key */
	pub name: String,
	/** The Unix timestamp (in seconds) of when the API key was created */
	pub created_at: u64,
	/** The Unix timestamp (in seconds) of when the API key was last used. */
	pub last_used_at: u64,
	/** The identifier, which can be referenced in API endpoints */
	pub id: String,
	pub owner: ProjectApiKeyOwner,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectApiKeyDeleteResponseObject {
	#[serde(rename="organization.project.api_key.deleted")]
	OrganizationProjectApiKeyDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectApiKeyDeleteResponse {
	pub object: ProjectApiKeyDeleteResponseObject,
	pub id: String,
	pub deleted: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectApiKeyListResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectApiKeyListResponse {
	pub object: ProjectApiKeyListResponseObject,
	pub data: Vec<ProjectApiKey>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectCreateRequest {
	/** The friendly name of the project, this name appears in reports. */
	pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectListResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectListResponse {
	pub object: ProjectListResponseObject,
	pub data: Vec<Project>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectRateLimitObject {
	#[serde(rename="project.rate_limit")]
	ProjectRateLimit,
}

/** Represents a project rate limit config. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRateLimit {
	/** The object type, which is always `project.rate_limit` */
	pub object: ProjectRateLimitObject,
	/** The identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The model this rate limit applies to. */
	pub model: String,
	/** The maximum requests per minute. */
	pub max_requests_per_1_minute: u64,
	/** The maximum tokens per minute. */
	pub max_tokens_per_1_minute: u64,
	/** The maximum images per minute. Only present for relevant models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_images_per_1_minute: Option<u64>,
	/** The maximum audio megabytes per minute. Only present for relevant models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_audio_megabytes_per_1_minute: Option<u64>,
	/** The maximum requests per day. Only present for relevant models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_day: Option<u64>,
	/** The maximum batch input tokens per day. Only present for relevant models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_1_day_max_input_tokens: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectRateLimitListResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRateLimitListResponse {
	pub object: ProjectRateLimitListResponseObject,
	pub data: Vec<ProjectRateLimit>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectRateLimitUpdateRequest {
	/** The maximum requests per minute. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_minute: Option<u64>,
	/** The maximum tokens per minute. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tokens_per_1_minute: Option<u64>,
	/** The maximum images per minute. Only relevant for certain models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_images_per_1_minute: Option<u64>,
	/** The maximum audio megabytes per minute. Only relevant for certain models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_audio_megabytes_per_1_minute: Option<u64>,
	/** The maximum requests per day. Only relevant for certain models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_requests_per_1_day: Option<u64>,
	/** The maximum batch input tokens per day. Only relevant for certain models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch_1_day_max_input_tokens: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectServiceAccountObject {
	#[serde(rename="organization.project.service_account")]
	OrganizationProjectServiceAccount,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectServiceAccountRole {
	#[serde(rename="owner")]
	Owner,
	#[serde(rename="member")]
	Member,
}

/** Represents an individual service account in a project. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServiceAccount {
	/** The object type, which is always `organization.project.service_account` */
	pub object: ProjectServiceAccountObject,
	/** The identifier, which can be referenced in API endpoints */
	pub id: String,
	/** The name of the service account */
	pub name: String,
	/** `owner` or `member` */
	pub role: ProjectServiceAccountRole,
	/** The Unix timestamp (in seconds) of when the service account was created */
	pub created_at: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectServiceAccountApiKeyObject {
	#[serde(rename="organization.project.service_account.api_key")]
	OrganizationProjectServiceAccountApiKey,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServiceAccountApiKey {
	/** The object type, which is always `organization.project.service_account.api_key` */
	pub object: ProjectServiceAccountApiKeyObject,
	pub value: String,
	pub name: String,
	pub created_at: u64,
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServiceAccountCreateRequest {
	/** The name of the service account being created. */
	pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectServiceAccountCreateResponseObject {
	#[serde(rename="organization.project.service_account")]
	OrganizationProjectServiceAccount,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectServiceAccountCreateResponseRole {
	#[serde(rename="member")]
	Member,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServiceAccountCreateResponse {
	pub object: ProjectServiceAccountCreateResponseObject,
	pub id: String,
	pub name: String,
	/** Service accounts can only have one role of type `member` */
	pub role: ProjectServiceAccountCreateResponseRole,
	pub created_at: u64,
	pub api_key: ProjectServiceAccountApiKey,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectServiceAccountDeleteResponseObject {
	#[serde(rename="organization.project.service_account.deleted")]
	OrganizationProjectServiceAccountDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServiceAccountDeleteResponse {
	pub object: ProjectServiceAccountDeleteResponseObject,
	pub id: String,
	pub deleted: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectServiceAccountListResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectServiceAccountListResponse {
	pub object: ProjectServiceAccountListResponseObject,
	pub data: Vec<ProjectServiceAccount>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectUpdateRequest {
	/** The updated name of the project, this name appears in reports. */
	pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectUserObject {
	#[serde(rename="organization.project.user")]
	OrganizationProjectUser,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectUserRole {
	#[serde(rename="owner")]
	Owner,
	#[serde(rename="member")]
	Member,
}

/** Represents an individual user in a project. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectUser {
	/** The object type, which is always `organization.project.user` */
	pub object: ProjectUserObject,
	/** The identifier, which can be referenced in API endpoints */
	pub id: String,
	/** The name of the user */
	pub name: String,
	/** The email address of the user */
	pub email: String,
	/** `owner` or `member` */
	pub role: ProjectUserRole,
	/** The Unix timestamp (in seconds) of when the project was added. */
	pub added_at: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectUserCreateRequestRole {
	#[serde(rename="owner")]
	Owner,
	#[serde(rename="member")]
	Member,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectUserCreateRequest {
	/** The ID of the user. */
	pub user_id: String,
	/** `owner` or `member` */
	pub role: ProjectUserCreateRequestRole,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectUserDeleteResponseObject {
	#[serde(rename="organization.project.user.deleted")]
	OrganizationProjectUserDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectUserDeleteResponse {
	pub object: ProjectUserDeleteResponseObject,
	pub id: String,
	pub deleted: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectUserListResponse {
	pub object: String,
	pub data: Vec<ProjectUser>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProjectUserUpdateRequestRole {
	#[serde(rename="owner")]
	Owner,
	#[serde(rename="member")]
	Member,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectUserUpdateRequest {
	/** `owner` or `member` */
	pub role: ProjectUserUpdateRequestRole,
}

/** Reference to a prompt template and its variables. 
[Learn more](/docs/guides/text?api-mode=responses#reusable-prompts). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Prompt {
	/** The unique identifier of the prompt template to use. */
	pub id: String,
	/** Optional version of the prompt template. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub version: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub variables: Option<ResponsePromptVariables>,
}

/** A realtime client event. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeClientEvent {
	RealtimeClientEventConversationItemCreate(RealtimeClientEventConversationItemCreate),
	RealtimeClientEventConversationItemDelete(RealtimeClientEventConversationItemDelete),
	RealtimeClientEventConversationItemRetrieve(RealtimeClientEventConversationItemRetrieve),
	RealtimeClientEventConversationItemTruncate(RealtimeClientEventConversationItemTruncate),
	RealtimeClientEventInputAudioBufferAppend(RealtimeClientEventInputAudioBufferAppend),
	RealtimeClientEventInputAudioBufferClear(RealtimeClientEventInputAudioBufferClear),
	RealtimeClientEventOutputAudioBufferClear(RealtimeClientEventOutputAudioBufferClear),
	RealtimeClientEventInputAudioBufferCommit(RealtimeClientEventInputAudioBufferCommit),
	RealtimeClientEventResponseCancel(RealtimeClientEventResponseCancel),
	RealtimeClientEventResponseCreate(RealtimeClientEventResponseCreate),
	RealtimeClientEventSessionUpdate(RealtimeClientEventSessionUpdate),
	RealtimeClientEventTranscriptionSessionUpdate(RealtimeClientEventTranscriptionSessionUpdate),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventConversationItemCreateType {
	#[serde(rename="conversation.item.create")]
	ConversationItemCreate,
}

/** Add a new Item to the Conversation's context, including messages, function 
calls, and function call responses. This event can be used both to populate a 
"history" of the conversation and to add new items mid-stream, but has the 
current limitation that it cannot populate assistant audio messages.

If successful, the server will respond with a `conversation.item.created` 
event, otherwise an `error` event will be sent. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventConversationItemCreate {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.create`. */
	pub r#type: RealtimeClientEventConversationItemCreateType,
	/** The ID of the preceding item after which the new item will be inserted. 
If not set, the new item will be appended to the end of the conversation.
If set to `root`, the new item will be added to the beginning of the conversation.
If set to an existing ID, it allows an item to be inserted mid-conversation. If the
ID cannot be found, an error will be returned and the item will not be added. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_item_id: Option<String>,
	pub item: RealtimeConversationItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventConversationItemDeleteType {
	#[serde(rename="conversation.item.delete")]
	ConversationItemDelete,
}

/** Send this event when you want to remove any item from the conversation 
history. The server will respond with a `conversation.item.deleted` event, 
unless the item does not exist in the conversation history, in which case the 
server will respond with an error. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventConversationItemDelete {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.delete`. */
	pub r#type: RealtimeClientEventConversationItemDeleteType,
	/** The ID of the item to delete. */
	pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventConversationItemRetrieveType {
	#[serde(rename="conversation.item.retrieve")]
	ConversationItemRetrieve,
}

/** Send this event when you want to retrieve the server's representation of a specific item in the conversation history. This is useful, for example, to inspect user audio after noise cancellation and VAD.
The server will respond with a `conversation.item.retrieved` event, 
unless the item does not exist in the conversation history, in which case the 
server will respond with an error. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventConversationItemRetrieve {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.retrieve`. */
	pub r#type: RealtimeClientEventConversationItemRetrieveType,
	/** The ID of the item to retrieve. */
	pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventConversationItemTruncateType {
	#[serde(rename="conversation.item.truncate")]
	ConversationItemTruncate,
}

/** Send this event to truncate a previous assistant message’s audio. The server 
will produce audio faster than realtime, so this event is useful when the user 
interrupts to truncate audio that has already been sent to the client but not 
yet played. This will synchronize the server's understanding of the audio with 
the client's playback.

Truncating audio will delete the server-side text transcript to ensure there 
is not text in the context that hasn't been heard by the user.

If successful, the server will respond with a `conversation.item.truncated` 
event. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventConversationItemTruncate {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.truncate`. */
	pub r#type: RealtimeClientEventConversationItemTruncateType,
	/** The ID of the assistant message item to truncate. Only assistant message 
items can be truncated. */
	pub item_id: String,
	/** The index of the content part to truncate. Set this to 0. */
	pub content_index: u64,
	/** Inclusive duration up to which audio is truncated, in milliseconds. If 
the audio_end_ms is greater than the actual audio duration, the server 
will respond with an error. */
	pub audio_end_ms: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventInputAudioBufferAppendType {
	#[serde(rename="input_audio_buffer.append")]
	InputAudioBufferAppend,
}

/** Send this event to append audio bytes to the input audio buffer. The audio 
buffer is temporary storage you can write to and later commit. In Server VAD 
mode, the audio buffer is used to detect speech and the server will decide 
when to commit. When Server VAD is disabled, you must commit the audio buffer
manually.

The client may choose how much audio to place in each event up to a maximum 
of 15 MiB, for example streaming smaller chunks from the client may allow the 
VAD to be more responsive. Unlike made other client events, the server will 
not send a confirmation response to this event. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventInputAudioBufferAppend {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `input_audio_buffer.append`. */
	pub r#type: RealtimeClientEventInputAudioBufferAppendType,
	/** Base64-encoded audio bytes. This must be in the format specified by the 
`input_audio_format` field in the session configuration. */
	pub audio: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventInputAudioBufferClearType {
	#[serde(rename="input_audio_buffer.clear")]
	InputAudioBufferClear,
}

/** Send this event to clear the audio bytes in the buffer. The server will 
respond with an `input_audio_buffer.cleared` event. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventInputAudioBufferClear {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `input_audio_buffer.clear`. */
	pub r#type: RealtimeClientEventInputAudioBufferClearType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventInputAudioBufferCommitType {
	#[serde(rename="input_audio_buffer.commit")]
	InputAudioBufferCommit,
}

/** Send this event to commit the user input audio buffer, which will create a 
new user message item in the conversation. This event will produce an error 
if the input audio buffer is empty. When in Server VAD mode, the client does 
not need to send this event, the server will commit the audio buffer 
automatically.

Committing the input audio buffer will trigger input audio transcription 
(if enabled in session configuration), but it will not create a response 
from the model. The server will respond with an `input_audio_buffer.committed` 
event. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventInputAudioBufferCommit {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `input_audio_buffer.commit`. */
	pub r#type: RealtimeClientEventInputAudioBufferCommitType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventOutputAudioBufferClearType {
	#[serde(rename="output_audio_buffer.clear")]
	OutputAudioBufferClear,
}

/** **WebRTC Only:** Emit to cut off the current audio response. This will trigger the server to
stop generating audio and emit a `output_audio_buffer.cleared` event. This 
event should be preceded by a `response.cancel` client event to stop the 
generation of the current response.
[Learn more](/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventOutputAudioBufferClear {
	/** The unique ID of the client event used for error handling. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `output_audio_buffer.clear`. */
	pub r#type: RealtimeClientEventOutputAudioBufferClearType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventResponseCancelType {
	#[serde(rename="response.cancel")]
	ResponseCancel,
}

/** Send this event to cancel an in-progress response. The server will respond 
with a `response.cancelled` event or an error if there is no response to 
cancel. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventResponseCancel {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `response.cancel`. */
	pub r#type: RealtimeClientEventResponseCancelType,
	/** A specific response ID to cancel - if not provided, will cancel an 
in-progress response in the default conversation. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventResponseCreateType {
	#[serde(rename="response.create")]
	ResponseCreate,
}

/** This event instructs the server to create a Response, which means triggering 
model inference. When in Server VAD mode, the server will create Responses 
automatically.

A Response will include at least one Item, and may have two, in which case 
the second will be a function call. These Items will be appended to the 
conversation history.

The server will respond with a `response.created` event, events for Items 
and content created, and finally a `response.done` event to indicate the 
Response is complete.

The `response.create` event includes inference configuration like 
`instructions`, and `temperature`. These fields will override the Session's 
configuration for this Response only. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventResponseCreate {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `response.create`. */
	pub r#type: RealtimeClientEventResponseCreateType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response: Option<RealtimeResponseCreateParams>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventSessionUpdateType {
	#[serde(rename="session.update")]
	SessionUpdate,
}

/** Send this event to update the session’s default configuration.
The client may send this event at any time to update any field,
except for `voice`. However, note that once a session has been
initialized with a particular `model`, it can’t be changed to
another model using `session.update`.

When the server receives a `session.update`, it will respond
with a `session.updated` event showing the full, effective configuration.
Only the fields that are present are updated. To clear a field like
`instructions`, pass an empty string. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventSessionUpdate {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `session.update`. */
	pub r#type: RealtimeClientEventSessionUpdateType,
	pub session: RealtimeSessionCreateRequest,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeClientEventTranscriptionSessionUpdateType {
	#[serde(rename="transcription_session.update")]
	TranscriptionSessionUpdate,
}

/** Send this event to update a transcription session. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeClientEventTranscriptionSessionUpdate {
	/** Optional client-generated ID used to identify this event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
	#[serde(rename="type")]
	/** The event type, must be `transcription_session.update`. */
	pub r#type: RealtimeClientEventTranscriptionSessionUpdateType,
	pub session: RealtimeTranscriptionSessionCreateRequest,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemType {
	#[serde(rename="message")]
	Message,
	#[serde(rename="function_call")]
	FunctionCall,
	#[serde(rename="function_call_output")]
	FunctionCallOutput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemObject {
	#[serde(rename="realtime.item")]
	RealtimeItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemStatus {
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemRole {
	#[serde(rename="user")]
	User,
	#[serde(rename="assistant")]
	Assistant,
	#[serde(rename="system")]
	System,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemContentType {
	#[serde(rename="input_audio")]
	InputAudio,
	#[serde(rename="input_text")]
	InputText,
	#[serde(rename="item_reference")]
	ItemReference,
	#[serde(rename="text")]
	Text,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeConversationItemContent {
	#[serde(rename="type")]
	/** The content type (`input_text`, `input_audio`, `item_reference`, `text`). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeConversationItemContentType>,
	/** The text content, used for `input_text` and `text` content types. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/** ID of a previous conversation item to reference (for `item_reference`
content types in `response.create` events). These can reference both
client and server created items. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** Base64-encoded audio bytes, used for `input_audio` content type. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<String>,
	/** The transcript of the audio, used for `input_audio` content type. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub transcript: Option<String>,
}

/** The item to add to the conversation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeConversationItem {
	/** The unique ID of the item, this can be generated by the client to help 
manage server-side context, but is not required because the server will 
generate one if not provided. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of the item (`message`, `function_call`, `function_call_output`). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeConversationItemType>,
	/** Identifier for the API object being returned - always `realtime.item`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<RealtimeConversationItemObject>,
	/** The status of the item (`completed`, `incomplete`). These have no effect 
on the conversation, but are accepted for consistency with the 
`conversation.item.created` event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<RealtimeConversationItemStatus>,
	/** The role of the message sender (`user`, `assistant`, `system`), only 
applicable for `message` items. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<RealtimeConversationItemRole>,
	/** The content of the message, applicable for `message` items. 
- Message items of role `system` support only `input_text` content
- Message items of role `user` support `input_text` and `input_audio` 
  content
- Message items of role `assistant` support `text` content. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<RealtimeConversationItemContent>>,
	/** The ID of the function call (for `function_call` and 
`function_call_output` items). If passed on a `function_call_output` 
item, the server will check that a `function_call` item with the same 
ID exists in the conversation history. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub call_id: Option<String>,
	/** The name of the function being called (for `function_call` items). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The arguments of the function call (for `function_call` items). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/** The output of the function call (for `function_call_output` items). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemWithReferenceType {
	#[serde(rename="message")]
	Message,
	#[serde(rename="function_call")]
	FunctionCall,
	#[serde(rename="function_call_output")]
	FunctionCallOutput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemWithReferenceObject {
	#[serde(rename="realtime.item")]
	RealtimeItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemWithReferenceStatus {
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemWithReferenceRole {
	#[serde(rename="user")]
	User,
	#[serde(rename="assistant")]
	Assistant,
	#[serde(rename="system")]
	System,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeConversationItemWithReferenceContentType {
	#[serde(rename="input_audio")]
	InputAudio,
	#[serde(rename="input_text")]
	InputText,
	#[serde(rename="item_reference")]
	ItemReference,
	#[serde(rename="text")]
	Text,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeConversationItemWithReferenceContent {
	#[serde(rename="type")]
	/** The content type (`input_text`, `input_audio`, `item_reference`, `text`). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeConversationItemWithReferenceContentType>,
	/** The text content, used for `input_text` and `text` content types. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/** ID of a previous conversation item to reference (for `item_reference`
content types in `response.create` events). These can reference both
client and server created items. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** Base64-encoded audio bytes, used for `input_audio` content type. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<String>,
	/** The transcript of the audio, used for `input_audio` content type. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub transcript: Option<String>,
}

/** The item to add to the conversation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeConversationItemWithReference {
	/** For an item of type (`message` | `function_call` | `function_call_output`)
this field allows the client to assign the unique ID of the item. It is
not required because the server will generate one if not provided.

For an item of type `item_reference`, this field is required and is a
reference to any item that has previously existed in the conversation. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeConversationItemWithReferenceType>,
	/** Identifier for the API object being returned - always `realtime.item`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<RealtimeConversationItemWithReferenceObject>,
	/** The status of the item (`completed`, `incomplete`). These have no effect 
on the conversation, but are accepted for consistency with the 
`conversation.item.created` event. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<RealtimeConversationItemWithReferenceStatus>,
	/** The role of the message sender (`user`, `assistant`, `system`), only 
applicable for `message` items. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<RealtimeConversationItemWithReferenceRole>,
	/** The content of the message, applicable for `message` items. 
- Message items of role `system` support only `input_text` content
- Message items of role `user` support `input_text` and `input_audio` 
  content
- Message items of role `assistant` support `text` content. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<RealtimeConversationItemWithReferenceContent>>,
	/** The ID of the function call (for `function_call` and 
`function_call_output` items). If passed on a `function_call_output` 
item, the server will check that a `function_call` item with the same 
ID exists in the conversation history. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub call_id: Option<String>,
	/** The name of the function being called (for `function_call` items). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The arguments of the function call (for `function_call` items). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/** The output of the function call (for `function_call_output` items). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeResponseObject {
	#[serde(rename="realtime.response")]
	RealtimeResponse,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeResponseStatus {
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="cancelled")]
	Cancelled,
	#[serde(rename="failed")]
	Failed,
	#[serde(rename="incomplete")]
	Incomplete,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeResponseStatusDetailsType {
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="cancelled")]
	Cancelled,
	#[serde(rename="failed")]
	Failed,
	#[serde(rename="incomplete")]
	Incomplete,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeResponseStatusDetailsReason {
	#[serde(rename="turn_detected")]
	TurnDetected,
	#[serde(rename="client_cancelled")]
	ClientCancelled,
	#[serde(rename="max_output_tokens")]
	MaxOutputTokens,
	#[serde(rename="content_filter")]
	ContentFilter,
}

/** A description of the error that caused the response to fail, 
populated when the `status` is `failed`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponseStatusDetailsError {
	#[serde(rename="type")]
	/** The type of error. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	/** Error code, if any. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
}

/** Additional details about the status. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponseStatusDetails {
	#[serde(rename="type")]
	/** The type of error that caused the response to fail, corresponding 
with the `status` field (`completed`, `cancelled`, `incomplete`, 
`failed`). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeResponseStatusDetailsType>,
	/** The reason the Response did not complete. For a `cancelled` Response, 
one of `turn_detected` (the server VAD detected a new start of speech) 
or `client_cancelled` (the client sent a cancel event). For an 
`incomplete` Response, one of `max_output_tokens` or `content_filter` 
(the server-side safety filter activated and cut off the response). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<RealtimeResponseStatusDetailsReason>,
	/** A description of the error that caused the response to fail, 
populated when the `status` is `failed`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<RealtimeResponseStatusDetailsError>,
}

/** Details about the input tokens used in the Response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponseUsageInputTokenDetails {
	/** The number of cached tokens used in the Response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cached_tokens: Option<u64>,
	/** The number of text tokens used in the Response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text_tokens: Option<u64>,
	/** The number of audio tokens used in the Response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<u64>,
}

/** Details about the output tokens used in the Response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponseUsageOutputTokenDetails {
	/** The number of text tokens used in the Response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text_tokens: Option<u64>,
	/** The number of audio tokens used in the Response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<u64>,
}

/** Usage statistics for the Response, this will correspond to billing. A 
Realtime API session will maintain a conversation context and append new 
Items to the Conversation, thus output from previous turns (text and 
audio tokens) will become the input for later turns. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponseUsage {
	/** The total number of tokens in the Response including input and output 
text and audio tokens. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_tokens: Option<u64>,
	/** The number of input tokens used in the Response, including text and 
audio tokens. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_tokens: Option<u64>,
	/** The number of output tokens sent in the Response, including text and 
audio tokens. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_tokens: Option<u64>,
	/** Details about the input tokens used in the Response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_token_details: Option<RealtimeResponseUsageInputTokenDetails>,
	/** Details about the output tokens used in the Response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_token_details: Option<RealtimeResponseUsageOutputTokenDetails>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeResponseOutputAudioFormat {
	#[serde(rename="pcm16")]
	Pcm16,
	#[serde(rename="g711_ulaw")]
	G711Ulaw,
	#[serde(rename="g711_alaw")]
	G711Alaw,
}

/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls, that was used in this response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeResponseMaxOutputTokens {
	Integer(u64),
	#[serde(rename="inf")]
	Inf,
}

/** The response resource. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponse {
	/** The unique ID of the response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The object type, must be `realtime.response`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<RealtimeResponseObject>,
	/** The final status of the response (`completed`, `cancelled`, `failed`, or 
`incomplete`). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<RealtimeResponseStatus>,
	/** Additional details about the status. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status_details: Option<RealtimeResponseStatusDetails>,
	/** The list of output items generated by the response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<Vec<RealtimeConversationItem>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** Usage statistics for the Response, this will correspond to billing. A 
Realtime API session will maintain a conversation context and append new 
Items to the Conversation, thus output from previous turns (text and 
audio tokens) will become the input for later turns. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<RealtimeResponseUsage>,
	/** Which conversation the response is added to, determined by the `conversation`
field in the `response.create` event. If `auto`, the response will be added to
the default conversation and the value of `conversation_id` will be an id like
`conv_1234`. If `none`, the response will not be added to any conversation and
the value of `conversation_id` will be `null`. If responses are being triggered
by server VAD, the response will be added to the default conversation, thus
the `conversation_id` will be an id like `conv_1234`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub conversation_id: Option<String>,
	/** The voice the model used to respond.
Current voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,
`onyx`, `nova`, `sage`, `shimmer`, and `verse`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
	/** The set of modalities the model used to respond. If there are multiple modalities,
the model will pick one, for example if `modalities` is `["text", "audio"]`, the model
could be responding in either text or audio. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<String>>,
	/** The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeResponseOutputAudioFormat>,
	/** Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls, that was used in this response. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_output_tokens: Option<RealtimeResponseMaxOutputTokens>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeResponseCreateParamsOutputAudioFormat {
	#[serde(rename="pcm16")]
	Pcm16,
	#[serde(rename="g711_ulaw")]
	G711Ulaw,
	#[serde(rename="g711_alaw")]
	G711Alaw,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeResponseCreateParamsToolsType {
	#[serde(rename="function")]
	Function,
}

	/** Parameters of the function in JSON Schema. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponseCreateParamsToolsParameters(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponseCreateParamsTools {
	#[serde(rename="type")]
	/** The type of the tool, i.e. `function`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeResponseCreateParamsToolsType>,
	/** The name of the function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The description of the function, including guidance on when and how 
to call it, and guidance about what to tell the user when calling 
(if anything). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** Parameters of the function in JSON Schema. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<RealtimeResponseCreateParamsToolsParameters>,
}

/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls. Provide an integer between 1 and 4096 to
limit output tokens, or `inf` for the maximum available tokens for a
given model. Defaults to `inf`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsMaxResponseOutputTokens {
	Integer(u64),
	#[serde(rename="inf")]
	Inf,
}

/** Controls which conversation the response is added to. Currently supports
`auto` and `none`, with `auto` as the default value. The `auto` value
means that the contents of the response will be added to the default
conversation. Set this to `none` to create an out-of-band response which 
will not add items to default conversation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeResponseCreateParamsConversation {
	String(String),
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="none")]
	None,
}

/** Create a new Realtime response with these parameters */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeResponseCreateParams {
	/** The set of modalities the model can respond with. To disable audio,
set this to ["text"]. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<String>>,
	/** The default system instructions (i.e. system message) prepended to model 
calls. This field allows the client to guide the model on desired 
responses. The model can be instructed on response content and format, 
(e.g. "be extremely succinct", "act friendly", "here are examples of good 
responses") and on audio behavior (e.g. "talk quickly", "inject emotion 
into your voice", "laugh frequently"). The instructions are not guaranteed 
to be followed by the model, but they provide guidance to the model on the 
desired behavior.

Note that the server sets default instructions which will be used if this 
field is not set and are visible in the `session.created` event at the 
start of the session. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** The voice the model uses to respond. Voice cannot be changed during the 
session once the model has responded with audio at least once. Current 
voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,
`onyx`, `nova`, `sage`, `shimmer`, and `verse`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
	/** The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeResponseCreateParamsOutputAudioFormat>,
	/** Tools (functions) available to the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<RealtimeResponseCreateParamsTools>>,
	/** How the model chooses tools. Options are `auto`, `none`, `required`, or 
specify a function, like `{"type": "function", "function": {"name": "my_function"}}`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<String>,
	/** Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls. Provide an integer between 1 and 4096 to
limit output tokens, or `inf` for the maximum available tokens for a
given model. Defaults to `inf`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_output_tokens: Option<RealtimeResponseCreateParamsMaxResponseOutputTokens>,
	/** Controls which conversation the response is added to. Currently supports
`auto` and `none`, with `auto` as the default value. The `auto` value
means that the contents of the response will be added to the default
conversation. Set this to `none` to create an out-of-band response which 
will not add items to default conversation. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub conversation: Option<RealtimeResponseCreateParamsConversation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
	/** Input items to include in the prompt for the model. Using this field
creates a new context for this Response instead of using the default
conversation. An empty array `[]` will clear the context for this Response.
Note that this can include references to items from the default conversation. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input: Option<Vec<RealtimeConversationItemWithReference>>,
}

/** A realtime server event. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeServerEvent {
	RealtimeServerEventConversationCreated(RealtimeServerEventConversationCreated),
	RealtimeServerEventConversationItemCreated(RealtimeServerEventConversationItemCreated),
	RealtimeServerEventConversationItemDeleted(RealtimeServerEventConversationItemDeleted),
	RealtimeServerEventConversationItemInputAudioTranscriptionCompleted(RealtimeServerEventConversationItemInputAudioTranscriptionCompleted),
	RealtimeServerEventConversationItemInputAudioTranscriptionDelta(RealtimeServerEventConversationItemInputAudioTranscriptionDelta),
	RealtimeServerEventConversationItemInputAudioTranscriptionFailed(RealtimeServerEventConversationItemInputAudioTranscriptionFailed),
	RealtimeServerEventConversationItemRetrieved(RealtimeServerEventConversationItemRetrieved),
	RealtimeServerEventConversationItemTruncated(RealtimeServerEventConversationItemTruncated),
	RealtimeServerEventError(RealtimeServerEventError),
	RealtimeServerEventInputAudioBufferCleared(RealtimeServerEventInputAudioBufferCleared),
	RealtimeServerEventInputAudioBufferCommitted(RealtimeServerEventInputAudioBufferCommitted),
	RealtimeServerEventInputAudioBufferSpeechStarted(RealtimeServerEventInputAudioBufferSpeechStarted),
	RealtimeServerEventInputAudioBufferSpeechStopped(RealtimeServerEventInputAudioBufferSpeechStopped),
	RealtimeServerEventRateLimitsUpdated(RealtimeServerEventRateLimitsUpdated),
	RealtimeServerEventResponseAudioDelta(RealtimeServerEventResponseAudioDelta),
	RealtimeServerEventResponseAudioDone(RealtimeServerEventResponseAudioDone),
	RealtimeServerEventResponseAudioTranscriptDelta(RealtimeServerEventResponseAudioTranscriptDelta),
	RealtimeServerEventResponseAudioTranscriptDone(RealtimeServerEventResponseAudioTranscriptDone),
	RealtimeServerEventResponseContentPartAdded(RealtimeServerEventResponseContentPartAdded),
	RealtimeServerEventResponseContentPartDone(RealtimeServerEventResponseContentPartDone),
	RealtimeServerEventResponseCreated(RealtimeServerEventResponseCreated),
	RealtimeServerEventResponseDone(RealtimeServerEventResponseDone),
	RealtimeServerEventResponseFunctionCallArgumentsDelta(RealtimeServerEventResponseFunctionCallArgumentsDelta),
	RealtimeServerEventResponseFunctionCallArgumentsDone(RealtimeServerEventResponseFunctionCallArgumentsDone),
	RealtimeServerEventResponseOutputItemAdded(RealtimeServerEventResponseOutputItemAdded),
	RealtimeServerEventResponseOutputItemDone(RealtimeServerEventResponseOutputItemDone),
	RealtimeServerEventResponseTextDelta(RealtimeServerEventResponseTextDelta),
	RealtimeServerEventResponseTextDone(RealtimeServerEventResponseTextDone),
	RealtimeServerEventSessionCreated(RealtimeServerEventSessionCreated),
	RealtimeServerEventSessionUpdated(RealtimeServerEventSessionUpdated),
	RealtimeServerEventTranscriptionSessionUpdated(RealtimeServerEventTranscriptionSessionUpdated),
	RealtimeServerEventOutputAudioBufferStarted(RealtimeServerEventOutputAudioBufferStarted),
	RealtimeServerEventOutputAudioBufferStopped(RealtimeServerEventOutputAudioBufferStopped),
	RealtimeServerEventOutputAudioBufferCleared(RealtimeServerEventOutputAudioBufferCleared),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventConversationCreatedType {
	#[serde(rename="conversation.created")]
	ConversationCreated,
}

/** The conversation resource. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationCreatedConversation {
	/** The unique ID of the conversation. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The object type, must be `realtime.conversation`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
}

/** Returned when a conversation is created. Emitted right after session creation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationCreated {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `conversation.created`. */
	pub r#type: RealtimeServerEventConversationCreatedType,
	/** The conversation resource. */
	pub conversation: RealtimeServerEventConversationCreatedConversation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventConversationItemCreatedType {
	#[serde(rename="conversation.item.created")]
	ConversationItemCreated,
}

/** Returned when a conversation item is created. There are several scenarios that produce this event:
  - The server is generating a Response, which if successful will produce 
    either one or two Items, which will be of type `message` 
    (role `assistant`) or type `function_call`.
  - The input audio buffer has been committed, either by the client or the 
    server (in `server_vad` mode). The server will take the content of the 
    input audio buffer and add it to a new user message Item.
  - The client has sent a `conversation.item.create` event to add a new Item 
    to the Conversation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemCreated {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.created`. */
	pub r#type: RealtimeServerEventConversationItemCreatedType,
	/** The ID of the preceding item in the Conversation context, allows the 
client to understand the order of the conversation. */
	pub previous_item_id: String,
	pub item: RealtimeConversationItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventConversationItemDeletedType {
	#[serde(rename="conversation.item.deleted")]
	ConversationItemDeleted,
}

/** Returned when an item in the conversation is deleted by the client with a 
`conversation.item.delete` event. This event is used to synchronize the 
server's understanding of the conversation history with the client's view. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemDeleted {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.deleted`. */
	pub r#type: RealtimeServerEventConversationItemDeletedType,
	/** The ID of the item that was deleted. */
	pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType {
	#[serde(rename="conversation.item.input_audio_transcription.completed")]
	ConversationItemInputAudioTranscriptionCompleted,
}

/** This event is the output of audio transcription for user audio written to the 
user audio buffer. Transcription begins when the input audio buffer is 
committed by the client or server (in `server_vad` mode). Transcription runs 
asynchronously with Response creation, so this event may come before or after 
the Response events.

Realtime API models accept audio natively, and thus input transcription is a
separate process run on a separate ASR (Automatic Speech Recognition) model.
The transcript may diverge somewhat from the model's interpretation, and
should be treated as a rough guide. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be
`conversation.item.input_audio_transcription.completed`. */
	pub r#type: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType,
	/** The ID of the user message item containing the audio. */
	pub item_id: String,
	/** The index of the content part containing the audio. */
	pub content_index: u64,
	/** The transcribed text. */
	pub transcript: String,
	/** The log probabilities of the transcription. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<Vec<LogProbProperties>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType {
	#[serde(rename="conversation.item.input_audio_transcription.delta")]
	ConversationItemInputAudioTranscriptionDelta,
}

/** Returned when the text value of an input audio transcription content part is updated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.input_audio_transcription.delta`. */
	pub r#type: RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the content part in the item's content array. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content_index: Option<u64>,
	/** The text delta. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub delta: Option<String>,
	/** The log probabilities of the transcription. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<Vec<LogProbProperties>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionFailedType {
	#[serde(rename="conversation.item.input_audio_transcription.failed")]
	ConversationItemInputAudioTranscriptionFailed,
}

/** Details of the transcription error. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailedError {
	#[serde(rename="type")]
	/** The type of error. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	/** Error code, if any. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	/** A human-readable error message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<String>,
	/** Parameter related to the error, if any. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub param: Option<String>,
}

/** Returned when input audio transcription is configured, and a transcription 
request for a user message failed. These events are separate from other 
`error` events so that the client can identify the related Item. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be
`conversation.item.input_audio_transcription.failed`. */
	pub r#type: RealtimeServerEventConversationItemInputAudioTranscriptionFailedType,
	/** The ID of the user message item. */
	pub item_id: String,
	/** The index of the content part containing the audio. */
	pub content_index: u64,
	/** Details of the transcription error. */
	pub error: RealtimeServerEventConversationItemInputAudioTranscriptionFailedError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventConversationItemRetrievedType {
	#[serde(rename="conversation.item.retrieved")]
	ConversationItemRetrieved,
}

/** Returned when a conversation item is retrieved with `conversation.item.retrieve`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemRetrieved {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.retrieved`. */
	pub r#type: RealtimeServerEventConversationItemRetrievedType,
	pub item: RealtimeConversationItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventConversationItemTruncatedType {
	#[serde(rename="conversation.item.truncated")]
	ConversationItemTruncated,
}

/** Returned when an earlier assistant audio message item is truncated by the 
client with a `conversation.item.truncate` event. This event is used to 
synchronize the server's understanding of the audio with the client's playback.

This action will truncate the audio and remove the server-side text transcript 
to ensure there is no text in the context that hasn't been heard by the user. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventConversationItemTruncated {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `conversation.item.truncated`. */
	pub r#type: RealtimeServerEventConversationItemTruncatedType,
	/** The ID of the assistant message item that was truncated. */
	pub item_id: String,
	/** The index of the content part that was truncated. */
	pub content_index: u64,
	/** The duration up to which the audio was truncated, in milliseconds. */
	pub audio_end_ms: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventErrorType {
	#[serde(rename="error")]
	Error,
}

/** Details of the error. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventErrorError {
	#[serde(rename="type")]
	/** The type of error (e.g., "invalid_request_error", "server_error"). */
	pub r#type: String,
	/** Error code, if any. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	/** A human-readable error message. */
	pub message: String,
	/** Parameter related to the error, if any. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub param: Option<String>,
	/** The event_id of the client event that caused the error, if applicable. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub event_id: Option<String>,
}

/** Returned when an error occurs, which could be a client problem or a server 
problem. Most errors are recoverable and the session will stay open, we 
recommend to implementors to monitor and log error messages by default. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventError {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `error`. */
	pub r#type: RealtimeServerEventErrorType,
	/** Details of the error. */
	pub error: RealtimeServerEventErrorError,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventInputAudioBufferClearedType {
	#[serde(rename="input_audio_buffer.cleared")]
	InputAudioBufferCleared,
}

/** Returned when the input audio buffer is cleared by the client with a 
`input_audio_buffer.clear` event. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventInputAudioBufferCleared {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `input_audio_buffer.cleared`. */
	pub r#type: RealtimeServerEventInputAudioBufferClearedType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventInputAudioBufferCommittedType {
	#[serde(rename="input_audio_buffer.committed")]
	InputAudioBufferCommitted,
}

/** Returned when an input audio buffer is committed, either by the client or 
automatically in server VAD mode. The `item_id` property is the ID of the user
message item that will be created, thus a `conversation.item.created` event 
will also be sent to the client. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventInputAudioBufferCommitted {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `input_audio_buffer.committed`. */
	pub r#type: RealtimeServerEventInputAudioBufferCommittedType,
	/** The ID of the preceding item after which the new item will be inserted. */
	pub previous_item_id: String,
	/** The ID of the user message item that will be created. */
	pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventInputAudioBufferSpeechStartedType {
	#[serde(rename="input_audio_buffer.speech_started")]
	InputAudioBufferSpeechStarted,
}

/** Sent by the server when in `server_vad` mode to indicate that speech has been 
detected in the audio buffer. This can happen any time audio is added to the 
buffer (unless speech is already detected). The client may want to use this 
event to interrupt audio playback or provide visual feedback to the user. 

The client should expect to receive a `input_audio_buffer.speech_stopped` event 
when speech stops. The `item_id` property is the ID of the user message item 
that will be created when speech stops and will also be included in the 
`input_audio_buffer.speech_stopped` event (unless the client manually commits 
the audio buffer during VAD activation). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `input_audio_buffer.speech_started`. */
	pub r#type: RealtimeServerEventInputAudioBufferSpeechStartedType,
	/** Milliseconds from the start of all audio written to the buffer during the 
session when speech was first detected. This will correspond to the 
beginning of audio sent to the model, and thus includes the 
`prefix_padding_ms` configured in the Session. */
	pub audio_start_ms: u64,
	/** The ID of the user message item that will be created when speech stops. */
	pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventInputAudioBufferSpeechStoppedType {
	#[serde(rename="input_audio_buffer.speech_stopped")]
	InputAudioBufferSpeechStopped,
}

/** Returned in `server_vad` mode when the server detects the end of speech in 
the audio buffer. The server will also send an `conversation.item.created` 
event with the user message item that is created from the audio buffer. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `input_audio_buffer.speech_stopped`. */
	pub r#type: RealtimeServerEventInputAudioBufferSpeechStoppedType,
	/** Milliseconds since the session started when speech stopped. This will 
correspond to the end of audio sent to the model, and thus includes the 
`min_silence_duration_ms` configured in the Session. */
	pub audio_end_ms: u64,
	/** The ID of the user message item that will be created. */
	pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventOutputAudioBufferClearedType {
	#[serde(rename="output_audio_buffer.cleared")]
	OutputAudioBufferCleared,
}

/** **WebRTC Only:** Emitted when the output audio buffer is cleared. This happens either in VAD
mode when the user has interrupted (`input_audio_buffer.speech_started`),
or when the client has emitted the `output_audio_buffer.clear` event to manually
cut off the current audio response.
[Learn more](/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventOutputAudioBufferCleared {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `output_audio_buffer.cleared`. */
	pub r#type: RealtimeServerEventOutputAudioBufferClearedType,
	/** The unique ID of the response that produced the audio. */
	pub response_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventOutputAudioBufferStartedType {
	#[serde(rename="output_audio_buffer.started")]
	OutputAudioBufferStarted,
}

/** **WebRTC Only:** Emitted when the server begins streaming audio to the client. This event is
emitted after an audio content part has been added (`response.content_part.added`)
to the response.
[Learn more](/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventOutputAudioBufferStarted {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `output_audio_buffer.started`. */
	pub r#type: RealtimeServerEventOutputAudioBufferStartedType,
	/** The unique ID of the response that produced the audio. */
	pub response_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventOutputAudioBufferStoppedType {
	#[serde(rename="output_audio_buffer.stopped")]
	OutputAudioBufferStopped,
}

/** **WebRTC Only:** Emitted when the output audio buffer has been completely drained on the server,
and no more audio is forthcoming. This event is emitted after the full response
data has been sent to the client (`response.done`).
[Learn more](/docs/guides/realtime-conversations#client-and-server-events-for-audio-in-webrtc). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventOutputAudioBufferStopped {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `output_audio_buffer.stopped`. */
	pub r#type: RealtimeServerEventOutputAudioBufferStoppedType,
	/** The unique ID of the response that produced the audio. */
	pub response_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventRateLimitsUpdatedType {
	#[serde(rename="rate_limits.updated")]
	RateLimitsUpdated,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventRateLimitsUpdatedRateLimitsName {
	#[serde(rename="requests")]
	Requests,
	#[serde(rename="tokens")]
	Tokens,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventRateLimitsUpdatedRateLimits {
	/** The name of the rate limit (`requests`, `tokens`). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<RealtimeServerEventRateLimitsUpdatedRateLimitsName>,
	/** The maximum allowed value for the rate limit. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub limit: Option<u64>,
	/** The remaining value before the limit is reached. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub remaining: Option<u64>,
	/** Seconds until the rate limit resets. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reset_seconds: Option<f32>,
}

/** Emitted at the beginning of a Response to indicate the updated rate limits. 
When a Response is created some tokens will be "reserved" for the output 
tokens, the rate limits shown here reflect that reservation, which is then 
adjusted accordingly once the Response is completed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventRateLimitsUpdated {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `rate_limits.updated`. */
	pub r#type: RealtimeServerEventRateLimitsUpdatedType,
	/** List of rate limit information. */
	pub rate_limits: Vec<RealtimeServerEventRateLimitsUpdatedRateLimits>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseAudioDeltaType {
	#[serde(rename="response.audio.delta")]
	ResponseAudioDelta,
}

/** Returned when the model-generated audio is updated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseAudioDelta {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.audio.delta`. */
	pub r#type: RealtimeServerEventResponseAudioDeltaType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The index of the content part in the item's content array. */
	pub content_index: u64,
	/** Base64-encoded audio data delta. */
	pub delta: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseAudioDoneType {
	#[serde(rename="response.audio.done")]
	ResponseAudioDone,
}

/** Returned when the model-generated audio is done. Also emitted when a Response
is interrupted, incomplete, or cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseAudioDone {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.audio.done`. */
	pub r#type: RealtimeServerEventResponseAudioDoneType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The index of the content part in the item's content array. */
	pub content_index: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseAudioTranscriptDeltaType {
	#[serde(rename="response.audio_transcript.delta")]
	ResponseAudioTranscriptDelta,
}

/** Returned when the model-generated transcription of audio output is updated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.audio_transcript.delta`. */
	pub r#type: RealtimeServerEventResponseAudioTranscriptDeltaType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The index of the content part in the item's content array. */
	pub content_index: u64,
	/** The transcript delta. */
	pub delta: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseAudioTranscriptDoneType {
	#[serde(rename="response.audio_transcript.done")]
	ResponseAudioTranscriptDone,
}

/** Returned when the model-generated transcription of audio output is done
streaming. Also emitted when a Response is interrupted, incomplete, or
cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.audio_transcript.done`. */
	pub r#type: RealtimeServerEventResponseAudioTranscriptDoneType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The index of the content part in the item's content array. */
	pub content_index: u64,
	/** The final transcript of the audio. */
	pub transcript: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseContentPartAddedType {
	#[serde(rename="response.content_part.added")]
	ResponseContentPartAdded,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseContentPartAddedPartType {
	#[serde(rename="audio")]
	Audio,
	#[serde(rename="text")]
	Text,
}

/** The content part that was added. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseContentPartAddedPart {
	#[serde(rename="type")]
	/** The content type ("text", "audio"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeServerEventResponseContentPartAddedPartType>,
	/** The text content (if type is "text"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/** Base64-encoded audio data (if type is "audio"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<String>,
	/** The transcript of the audio (if type is "audio"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub transcript: Option<String>,
}

/** Returned when a new content part is added to an assistant message item during
response generation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseContentPartAdded {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.content_part.added`. */
	pub r#type: RealtimeServerEventResponseContentPartAddedType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the item to which the content part was added. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The index of the content part in the item's content array. */
	pub content_index: u64,
	/** The content part that was added. */
	pub part: RealtimeServerEventResponseContentPartAddedPart,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseContentPartDoneType {
	#[serde(rename="response.content_part.done")]
	ResponseContentPartDone,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseContentPartDonePartType {
	#[serde(rename="audio")]
	Audio,
	#[serde(rename="text")]
	Text,
}

/** The content part that is done. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseContentPartDonePart {
	#[serde(rename="type")]
	/** The content type ("text", "audio"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeServerEventResponseContentPartDonePartType>,
	/** The text content (if type is "text"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
	/** Base64-encoded audio data (if type is "audio"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio: Option<String>,
	/** The transcript of the audio (if type is "audio"). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub transcript: Option<String>,
}

/** Returned when a content part is done streaming in an assistant message item.
Also emitted when a Response is interrupted, incomplete, or cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseContentPartDone {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.content_part.done`. */
	pub r#type: RealtimeServerEventResponseContentPartDoneType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The index of the content part in the item's content array. */
	pub content_index: u64,
	/** The content part that is done. */
	pub part: RealtimeServerEventResponseContentPartDonePart,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseCreatedType {
	#[serde(rename="response.created")]
	ResponseCreated,
}

/** Returned when a new Response is created. The first event of response creation,
where the response is in an initial state of `in_progress`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseCreated {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.created`. */
	pub r#type: RealtimeServerEventResponseCreatedType,
	pub response: RealtimeResponse,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseDoneType {
	#[serde(rename="response.done")]
	ResponseDone,
}

/** Returned when a Response is done streaming. Always emitted, no matter the 
final state. The Response object included in the `response.done` event will 
include all output Items in the Response but will omit the raw audio data. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseDone {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.done`. */
	pub r#type: RealtimeServerEventResponseDoneType,
	pub response: RealtimeResponse,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
	#[serde(rename="response.function_call_arguments.delta")]
	ResponseFunctionCallArgumentsDelta,
}

/** Returned when the model-generated function call arguments are updated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.function_call_arguments.delta`. */
	pub r#type: RealtimeServerEventResponseFunctionCallArgumentsDeltaType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the function call item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The ID of the function call. */
	pub call_id: String,
	/** The arguments delta as a JSON string. */
	pub delta: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseFunctionCallArgumentsDoneType {
	#[serde(rename="response.function_call_arguments.done")]
	ResponseFunctionCallArgumentsDone,
}

/** Returned when the model-generated function call arguments are done streaming.
Also emitted when a Response is interrupted, incomplete, or cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.function_call_arguments.done`. */
	pub r#type: RealtimeServerEventResponseFunctionCallArgumentsDoneType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the function call item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The ID of the function call. */
	pub call_id: String,
	/** The final arguments as a JSON string. */
	pub arguments: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseOutputItemAddedType {
	#[serde(rename="response.output_item.added")]
	ResponseOutputItemAdded,
}

/** Returned when a new Item is created during Response generation. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseOutputItemAdded {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.output_item.added`. */
	pub r#type: RealtimeServerEventResponseOutputItemAddedType,
	/** The ID of the Response to which the item belongs. */
	pub response_id: String,
	/** The index of the output item in the Response. */
	pub output_index: u64,
	pub item: RealtimeConversationItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseOutputItemDoneType {
	#[serde(rename="response.output_item.done")]
	ResponseOutputItemDone,
}

/** Returned when an Item is done streaming. Also emitted when a Response is 
interrupted, incomplete, or cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseOutputItemDone {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.output_item.done`. */
	pub r#type: RealtimeServerEventResponseOutputItemDoneType,
	/** The ID of the Response to which the item belongs. */
	pub response_id: String,
	/** The index of the output item in the Response. */
	pub output_index: u64,
	pub item: RealtimeConversationItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseTextDeltaType {
	#[serde(rename="response.text.delta")]
	ResponseTextDelta,
}

/** Returned when the text value of a "text" content part is updated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseTextDelta {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.text.delta`. */
	pub r#type: RealtimeServerEventResponseTextDeltaType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The index of the content part in the item's content array. */
	pub content_index: u64,
	/** The text delta. */
	pub delta: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventResponseTextDoneType {
	#[serde(rename="response.text.done")]
	ResponseTextDone,
}

/** Returned when the text value of a "text" content part is done streaming. Also
emitted when a Response is interrupted, incomplete, or cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventResponseTextDone {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `response.text.done`. */
	pub r#type: RealtimeServerEventResponseTextDoneType,
	/** The ID of the response. */
	pub response_id: String,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the output item in the response. */
	pub output_index: u64,
	/** The index of the content part in the item's content array. */
	pub content_index: u64,
	/** The final text content. */
	pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventSessionCreatedType {
	#[serde(rename="session.created")]
	SessionCreated,
}

/** Returned when a Session is created. Emitted automatically when a new 
connection is established as the first server event. This event will contain 
the default Session configuration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventSessionCreated {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `session.created`. */
	pub r#type: RealtimeServerEventSessionCreatedType,
	pub session: RealtimeSession,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventSessionUpdatedType {
	#[serde(rename="session.updated")]
	SessionUpdated,
}

/** Returned when a session is updated with a `session.update` event, unless 
there is an error. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventSessionUpdated {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `session.updated`. */
	pub r#type: RealtimeServerEventSessionUpdatedType,
	pub session: RealtimeSession,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeServerEventTranscriptionSessionUpdatedType {
	#[serde(rename="transcription_session.updated")]
	TranscriptionSessionUpdated,
}

/** Returned when a transcription session is updated with a `transcription_session.update` event, unless 
there is an error. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeServerEventTranscriptionSessionUpdated {
	/** The unique ID of the server event. */
	pub event_id: String,
	#[serde(rename="type")]
	/** The event type, must be `transcription_session.updated`. */
	pub r#type: RealtimeServerEventTranscriptionSessionUpdatedType,
	pub session: RealtimeTranscriptionSessionCreateResponse,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionModel {
	#[serde(rename="gpt-4o-realtime-preview")]
	Gpt4oRealtimePreview,
	#[serde(rename="gpt-4o-realtime-preview-2024-10-01")]
	Gpt4oRealtimePreview20241001,
	#[serde(rename="gpt-4o-realtime-preview-2024-12-17")]
	Gpt4oRealtimePreview20241217,
	#[serde(rename="gpt-4o-realtime-preview-2025-06-03")]
	Gpt4oRealtimePreview20250603,
	#[serde(rename="gpt-4o-mini-realtime-preview")]
	Gpt4oMiniRealtimePreview,
	#[serde(rename="gpt-4o-mini-realtime-preview-2024-12-17")]
	Gpt4oMiniRealtimePreview20241217,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionInputAudioFormat {
	#[serde(rename="pcm16")]
	Pcm16,
	#[serde(rename="g711_ulaw")]
	G711Ulaw,
	#[serde(rename="g711_alaw")]
	G711Alaw,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionOutputAudioFormat {
	#[serde(rename="pcm16")]
	Pcm16,
	#[serde(rename="g711_ulaw")]
	G711Ulaw,
	#[serde(rename="g711_alaw")]
	G711Alaw,
}

/** Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionInputAudioTranscription {
	/** The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/** The language of the input audio. Supplying the input language in
[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
will improve accuracy and latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/** An optional text to guide the model's style or continue a previous audio
segment.
For `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).
For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology". */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionTurnDetectionType {
	#[serde(rename="server_vad")]
	ServerVad,
	#[serde(rename="semantic_vad")]
	SemanticVad,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionTurnDetectionEagerness {
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
	#[serde(rename="auto")]
	Auto,
}

/** Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionTurnDetection {
	#[serde(rename="type")]
	/** Type of turn detection. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionTurnDetectionType>,
	/** Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eagerness: Option<RealtimeSessionTurnDetectionEagerness>,
	/** Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A 
higher threshold will require louder audio to activate the model, and 
thus might perform better in noisy environments. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f32>,
	/** Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in 
milliseconds). Defaults to 300ms. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<u64>,
	/** Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults 
to 500ms. With shorter values the model will respond more quickly, 
but may jump in on short pauses from the user. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<u64>,
	/** Whether or not to automatically generate a response when a VAD stop event occurs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_response: Option<bool>,
	/** Whether or not to automatically interrupt any ongoing response with output to the default
conversation (i.e. `conversation` of `auto`) when a VAD start event occurs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interrupt_response: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionInputAudioNoiseReductionType {
	#[serde(rename="near_field")]
	NearField,
	#[serde(rename="far_field")]
	FarField,
}

/** Configuration for input audio noise reduction. This can be set to `null` to turn off.
Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionInputAudioNoiseReduction {
	#[serde(rename="type")]
	/** Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionInputAudioNoiseReductionType>,
}

	/** The arbitrary metadata to attach to this trace to enable 
filtering in the traces dashboard. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionTracingMetadata(pub String);

/** Configuration options for tracing. Set to null to disable tracing. Once 
tracing is enabled for a session, the configuration cannot be modified.

`auto` will create a trace for the session with default values for the 
workflow name, group id, and metadata. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionTracing {
	/** Default tracing mode for the session. */
	#[serde(rename="auto")]
	Auto,
	/** Granular configuration for tracing. */
	TracingConfiguration {
		workflow_name: String,
		group_id: String,
		metadata: RealtimeSessionTracingMetadata,
	},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionToolsType {
	#[serde(rename="function")]
	Function,
}

	/** Parameters of the function in JSON Schema. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionToolsParameters(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionTools {
	#[serde(rename="type")]
	/** The type of the tool, i.e. `function`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionToolsType>,
	/** The name of the function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The description of the function, including guidance on when and how 
to call it, and guidance about what to tell the user when calling 
(if anything). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** Parameters of the function in JSON Schema. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<RealtimeSessionToolsParameters>,
}

/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls. Provide an integer between 1 and 4096 to
limit output tokens, or `inf` for the maximum available tokens for a
given model. Defaults to `inf`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionMaxResponseOutputTokens {
	Integer(u64),
	#[serde(rename="inf")]
	Inf,
}

/** Realtime session object configuration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSession {
	/** Unique identifier for the session that looks like `sess_1234567890abcdef`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/** The set of modalities the model can respond with. To disable audio,
set this to ["text"]. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<String>>,
	/** The Realtime model used for this session. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeSessionModel>,
	/** The default system instructions (i.e. system message) prepended to model 
calls. This field allows the client to guide the model on desired 
responses. The model can be instructed on response content and format, 
(e.g. "be extremely succinct", "act friendly", "here are examples of good 
responses") and on audio behavior (e.g. "talk quickly", "inject emotion 
into your voice", "laugh frequently"). The instructions are not
guaranteed to be followed by the model, but they provide guidance to the 
model on the desired behavior.


Note that the server sets default instructions which will be used if this
field is not set and are visible in the `session.created` event at the
start of the session. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** The voice the model uses to respond. Voice cannot be changed during the
session once the model has responded with audio at least once. Current
voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,
`onyx`, `nova`, `sage`, `shimmer`, and `verse`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
	/** The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, 
single channel (mono), and little-endian byte order. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<RealtimeSessionInputAudioFormat>,
	/** The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
For `pcm16`, output audio is sampled at a rate of 24kHz. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeSessionOutputAudioFormat>,
	/** Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeSessionInputAudioTranscription>,
	/** Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeSessionTurnDetection>,
	/** Configuration for input audio noise reduction. This can be set to `null` to turn off.
Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_noise_reduction: Option<RealtimeSessionInputAudioNoiseReduction>,
	/** The speed of the model's spoken response. 1.0 is the default speed. 0.25 is
the minimum speed. 1.5 is the maximum speed. This value can only be changed
in between model turns, not while a response is in progress. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<f32>,
	/** Configuration options for tracing. Set to null to disable tracing. Once 
tracing is enabled for a session, the configuration cannot be modified.

`auto` will create a trace for the session with default values for the 
workflow name, group id, and metadata. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tracing: Option<RealtimeSessionTracing>,
	/** Tools (functions) available to the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<RealtimeSessionTools>>,
	/** How the model chooses tools. Options are `auto`, `none`, `required`, or 
specify a function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<String>,
	/** Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls. Provide an integer between 1 and 4096 to
limit output tokens, or `inf` for the maximum available tokens for a
given model. Defaults to `inf`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_output_tokens: Option<RealtimeSessionMaxResponseOutputTokens>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateRequestModel {
	#[serde(rename="gpt-4o-realtime-preview")]
	Gpt4oRealtimePreview,
	#[serde(rename="gpt-4o-realtime-preview-2024-10-01")]
	Gpt4oRealtimePreview20241001,
	#[serde(rename="gpt-4o-realtime-preview-2024-12-17")]
	Gpt4oRealtimePreview20241217,
	#[serde(rename="gpt-4o-realtime-preview-2025-06-03")]
	Gpt4oRealtimePreview20250603,
	#[serde(rename="gpt-4o-mini-realtime-preview")]
	Gpt4oMiniRealtimePreview,
	#[serde(rename="gpt-4o-mini-realtime-preview-2024-12-17")]
	Gpt4oMiniRealtimePreview20241217,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateRequestInputAudioFormat {
	#[serde(rename="pcm16")]
	Pcm16,
	#[serde(rename="g711_ulaw")]
	G711Ulaw,
	#[serde(rename="g711_alaw")]
	G711Alaw,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateRequestOutputAudioFormat {
	#[serde(rename="pcm16")]
	Pcm16,
	#[serde(rename="g711_ulaw")]
	G711Ulaw,
	#[serde(rename="g711_alaw")]
	G711Alaw,
}

/** Configuration for input audio transcription, defaults to off and can be set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequestInputAudioTranscription {
	/** The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/** The language of the input audio. Supplying the input language in
[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
will improve accuracy and latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/** An optional text to guide the model's style or continue a previous audio
segment.
For `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).
For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology". */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateRequestTurnDetectionType {
	#[serde(rename="server_vad")]
	ServerVad,
	#[serde(rename="semantic_vad")]
	SemanticVad,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateRequestTurnDetectionEagerness {
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
	#[serde(rename="auto")]
	Auto,
}

/** Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequestTurnDetection {
	#[serde(rename="type")]
	/** Type of turn detection. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestTurnDetectionType>,
	/** Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eagerness: Option<RealtimeSessionCreateRequestTurnDetectionEagerness>,
	/** Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A
higher threshold will require louder audio to activate the model, and
thus might perform better in noisy environments. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f32>,
	/** Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in
milliseconds). Defaults to 300ms. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<u64>,
	/** Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults
to 500ms. With shorter values the model will respond more quickly,
but may jump in on short pauses from the user. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<u64>,
	/** Whether or not to automatically generate a response when a VAD stop event occurs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_response: Option<bool>,
	/** Whether or not to automatically interrupt any ongoing response with output to the default
conversation (i.e. `conversation` of `auto`) when a VAD start event occurs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interrupt_response: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateRequestInputAudioNoiseReductionType {
	#[serde(rename="near_field")]
	NearField,
	#[serde(rename="far_field")]
	FarField,
}

/** Configuration for input audio noise reduction. This can be set to `null` to turn off.
Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequestInputAudioNoiseReduction {
	#[serde(rename="type")]
	/** Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestInputAudioNoiseReductionType>,
}

	/** The arbitrary metadata to attach to this trace to enable 
filtering in the traces dashboard. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequestTracingMetadata(pub String);

/** Configuration options for tracing. Set to null to disable tracing. Once 
tracing is enabled for a session, the configuration cannot be modified.

`auto` will create a trace for the session with default values for the 
workflow name, group id, and metadata. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestTracing {
	/** Default tracing mode for the session. */
	#[serde(rename="auto")]
	Auto,
	/** Granular configuration for tracing. */
	TracingConfiguration {
		workflow_name: String,
		group_id: String,
		metadata: RealtimeSessionCreateRequestTracingMetadata,
	},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateRequestToolsType {
	#[serde(rename="function")]
	Function,
}

	/** Parameters of the function in JSON Schema. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequestToolsParameters(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequestTools {
	#[serde(rename="type")]
	/** The type of the tool, i.e. `function`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateRequestToolsType>,
	/** The name of the function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The description of the function, including guidance on when and how
to call it, and guidance about what to tell the user when calling
(if anything). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** Parameters of the function in JSON Schema. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<RealtimeSessionCreateRequestToolsParameters>,
}

/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls. Provide an integer between 1 and 4096 to
limit output tokens, or `inf` for the maximum available tokens for a
given model. Defaults to `inf`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens {
	Integer(u64),
	#[serde(rename="inf")]
	Inf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateRequestClientSecretExpiresAfterAnchor {
	#[serde(rename="created_at")]
	CreatedAt,
}

/** Configuration for the ephemeral token expiration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequestClientSecretExpiresAfter {
	/** The anchor point for the ephemeral token expiration. Only `created_at` is currently supported. */
	pub anchor: RealtimeSessionCreateRequestClientSecretExpiresAfterAnchor,
	/** The number of seconds from the anchor point to the expiration. Select a value between `10` and `7200`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seconds: Option<u64>,
}

/** Configuration options for the generated client secret. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequestClientSecret {
	/** Configuration for the ephemeral token expiration. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_after: Option<RealtimeSessionCreateRequestClientSecretExpiresAfter>,
}

/** Realtime session object configuration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateRequest {
	/** The set of modalities the model can respond with. To disable audio,
set this to ["text"]. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<String>>,
	/** The Realtime model used for this session. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeSessionCreateRequestModel>,
	/** The default system instructions (i.e. system message) prepended to model calls. This field allows the client to guide the model on desired responses. The model can be instructed on response content and format, (e.g. "be extremely succinct", "act friendly", "here are examples of good responses") and on audio behavior (e.g. "talk quickly", "inject emotion into your voice", "laugh frequently"). The instructions are not guaranteed to be followed by the model, but they provide guidance to the model on the desired behavior.

Note that the server sets default instructions which will be used if this field is not set and are visible in the `session.created` event at the start of the session. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** The voice the model uses to respond. Voice cannot be changed during the
session once the model has responded with audio at least once. Current
voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,
`onyx`, `nova`, `sage`, `shimmer`, and `verse`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
	/** The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,
single channel (mono), and little-endian byte order. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<RealtimeSessionCreateRequestInputAudioFormat>,
	/** The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
For `pcm16`, output audio is sampled at a rate of 24kHz. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<RealtimeSessionCreateRequestOutputAudioFormat>,
	/** Configuration for input audio transcription, defaults to off and can be set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeSessionCreateRequestInputAudioTranscription>,
	/** Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeSessionCreateRequestTurnDetection>,
	/** Configuration for input audio noise reduction. This can be set to `null` to turn off.
Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_noise_reduction: Option<RealtimeSessionCreateRequestInputAudioNoiseReduction>,
	/** The speed of the model's spoken response. 1.0 is the default speed. 0.25 is
the minimum speed. 1.5 is the maximum speed. This value can only be changed
in between model turns, not while a response is in progress. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<f32>,
	/** Configuration options for tracing. Set to null to disable tracing. Once 
tracing is enabled for a session, the configuration cannot be modified.

`auto` will create a trace for the session with default values for the 
workflow name, group id, and metadata. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tracing: Option<RealtimeSessionCreateRequestTracing>,
	/** Tools (functions) available to the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<RealtimeSessionCreateRequestTools>>,
	/** How the model chooses tools. Options are `auto`, `none`, `required`, or
specify a function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<String>,
	/** Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls. Provide an integer between 1 and 4096 to
limit output tokens, or `inf` for the maximum available tokens for a
given model. Defaults to `inf`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_output_tokens: Option<RealtimeSessionCreateRequestMaxResponseOutputTokens>,
	/** Configuration options for the generated client secret. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_secret: Option<RealtimeSessionCreateRequestClientSecret>,
}

/** Ephemeral key returned by the API. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponseClientSecret {
	/** Ephemeral key usable in client environments to authenticate connections
to the Realtime API. Use this in client-side environments rather than
a standard API token, which should only be used server-side. */
	pub value: String,
	/** Timestamp for when the token expires. Currently, all tokens expire
after one minute. */
	pub expires_at: u64,
}

/** Configuration for input audio transcription, defaults to off and can be 
set to `null` to turn off once on. Input audio transcription is not native 
to the model, since the model consumes audio directly. Transcription runs
asynchronously and should be treated as rough guidance
rather than the representation understood by the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponseInputAudioTranscription {
	/** The model to use for transcription. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
}

	/** The arbitrary metadata to attach to this trace to enable 
filtering in the traces dashboard. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponseTracingMetadata(pub String);

/** Configuration options for tracing. Set to null to disable tracing. Once 
tracing is enabled for a session, the configuration cannot be modified.

`auto` will create a trace for the session with default values for the 
workflow name, group id, and metadata. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionCreateResponseTracing {
	/** Default tracing mode for the session. */
	#[serde(rename="auto")]
	Auto,
	/** Granular configuration for tracing. */
	TracingConfiguration {
		workflow_name: String,
		group_id: String,
		metadata: RealtimeSessionCreateResponseTracingMetadata,
	},
}

/** Configuration for turn detection. Can be set to `null` to turn off. Server 
VAD means that the model will detect the start and end of speech based on 
audio volume and respond at the end of user speech. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponseTurnDetection {
	#[serde(rename="type")]
	/** Type of turn detection, only `server_vad` is currently supported. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	/** Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A 
higher threshold will require louder audio to activate the model, and 
thus might perform better in noisy environments. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f32>,
	/** Amount of audio to include before the VAD detected speech (in 
milliseconds). Defaults to 300ms. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<u64>,
	/** Duration of silence to detect speech stop (in milliseconds). Defaults 
to 500ms. With shorter values the model will respond more quickly, 
but may jump in on short pauses from the user. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeSessionCreateResponseToolsType {
	#[serde(rename="function")]
	Function,
}

	/** Parameters of the function in JSON Schema. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponseToolsParameters(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponseTools {
	#[serde(rename="type")]
	/** The type of the tool, i.e. `function`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeSessionCreateResponseToolsType>,
	/** The name of the function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The description of the function, including guidance on when and how 
to call it, and guidance about what to tell the user when calling 
(if anything). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** Parameters of the function in JSON Schema. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameters: Option<RealtimeSessionCreateResponseToolsParameters>,
}

/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls. Provide an integer between 1 and 4096 to
limit output tokens, or `inf` for the maximum available tokens for a
given model. Defaults to `inf`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RealtimeSessionCreateResponseMaxResponseOutputTokens {
	Integer(u64),
	#[serde(rename="inf")]
	Inf,
}

/** A new Realtime session configuration, with an ephermeral key. Default TTL
for keys is one minute. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeSessionCreateResponse {
	/** Ephemeral key returned by the API. */
	pub client_secret: RealtimeSessionCreateResponseClientSecret,
	/** The set of modalities the model can respond with. To disable audio,
set this to ["text"]. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<String>>,
	/** The default system instructions (i.e. system message) prepended to model 
calls. This field allows the client to guide the model on desired 
responses. The model can be instructed on response content and format, 
(e.g. "be extremely succinct", "act friendly", "here are examples of good 
responses") and on audio behavior (e.g. "talk quickly", "inject emotion 
into your voice", "laugh frequently"). The instructions are not guaranteed 
to be followed by the model, but they provide guidance to the model on the 
desired behavior.

Note that the server sets default instructions which will be used if this 
field is not set and are visible in the `session.created` event at the 
start of the session. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<String>,
	/** The voice the model uses to respond. Voice cannot be changed during the 
session once the model has responded with audio at least once. Current 
voice options are `alloy`, `ash`, `ballad`, `coral`, `echo` `sage`, 
`shimmer` and `verse`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voice: Option<VoiceIdsShared>,
	/** The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<String>,
	/** The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_format: Option<String>,
	/** Configuration for input audio transcription, defaults to off and can be 
set to `null` to turn off once on. Input audio transcription is not native 
to the model, since the model consumes audio directly. Transcription runs
asynchronously and should be treated as rough guidance
rather than the representation understood by the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeSessionCreateResponseInputAudioTranscription>,
	/** The speed of the model's spoken response. 1.0 is the default speed. 0.25 is
the minimum speed. 1.5 is the maximum speed. This value can only be changed
in between model turns, not while a response is in progress. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub speed: Option<f32>,
	/** Configuration options for tracing. Set to null to disable tracing. Once 
tracing is enabled for a session, the configuration cannot be modified.

`auto` will create a trace for the session with default values for the 
workflow name, group id, and metadata. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tracing: Option<RealtimeSessionCreateResponseTracing>,
	/** Configuration for turn detection. Can be set to `null` to turn off. Server 
VAD means that the model will detect the start and end of speech based on 
audio volume and respond at the end of user speech. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeSessionCreateResponseTurnDetection>,
	/** Tools (functions) available to the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<RealtimeSessionCreateResponseTools>>,
	/** How the model chooses tools. Options are `auto`, `none`, `required`, or 
specify a function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<String>,
	/** Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** Maximum number of output tokens for a single assistant response,
inclusive of tool calls. Provide an integer between 1 and 4096 to
limit output tokens, or `inf` for the maximum available tokens for a
given model. Defaults to `inf`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_output_tokens: Option<RealtimeSessionCreateResponseMaxResponseOutputTokens>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
	#[serde(rename="pcm16")]
	Pcm16,
	#[serde(rename="g711_ulaw")]
	G711Ulaw,
	#[serde(rename="g711_alaw")]
	G711Alaw,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel {
	#[serde(rename="gpt-4o-transcribe")]
	Gpt4oTranscribe,
	#[serde(rename="gpt-4o-mini-transcribe")]
	Gpt4oMiniTranscribe,
	#[serde(rename="whisper-1")]
	Whisper1,
}

/** Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioTranscription {
	/** The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel>,
	/** The language of the input audio. Supplying the input language in
[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
will improve accuracy and latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/** An optional text to guide the model's style or continue a previous audio
segment.
For `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).
For `gpt-4o-transcribe` models, the prompt is a free text string, for example "expect words related to technology". */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
	#[serde(rename="server_vad")]
	ServerVad,
	#[serde(rename="semantic_vad")]
	SemanticVad,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness {
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
	#[serde(rename="auto")]
	Auto,
}

/** Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateRequestTurnDetection {
	#[serde(rename="type")]
	/** Type of turn detection. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionType>,
	/** Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub eagerness: Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness>,
	/** Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A
higher threshold will require louder audio to activate the model, and
thus might perform better in noisy environments. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f32>,
	/** Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in
milliseconds). Defaults to 300ms. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<u64>,
	/** Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults
to 500ms. With shorter values the model will respond more quickly,
but may jump in on short pauses from the user. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<u64>,
	/** Whether or not to automatically generate a response when a VAD stop event occurs. Not available for transcription sessions. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub create_response: Option<bool>,
	/** Whether or not to automatically interrupt any ongoing response with output to the default
conversation (i.e. `conversation` of `auto`) when a VAD start event occurs. Not available for transcription sessions. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub interrupt_response: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType {
	#[serde(rename="near_field")]
	NearField,
	#[serde(rename="far_field")]
	FarField,
}

/** Configuration for input audio noise reduction. This can be set to `null` to turn off.
Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction {
	#[serde(rename="type")]
	/** Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAtAnchor {
	#[serde(rename="created_at")]
	CreatedAt,
}

/** Configuration for the ephemeral token expiration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAt {
	/** The anchor point for the ephemeral token expiration. Only `created_at` is currently supported. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub anchor: Option<RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAtAnchor>,
	/** The number of seconds from the anchor point to the expiration. Select a value between `10` and `7200`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub seconds: Option<u64>,
}

/** Configuration options for the generated client secret. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateRequestClientSecret {
	/** Configuration for the ephemeral token expiration. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<RealtimeTranscriptionSessionCreateRequestClientSecretExpiresAt>,
}

/** Realtime transcription session object configuration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateRequest {
	/** The set of modalities the model can respond with. To disable audio,
set this to ["text"]. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<String>>,
	/** The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.
For `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate,
single channel (mono), and little-endian byte order. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<RealtimeTranscriptionSessionCreateRequestInputAudioFormat>,
	/** Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeTranscriptionSessionCreateRequestInputAudioTranscription>,
	/** Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.
Server VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.
Semantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with "uhhm", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeTranscriptionSessionCreateRequestTurnDetection>,
	/** Configuration for input audio noise reduction. This can be set to `null` to turn off.
Noise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.
Filtering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_noise_reduction: Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction>,
	/** The set of items to include in the transcription. Current available items are:
- `item.input_audio_transcription.logprobs` */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub include: Option<Vec<String>>,
	/** Configuration options for the generated client secret. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub client_secret: Option<RealtimeTranscriptionSessionCreateRequestClientSecret>,
}

/** Ephemeral key returned by the API. Only present when the session is
created on the server via REST API. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateResponseClientSecret {
	/** Ephemeral key usable in client environments to authenticate connections
to the Realtime API. Use this in client-side environments rather than
a standard API token, which should only be used server-side. */
	pub value: String,
	/** Timestamp for when the token expires. Currently, all tokens expire
after one minute. */
	pub expires_at: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RealtimeTranscriptionSessionCreateResponseInputAudioTranscriptionModel {
	#[serde(rename="gpt-4o-transcribe")]
	Gpt4oTranscribe,
	#[serde(rename="gpt-4o-mini-transcribe")]
	Gpt4oMiniTranscribe,
	#[serde(rename="whisper-1")]
	Whisper1,
}

/** Configuration of the transcription model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateResponseInputAudioTranscription {
	/** The model to use for transcription. Can be `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, or `whisper-1`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<RealtimeTranscriptionSessionCreateResponseInputAudioTranscriptionModel>,
	/** The language of the input audio. Supplying the input language in
[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format
will improve accuracy and latency. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub language: Option<String>,
	/** An optional text to guide the model's style or continue a previous audio
segment. The [prompt](/docs/guides/speech-to-text#prompting) should match
the audio language. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<String>,
}

/** Configuration for turn detection. Can be set to `null` to turn off. Server 
VAD means that the model will detect the start and end of speech based on 
audio volume and respond at the end of user speech. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateResponseTurnDetection {
	#[serde(rename="type")]
	/** Type of turn detection, only `server_vad` is currently supported. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	/** Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A 
higher threshold will require louder audio to activate the model, and 
thus might perform better in noisy environments. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f32>,
	/** Amount of audio to include before the VAD detected speech (in 
milliseconds). Defaults to 300ms. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<u64>,
	/** Duration of silence to detect speech stop (in milliseconds). Defaults 
to 500ms. With shorter values the model will respond more quickly, 
but may jump in on short pauses from the user. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<u64>,
}

/** A new Realtime transcription session configuration.

When a session is created on the server via REST API, the session object
also contains an ephemeral key. Default TTL for keys is 10 minutes. This 
property is not present when a session is updated via the WebSocket API. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RealtimeTranscriptionSessionCreateResponse {
	/** Ephemeral key returned by the API. Only present when the session is
created on the server via REST API. */
	pub client_secret: RealtimeTranscriptionSessionCreateResponseClientSecret,
	/** The set of modalities the model can respond with. To disable audio,
set this to ["text"]. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub modalities: Option<Vec<String>>,
	/** The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_format: Option<String>,
	/** Configuration of the transcription model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_transcription: Option<RealtimeTranscriptionSessionCreateResponseInputAudioTranscription>,
	/** Configuration for turn detection. Can be set to `null` to turn off. Server 
VAD means that the model will detect the start and end of speech based on 
audio volume and respond at the end of user speech. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub turn_detection: Option<RealtimeTranscriptionSessionCreateResponseTurnDetection>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReasoningSummary {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="concise")]
	Concise,
	#[serde(rename="detailed")]
	Detailed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReasoningGenerateSummary {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="concise")]
	Concise,
	#[serde(rename="detailed")]
	Detailed,
}

/** **o-series models only**

Configuration options for 
[reasoning models](https://platform.openai.com/docs/guides/reasoning). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Reasoning {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub effort: Option<ReasoningEffort>,
	/** A summary of the reasoning performed by the model. This can be
useful for debugging and understanding the model's reasoning process.
One of `auto`, `concise`, or `detailed`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub summary: Option<ReasoningSummary>,
	/** **Deprecated:** use `summary` instead.

A summary of the reasoning performed by the model. This can be
useful for debugging and understanding the model's reasoning process.
One of `auto`, `concise`, or `detailed`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub generate_summary: Option<ReasoningGenerateSummary>,
}

/** **o-series models only** 

Constrains effort on reasoning for 
[reasoning models](https://platform.openai.com/docs/guides/reasoning).
Currently supported values are `low`, `medium`, and `high`. Reducing
reasoning effort can result in faster responses and fewer tokens used
on reasoning in a response. */
pub type ReasoningEffort = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReasoningItemType {
	#[serde(rename="reasoning")]
	Reasoning,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReasoningItemSummaryType {
	#[serde(rename="summary_text")]
	SummaryText,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReasoningItemSummary {
	#[serde(rename="type")]
	/** The type of the object. Always `summary_text`. */
	pub r#type: ReasoningItemSummaryType,
	/** A short summary of the reasoning used by the model when generating
the response. */
	pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ReasoningItemStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
}

/** A description of the chain of thought used by a reasoning model while generating
a response. Be sure to include these items in your `input` to the Responses API
for subsequent turns of a conversation if you are manually 
[managing context](/docs/guides/conversation-state). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ReasoningItem {
	#[serde(rename="type")]
	/** The type of the object. Always `reasoning`. */
	pub r#type: ReasoningItemType,
	/** The unique identifier of the reasoning content. */
	pub id: String,
	/** The encrypted content of the reasoning item - populated when a response is
generated with `reasoning.encrypted_content` in the `include` parameter. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub encrypted_content: Option<String>,
	/** Reasoning text contents. */
	pub summary: Vec<ReasoningItemSummary>,
	/** The status of the item. One of `in_progress`, `completed`, or
`incomplete`. Populated when items are returned via API. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<ReasoningItemStatus>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseObjectObject {
	#[serde(rename="response")]
	Response,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseObjectStatus {
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="failed")]
	Failed,
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="cancelled")]
	Cancelled,
	#[serde(rename="queued")]
	Queued,
	#[serde(rename="incomplete")]
	Incomplete,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseObjectIncompleteDetailsReason {
	#[serde(rename="max_output_tokens")]
	MaxOutputTokens,
	#[serde(rename="content_filter")]
	ContentFilter,
}

/** Details about why the response is incomplete. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseObjectIncompleteDetails {
	/** The reason why the response is incomplete. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<ResponseObjectIncompleteDetailsReason>,
}

/** A system (or developer) message inserted into the model's context.

When using along with `previous_response_id`, the instructions from a previous
response will not be carried over to the next response. This makes it simple
to swap out system (or developer) messages in new responses. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseObjectInstructions {
	/** A text input to the model, equivalent to a text input with the 
`developer` role. */
	String(String),
	/** A list of one or many input items to the model, containing 
different content types. */
	ArrayList(Vec<InputItem>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseObject {
	/** Unique identifier for this Response. */
	pub id: String,
	/** The object type of this resource - always set to `response`. */
	pub object: ResponseObjectObject,
	/** The status of the response generation. One of `completed`, `failed`, 
`in_progress`, `cancelled`, `queued`, or `incomplete`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<ResponseObjectStatus>,
	/** Unix timestamp (in seconds) of when this Response was created. */
	pub created_at: f32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<ResponseError>,
	/** Details about why the response is incomplete. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incomplete_details: Option<ResponseObjectIncompleteDetails>,
	/** An array of content items generated by the model.

- The length and order of items in the `output` array is dependent
  on the model's response.
- Rather than accessing the first item in the `output` array and 
  assuming it's an `assistant` message with the content generated by
  the model, you might consider using the `output_text` property where
  supported in SDKs. */
	pub output: Vec<OutputItem>,
	/** A system (or developer) message inserted into the model's context.

When using along with `previous_response_id`, the instructions from a previous
response will not be carried over to the next response. This makes it simple
to swap out system (or developer) messages in new responses. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub instructions: Option<ResponseObjectInstructions>,
	/** SDK-only convenience property that contains the aggregated text output 
from all `output_text` items in the `output` array, if any are present. 
Supported in the Python and JavaScript SDKs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_text: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<ResponseUsage>,
	/** Whether to allow the model to run tool calls in parallel. */
	pub parallel_tool_calls: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Response {
	#[serde(flatten)]
	pub model_response_properties: ModelResponseProperties,
	#[serde(flatten)]
	pub response_properties: ResponseProperties,
	#[serde(flatten)]
	pub object: ResponseObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseAudioDeltaEventType {
	#[serde(rename="response.audio.delta")]
	ResponseAudioDelta,
}

/** Emitted when there is a partial audio response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseAudioDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.audio.delta`. */
	pub r#type: ResponseAudioDeltaEventType,
	/** A sequence number for this chunk of the stream response. */
	pub sequence_number: u64,
	/** A chunk of Base64 encoded response audio bytes. */
	pub delta: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseAudioDoneEventType {
	#[serde(rename="response.audio.done")]
	ResponseAudioDone,
}

/** Emitted when the audio response is complete. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseAudioDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.audio.done`. */
	pub r#type: ResponseAudioDoneEventType,
	/** The sequence number of the delta. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseAudioTranscriptDeltaEventType {
	#[serde(rename="response.audio.transcript.delta")]
	ResponseAudioTranscriptDelta,
}

/** Emitted when there is a partial transcript of audio. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseAudioTranscriptDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.audio.transcript.delta`. */
	pub r#type: ResponseAudioTranscriptDeltaEventType,
	/** The partial transcript of the audio response. */
	pub delta: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseAudioTranscriptDoneEventType {
	#[serde(rename="response.audio.transcript.done")]
	ResponseAudioTranscriptDone,
}

/** Emitted when the full audio transcript is completed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseAudioTranscriptDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.audio.transcript.done`. */
	pub r#type: ResponseAudioTranscriptDoneEventType,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseCodeInterpreterCallCodeDeltaEventType {
	#[serde(rename="response.code_interpreter_call_code.delta")]
	ResponseCodeInterpreterCallCodeDelta,
}

/** Emitted when a partial code snippet is streamed by the code interpreter. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.code_interpreter_call_code.delta`. */
	pub r#type: ResponseCodeInterpreterCallCodeDeltaEventType,
	/** The index of the output item in the response for which the code is being streamed. */
	pub output_index: u64,
	/** The unique identifier of the code interpreter tool call item. */
	pub item_id: String,
	/** The partial code snippet being streamed by the code interpreter. */
	pub delta: String,
	/** The sequence number of this event, used to order streaming events. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseCodeInterpreterCallCodeDoneEventType {
	#[serde(rename="response.code_interpreter_call_code.done")]
	ResponseCodeInterpreterCallCodeDone,
}

/** Emitted when the code snippet is finalized by the code interpreter. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.code_interpreter_call_code.done`. */
	pub r#type: ResponseCodeInterpreterCallCodeDoneEventType,
	/** The index of the output item in the response for which the code is finalized. */
	pub output_index: u64,
	/** The unique identifier of the code interpreter tool call item. */
	pub item_id: String,
	/** The final code snippet output by the code interpreter. */
	pub code: String,
	/** The sequence number of this event, used to order streaming events. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseCodeInterpreterCallCompletedEventType {
	#[serde(rename="response.code_interpreter_call.completed")]
	ResponseCodeInterpreterCallCompleted,
}

/** Emitted when the code interpreter call is completed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseCodeInterpreterCallCompletedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.code_interpreter_call.completed`. */
	pub r#type: ResponseCodeInterpreterCallCompletedEventType,
	/** The index of the output item in the response for which the code interpreter call is completed. */
	pub output_index: u64,
	/** The unique identifier of the code interpreter tool call item. */
	pub item_id: String,
	/** The sequence number of this event, used to order streaming events. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseCodeInterpreterCallInProgressEventType {
	#[serde(rename="response.code_interpreter_call.in_progress")]
	ResponseCodeInterpreterCallInProgress,
}

/** Emitted when a code interpreter call is in progress. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseCodeInterpreterCallInProgressEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.code_interpreter_call.in_progress`. */
	pub r#type: ResponseCodeInterpreterCallInProgressEventType,
	/** The index of the output item in the response for which the code interpreter call is in progress. */
	pub output_index: u64,
	/** The unique identifier of the code interpreter tool call item. */
	pub item_id: String,
	/** The sequence number of this event, used to order streaming events. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseCodeInterpreterCallInterpretingEventType {
	#[serde(rename="response.code_interpreter_call.interpreting")]
	ResponseCodeInterpreterCallInterpreting,
}

/** Emitted when the code interpreter is actively interpreting the code snippet. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.code_interpreter_call.interpreting`. */
	pub r#type: ResponseCodeInterpreterCallInterpretingEventType,
	/** The index of the output item in the response for which the code interpreter is interpreting code. */
	pub output_index: u64,
	/** The unique identifier of the code interpreter tool call item. */
	pub item_id: String,
	/** The sequence number of this event, used to order streaming events. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseCompletedEventType {
	#[serde(rename="response.completed")]
	ResponseCompleted,
}

/** Emitted when the model response is complete. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseCompletedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.completed`. */
	pub r#type: ResponseCompletedEventType,
	/** Properties of the completed response. */
	pub response: Response,
	/** The sequence number for this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseContentPartAddedEventType {
	#[serde(rename="response.content_part.added")]
	ResponseContentPartAdded,
}

/** Emitted when a new content part is added. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseContentPartAddedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.content_part.added`. */
	pub r#type: ResponseContentPartAddedEventType,
	/** The ID of the output item that the content part was added to. */
	pub item_id: String,
	/** The index of the output item that the content part was added to. */
	pub output_index: u64,
	/** The index of the content part that was added. */
	pub content_index: u64,
	/** The content part that was added. */
	pub part: OutputContent,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseContentPartDoneEventType {
	#[serde(rename="response.content_part.done")]
	ResponseContentPartDone,
}

/** Emitted when a content part is done. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseContentPartDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.content_part.done`. */
	pub r#type: ResponseContentPartDoneEventType,
	/** The ID of the output item that the content part was added to. */
	pub item_id: String,
	/** The index of the output item that the content part was added to. */
	pub output_index: u64,
	/** The index of the content part that is done. */
	pub content_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The content part that is done. */
	pub part: OutputContent,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseCreatedEventType {
	#[serde(rename="response.created")]
	ResponseCreated,
}

/** An event that is emitted when a response is created. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseCreatedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.created`. */
	pub r#type: ResponseCreatedEventType,
	/** The response that was created. */
	pub response: Response,
	/** The sequence number for this event. */
	pub sequence_number: u64,
}

/** An error object returned when the model fails to generate a Response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseError {
	pub code: ResponseErrorCode,
	/** A human-readable description of the error. */
	pub message: String,
}

/** The error code for the response. */
pub type ResponseErrorCode = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseErrorEventType {
	#[serde(rename="error")]
	Error,
}

/** Emitted when an error occurs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseErrorEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `error`. */
	pub r#type: ResponseErrorEventType,
	/** The error code. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<String>,
	/** The error message. */
	pub message: String,
	/** The error parameter. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub param: Option<String>,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFailedEventType {
	#[serde(rename="response.failed")]
	ResponseFailed,
}

/** An event that is emitted when a response fails. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFailedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.failed`. */
	pub r#type: ResponseFailedEventType,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The response that failed. */
	pub response: Response,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFileSearchCallCompletedEventType {
	#[serde(rename="response.file_search_call.completed")]
	ResponseFileSearchCallCompleted,
}

/** Emitted when a file search call is completed (results found). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFileSearchCallCompletedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.file_search_call.completed`. */
	pub r#type: ResponseFileSearchCallCompletedEventType,
	/** The index of the output item that the file search call is initiated. */
	pub output_index: u64,
	/** The ID of the output item that the file search call is initiated. */
	pub item_id: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFileSearchCallInProgressEventType {
	#[serde(rename="response.file_search_call.in_progress")]
	ResponseFileSearchCallInProgress,
}

/** Emitted when a file search call is initiated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFileSearchCallInProgressEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.file_search_call.in_progress`. */
	pub r#type: ResponseFileSearchCallInProgressEventType,
	/** The index of the output item that the file search call is initiated. */
	pub output_index: u64,
	/** The ID of the output item that the file search call is initiated. */
	pub item_id: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFileSearchCallSearchingEventType {
	#[serde(rename="response.file_search_call.searching")]
	ResponseFileSearchCallSearching,
}

/** Emitted when a file search is currently searching. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFileSearchCallSearchingEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.file_search_call.searching`. */
	pub r#type: ResponseFileSearchCallSearchingEventType,
	/** The index of the output item that the file search call is searching. */
	pub output_index: u64,
	/** The ID of the output item that the file search call is initiated. */
	pub item_id: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFormatJsonObjectType {
	#[serde(rename="json_object")]
	JsonObject,
}

/** JSON object response format. An older method of generating JSON responses.
Using `json_schema` is recommended for models that support it. Note that the
model will not generate JSON without a system or user message instructing it
to do so. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFormatJsonObject {
	#[serde(rename="type")]
	/** The type of response format being defined. Always `json_object`. */
	pub r#type: ResponseFormatJsonObjectType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFormatJsonSchemaType {
	#[serde(rename="json_schema")]
	JsonSchema,
}

/** Structured Outputs configuration options, including a JSON Schema. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFormatJsonSchemaJsonSchema {
	/** A description of what the response format is for, used by the model to
determine how to respond in the format. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** The name of the response format. Must be a-z, A-Z, 0-9, or contain
underscores and dashes, with a maximum length of 64. */
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub schema: Option<ResponseFormatJsonSchemaSchema>,
	/** Whether to enable strict schema adherence when generating the output.
If set to true, the model will always follow the exact schema defined
in the `schema` field. Only a subset of JSON Schema is supported when
`strict` is `true`. To learn more, read the [Structured Outputs
guide](/docs/guides/structured-outputs). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strict: Option<bool>,
}

/** JSON Schema response format. Used to generate structured JSON responses.
Learn more about [Structured Outputs](/docs/guides/structured-outputs). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFormatJsonSchema {
	#[serde(rename="type")]
	/** The type of response format being defined. Always `json_schema`. */
	pub r#type: ResponseFormatJsonSchemaType,
	/** Structured Outputs configuration options, including a JSON Schema. */
	pub json_schema: ResponseFormatJsonSchemaJsonSchema,
}

/** The schema for the response format, described as a JSON Schema object.
Learn how to build JSON schemas [here](https://json-schema.org/). */
pub type ResponseFormatJsonSchemaSchema = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFormatTextType {
	#[serde(rename="text")]
	Text,
}

/** Default response format. Used to generate text responses. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFormatText {
	#[serde(rename="type")]
	/** The type of response format being defined. Always `text`. */
	pub r#type: ResponseFormatTextType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFunctionCallArgumentsDeltaEventType {
	#[serde(rename="response.function_call_arguments.delta")]
	ResponseFunctionCallArgumentsDelta,
}

/** Emitted when there is a partial function-call arguments delta. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.function_call_arguments.delta`. */
	pub r#type: ResponseFunctionCallArgumentsDeltaEventType,
	/** The ID of the output item that the function-call arguments delta is added to. */
	pub item_id: String,
	/** The index of the output item that the function-call arguments delta is added to. */
	pub output_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The function-call arguments delta that is added. */
	pub delta: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseFunctionCallArgumentsDoneEventType {
	#[serde(rename="response.function_call_arguments.done")]
	ResponseFunctionCallArgumentsDone,
}

/** Emitted when function-call arguments are finalized. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseFunctionCallArgumentsDoneEvent {
	#[serde(rename="type")]
	pub r#type: ResponseFunctionCallArgumentsDoneEventType,
	/** The ID of the item. */
	pub item_id: String,
	/** The index of the output item. */
	pub output_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The function-call arguments. */
	pub arguments: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseImageGenCallCompletedEventType {
	#[serde(rename="response.image_generation_call.completed")]
	ResponseImageGenerationCallCompleted,
}

/** Emitted when an image generation tool call has completed and the final image is available. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseImageGenCallCompletedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.image_generation_call.completed'. */
	pub r#type: ResponseImageGenCallCompletedEventType,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The unique identifier of the image generation item being processed. */
	pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseImageGenCallGeneratingEventType {
	#[serde(rename="response.image_generation_call.generating")]
	ResponseImageGenerationCallGenerating,
}

/** Emitted when an image generation tool call is actively generating an image (intermediate state). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseImageGenCallGeneratingEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.image_generation_call.generating'. */
	pub r#type: ResponseImageGenCallGeneratingEventType,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The unique identifier of the image generation item being processed. */
	pub item_id: String,
	/** The sequence number of the image generation item being processed. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseImageGenCallInProgressEventType {
	#[serde(rename="response.image_generation_call.in_progress")]
	ResponseImageGenerationCallInProgress,
}

/** Emitted when an image generation tool call is in progress. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseImageGenCallInProgressEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.image_generation_call.in_progress'. */
	pub r#type: ResponseImageGenCallInProgressEventType,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The unique identifier of the image generation item being processed. */
	pub item_id: String,
	/** The sequence number of the image generation item being processed. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseImageGenCallPartialImageEventType {
	#[serde(rename="response.image_generation_call.partial_image")]
	ResponseImageGenerationCallPartialImage,
}

/** Emitted when a partial image is available during image generation streaming. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseImageGenCallPartialImageEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.image_generation_call.partial_image'. */
	pub r#type: ResponseImageGenCallPartialImageEventType,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The unique identifier of the image generation item being processed. */
	pub item_id: String,
	/** The sequence number of the image generation item being processed. */
	pub sequence_number: u64,
	/** 0-based index for the partial image (backend is 1-based, but this is 0-based for the user). */
	pub partial_image_index: u64,
	/** Base64-encoded partial image data, suitable for rendering as an image. */
	pub partial_image_b64: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseInProgressEventType {
	#[serde(rename="response.in_progress")]
	ResponseInProgress,
}

/** Emitted when the response is in progress. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseInProgressEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.in_progress`. */
	pub r#type: ResponseInProgressEventType,
	/** The response that is in progress. */
	pub response: Response,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseIncompleteEventType {
	#[serde(rename="response.incomplete")]
	ResponseIncomplete,
}

/** An event that is emitted when a response finishes as incomplete. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseIncompleteEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.incomplete`. */
	pub r#type: ResponseIncompleteEventType,
	/** The response that was incomplete. */
	pub response: Response,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseItemListObject {
	#[serde(rename="list")]
	List,
}

/** A list of Response items. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseItemList {
	/** The type of object returned, must be `list`. */
	pub object: ResponseItemListObject,
	/** A list of items used to generate this response. */
	pub data: Vec<ItemResource>,
	/** Whether there are more items available. */
	pub has_more: bool,
	/** The ID of the first item in the list. */
	pub first_id: String,
	/** The ID of the last item in the list. */
	pub last_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseMCPCallArgumentsDeltaEventType {
	#[serde(rename="response.mcp_call.arguments_delta")]
	ResponseMcpCallArgumentsDelta,
}

	/** The partial update to the arguments for the MCP tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPCallArgumentsDeltaEventDelta(pub String);

/** Emitted when there is a delta (partial update) to the arguments of an MCP tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPCallArgumentsDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.mcp_call.arguments_delta'. */
	pub r#type: ResponseMCPCallArgumentsDeltaEventType,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The unique identifier of the MCP tool call item being processed. */
	pub item_id: String,
	/** The partial update to the arguments for the MCP tool call. */
	pub delta: ResponseMCPCallArgumentsDeltaEventDelta,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseMCPCallArgumentsDoneEventType {
	#[serde(rename="response.mcp_call.arguments_done")]
	ResponseMcpCallArgumentsDone,
}

	/** The finalized arguments for the MCP tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPCallArgumentsDoneEventArguments(pub String);

/** Emitted when the arguments for an MCP tool call are finalized. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPCallArgumentsDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.mcp_call.arguments_done'. */
	pub r#type: ResponseMCPCallArgumentsDoneEventType,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The unique identifier of the MCP tool call item being processed. */
	pub item_id: String,
	/** The finalized arguments for the MCP tool call. */
	pub arguments: ResponseMCPCallArgumentsDoneEventArguments,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseMCPCallCompletedEventType {
	#[serde(rename="response.mcp_call.completed")]
	ResponseMcpCallCompleted,
}

/** Emitted when an MCP  tool call has completed successfully. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPCallCompletedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.mcp_call.completed'. */
	pub r#type: ResponseMCPCallCompletedEventType,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseMCPCallFailedEventType {
	#[serde(rename="response.mcp_call.failed")]
	ResponseMcpCallFailed,
}

/** Emitted when an MCP  tool call has failed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPCallFailedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.mcp_call.failed'. */
	pub r#type: ResponseMCPCallFailedEventType,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseMCPCallInProgressEventType {
	#[serde(rename="response.mcp_call.in_progress")]
	ResponseMcpCallInProgress,
}

/** Emitted when an MCP  tool call is in progress. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPCallInProgressEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.mcp_call.in_progress'. */
	pub r#type: ResponseMCPCallInProgressEventType,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The unique identifier of the MCP tool call item being processed. */
	pub item_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseMCPListToolsCompletedEventType {
	#[serde(rename="response.mcp_list_tools.completed")]
	ResponseMcpListToolsCompleted,
}

/** Emitted when the list of available MCP tools has been successfully retrieved. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPListToolsCompletedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.mcp_list_tools.completed'. */
	pub r#type: ResponseMCPListToolsCompletedEventType,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseMCPListToolsFailedEventType {
	#[serde(rename="response.mcp_list_tools.failed")]
	ResponseMcpListToolsFailed,
}

/** Emitted when the attempt to list available MCP tools has failed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPListToolsFailedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.mcp_list_tools.failed'. */
	pub r#type: ResponseMCPListToolsFailedEventType,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseMCPListToolsInProgressEventType {
	#[serde(rename="response.mcp_list_tools.in_progress")]
	ResponseMcpListToolsInProgress,
}

/** Emitted when the system is in the process of retrieving the list of available MCP tools. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMCPListToolsInProgressEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.mcp_list_tools.in_progress'. */
	pub r#type: ResponseMCPListToolsInProgressEventType,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

/** Output types that you would like the model to generate.
Most models are capable of generating text, which is the default:

`["text"]`

The `gpt-4o-audio-preview` model can also be used to 
[generate audio](/docs/guides/audio). To request that this model generate 
both text and audio responses, you can use:

`["text", "audio"]` */
pub type ResponseModalities = Vec<String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseOutputItemAddedEventType {
	#[serde(rename="response.output_item.added")]
	ResponseOutputItemAdded,
}

/** Emitted when a new output item is added. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseOutputItemAddedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.output_item.added`. */
	pub r#type: ResponseOutputItemAddedEventType,
	/** The index of the output item that was added. */
	pub output_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The output item that was added. */
	pub item: OutputItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseOutputItemDoneEventType {
	#[serde(rename="response.output_item.done")]
	ResponseOutputItemDone,
}

/** Emitted when an output item is marked done. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseOutputItemDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.output_item.done`. */
	pub r#type: ResponseOutputItemDoneEventType,
	/** The index of the output item that was marked done. */
	pub output_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The output item that was marked done. */
	pub item: OutputItem,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseOutputTextAnnotationAddedEventType {
	#[serde(rename="response.output_text_annotation.added")]
	ResponseOutputTextAnnotationAdded,
}

	/** The annotation object being added. (See annotation schema for details.) */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseOutputTextAnnotationAddedEventAnnotation(pub String);

/** Emitted when an annotation is added to output text content. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseOutputTextAnnotationAddedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.output_text_annotation.added'. */
	pub r#type: ResponseOutputTextAnnotationAddedEventType,
	/** The unique identifier of the item to which the annotation is being added. */
	pub item_id: String,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The index of the content part within the output item. */
	pub content_index: u64,
	/** The index of the annotation within the content part. */
	pub annotation_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The annotation object being added. (See annotation schema for details.) */
	pub annotation: ResponseOutputTextAnnotationAddedEventAnnotation,
}

/** Optional map of values to substitute in for variables in your
prompt. The substitution values can either be strings, or other
Response input types like images or files. */
pub type ResponsePromptVariables = HashMap<String, String>;

/** Configuration options for a text response from the model. Can be plain
text or structured JSON data. Learn more:
- [Text inputs and outputs](/docs/guides/text)
- [Structured Outputs](/docs/guides/structured-outputs) */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponsePropertiesText {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub format: Option<TextResponseFormatConfiguration>,
}

/** How the model should select which tool (or tools) to use when generating
a response. See the `tools` parameter to see how to specify which tools
the model can call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponsePropertiesToolChoice {
	ToolChoiceOptions(ToolChoiceOptions),
	ToolChoiceTypes(ToolChoiceTypes),
	ToolChoiceFunction(ToolChoiceFunction),
	ToolChoiceMCP(ToolChoiceMCP),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponsePropertiesTruncation {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="disabled")]
	Disabled,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseProperties {
	/** The unique ID of the previous response to the model. Use this to
create multi-turn conversations. Learn more about 
[conversation state](/docs/guides/conversation-state). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub previous_response_id: Option<String>,
	/** Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI
offers a wide range of models with different capabilities, performance
characteristics, and price points. Refer to the [model guide](/docs/models)
to browse and compare available models. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<ModelIdsResponses>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reasoning: Option<Reasoning>,
	/** Whether to run the model response in the background. 
[Learn more](/docs/guides/background). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub background: Option<bool>,
	/** An upper bound for the number of tokens that can be generated for a response, including visible output tokens and [reasoning tokens](/docs/guides/reasoning). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_output_tokens: Option<u64>,
	/** The maximum number of total calls to built-in tools that can be processed in a response. This maximum number applies across all built-in tool calls, not per individual tool. Any further attempts to call a tool by the model will be ignored. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tool_calls: Option<u64>,
	/** Configuration options for a text response from the model. Can be plain
text or structured JSON data. Learn more:
- [Text inputs and outputs](/docs/guides/text)
- [Structured Outputs](/docs/guides/structured-outputs) */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<ResponsePropertiesText>,
	/** An array of tools the model may call while generating a response. You 
can specify which tool to use by setting the `tool_choice` parameter.

The two categories of tools you can provide the model are:

- **Built-in tools**: Tools that are provided by OpenAI that extend the
  model's capabilities, like [web search](/docs/guides/tools-web-search)
  or [file search](/docs/guides/tools-file-search). Learn more about
  [built-in tools](/docs/guides/tools).
- **Function calls (custom tools)**: Functions that are defined by you,
  enabling the model to call your own code. Learn more about
  [function calling](/docs/guides/function-calling). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tools: Option<Vec<Tool>>,
	/** How the model should select which tool (or tools) to use when generating
a response. See the `tools` parameter to see how to specify which tools
the model can call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_choice: Option<ResponsePropertiesToolChoice>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prompt: Option<Prompt>,
	/** The truncation strategy to use for the model response.
- `auto`: If the context of this response and previous ones exceeds
  the model's context window size, the model will truncate the 
  response to fit the context window by dropping input items in the
  middle of the conversation. 
- `disabled` (default): If a model response will exceed the context window 
  size for a model, the request will fail with a 400 error. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub truncation: Option<ResponsePropertiesTruncation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseQueuedEventType {
	#[serde(rename="response.queued")]
	ResponseQueued,
}

/** Emitted when a response is queued and waiting to be processed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseQueuedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.queued'. */
	pub r#type: ResponseQueuedEventType,
	/** The full response object that is queued. */
	pub response: Response,
	/** The sequence number for this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningDeltaEventType {
	#[serde(rename="response.reasoning.delta")]
	ResponseReasoningDelta,
}

	/** The partial update to the reasoning content. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningDeltaEventDelta(pub String);

/** Emitted when there is a delta (partial update) to the reasoning content. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.reasoning.delta'. */
	pub r#type: ResponseReasoningDeltaEventType,
	/** The unique identifier of the item for which reasoning is being updated. */
	pub item_id: String,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The index of the reasoning content part within the output item. */
	pub content_index: u64,
	/** The partial update to the reasoning content. */
	pub delta: ResponseReasoningDeltaEventDelta,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningDoneEventType {
	#[serde(rename="response.reasoning.done")]
	ResponseReasoningDone,
}

/** Emitted when the reasoning content is finalized for an item. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.reasoning.done'. */
	pub r#type: ResponseReasoningDoneEventType,
	/** The unique identifier of the item for which reasoning is finalized. */
	pub item_id: String,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The index of the reasoning content part within the output item. */
	pub content_index: u64,
	/** The finalized reasoning text. */
	pub text: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningSummaryDeltaEventType {
	#[serde(rename="response.reasoning_summary.delta")]
	ResponseReasoningSummaryDelta,
}

	/** The partial update to the reasoning summary content. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryDeltaEventDelta(pub String);

/** Emitted when there is a delta (partial update) to the reasoning summary content. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.reasoning_summary.delta'. */
	pub r#type: ResponseReasoningSummaryDeltaEventType,
	/** The unique identifier of the item for which the reasoning summary is being updated. */
	pub item_id: String,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The index of the summary part within the output item. */
	pub summary_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The partial update to the reasoning summary content. */
	pub delta: ResponseReasoningSummaryDeltaEventDelta,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningSummaryDoneEventType {
	#[serde(rename="response.reasoning_summary.done")]
	ResponseReasoningSummaryDone,
}

/** Emitted when the reasoning summary content is finalized for an item. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always 'response.reasoning_summary.done'. */
	pub r#type: ResponseReasoningSummaryDoneEventType,
	/** The unique identifier of the item for which the reasoning summary is finalized. */
	pub item_id: String,
	/** The index of the output item in the response's output array. */
	pub output_index: u64,
	/** The index of the summary part within the output item. */
	pub summary_index: u64,
	/** The finalized reasoning summary text. */
	pub text: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningSummaryPartAddedEventType {
	#[serde(rename="response.reasoning_summary_part.added")]
	ResponseReasoningSummaryPartAdded,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningSummaryPartAddedEventPartType {
	#[serde(rename="summary_text")]
	SummaryText,
}

/** The summary part that was added. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryPartAddedEventPart {
	#[serde(rename="type")]
	/** The type of the summary part. Always `summary_text`. */
	pub r#type: ResponseReasoningSummaryPartAddedEventPartType,
	/** The text of the summary part. */
	pub text: String,
}

/** Emitted when a new reasoning summary part is added. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryPartAddedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.reasoning_summary_part.added`. */
	pub r#type: ResponseReasoningSummaryPartAddedEventType,
	/** The ID of the item this summary part is associated with. */
	pub item_id: String,
	/** The index of the output item this summary part is associated with. */
	pub output_index: u64,
	/** The index of the summary part within the reasoning summary. */
	pub summary_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The summary part that was added. */
	pub part: ResponseReasoningSummaryPartAddedEventPart,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningSummaryPartDoneEventType {
	#[serde(rename="response.reasoning_summary_part.done")]
	ResponseReasoningSummaryPartDone,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningSummaryPartDoneEventPartType {
	#[serde(rename="summary_text")]
	SummaryText,
}

/** The completed summary part. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryPartDoneEventPart {
	#[serde(rename="type")]
	/** The type of the summary part. Always `summary_text`. */
	pub r#type: ResponseReasoningSummaryPartDoneEventPartType,
	/** The text of the summary part. */
	pub text: String,
}

/** Emitted when a reasoning summary part is completed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryPartDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.reasoning_summary_part.done`. */
	pub r#type: ResponseReasoningSummaryPartDoneEventType,
	/** The ID of the item this summary part is associated with. */
	pub item_id: String,
	/** The index of the output item this summary part is associated with. */
	pub output_index: u64,
	/** The index of the summary part within the reasoning summary. */
	pub summary_index: u64,
	/** The sequence number of this event. */
	pub sequence_number: u64,
	/** The completed summary part. */
	pub part: ResponseReasoningSummaryPartDoneEventPart,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningSummaryTextDeltaEventType {
	#[serde(rename="response.reasoning_summary_text.delta")]
	ResponseReasoningSummaryTextDelta,
}

/** Emitted when a delta is added to a reasoning summary text. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryTextDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.reasoning_summary_text.delta`. */
	pub r#type: ResponseReasoningSummaryTextDeltaEventType,
	/** The ID of the item this summary text delta is associated with. */
	pub item_id: String,
	/** The index of the output item this summary text delta is associated with. */
	pub output_index: u64,
	/** The index of the summary part within the reasoning summary. */
	pub summary_index: u64,
	/** The text delta that was added to the summary. */
	pub delta: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseReasoningSummaryTextDoneEventType {
	#[serde(rename="response.reasoning_summary_text.done")]
	ResponseReasoningSummaryTextDone,
}

/** Emitted when a reasoning summary text is completed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryTextDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.reasoning_summary_text.done`. */
	pub r#type: ResponseReasoningSummaryTextDoneEventType,
	/** The ID of the item this summary text is associated with. */
	pub item_id: String,
	/** The index of the output item this summary text is associated with. */
	pub output_index: u64,
	/** The index of the summary part within the reasoning summary. */
	pub summary_index: u64,
	/** The full text of the completed reasoning summary. */
	pub text: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseRefusalDeltaEventType {
	#[serde(rename="response.refusal.delta")]
	ResponseRefusalDelta,
}

/** Emitted when there is a partial refusal text. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseRefusalDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.refusal.delta`. */
	pub r#type: ResponseRefusalDeltaEventType,
	/** The ID of the output item that the refusal text is added to. */
	pub item_id: String,
	/** The index of the output item that the refusal text is added to. */
	pub output_index: u64,
	/** The index of the content part that the refusal text is added to. */
	pub content_index: u64,
	/** The refusal text that is added. */
	pub delta: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseRefusalDoneEventType {
	#[serde(rename="response.refusal.done")]
	ResponseRefusalDone,
}

/** Emitted when refusal text is finalized. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseRefusalDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.refusal.done`. */
	pub r#type: ResponseRefusalDoneEventType,
	/** The ID of the output item that the refusal text is finalized. */
	pub item_id: String,
	/** The index of the output item that the refusal text is finalized. */
	pub output_index: u64,
	/** The index of the content part that the refusal text is finalized. */
	pub content_index: u64,
	/** The refusal text that is finalized. */
	pub refusal: String,
	/** The sequence number of this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseStreamEvent {
	ResponseAudioDeltaEvent(ResponseAudioDeltaEvent),
	ResponseAudioDoneEvent(ResponseAudioDoneEvent),
	ResponseAudioTranscriptDeltaEvent(ResponseAudioTranscriptDeltaEvent),
	ResponseAudioTranscriptDoneEvent(ResponseAudioTranscriptDoneEvent),
	ResponseCodeInterpreterCallCodeDeltaEvent(ResponseCodeInterpreterCallCodeDeltaEvent),
	ResponseCodeInterpreterCallCodeDoneEvent(ResponseCodeInterpreterCallCodeDoneEvent),
	ResponseCodeInterpreterCallCompletedEvent(ResponseCodeInterpreterCallCompletedEvent),
	ResponseCodeInterpreterCallInProgressEvent(ResponseCodeInterpreterCallInProgressEvent),
	ResponseCodeInterpreterCallInterpretingEvent(ResponseCodeInterpreterCallInterpretingEvent),
	ResponseCompletedEvent(ResponseCompletedEvent),
	ResponseContentPartAddedEvent(ResponseContentPartAddedEvent),
	ResponseContentPartDoneEvent(ResponseContentPartDoneEvent),
	ResponseCreatedEvent(ResponseCreatedEvent),
	ResponseErrorEvent(ResponseErrorEvent),
	ResponseFileSearchCallCompletedEvent(ResponseFileSearchCallCompletedEvent),
	ResponseFileSearchCallInProgressEvent(ResponseFileSearchCallInProgressEvent),
	ResponseFileSearchCallSearchingEvent(ResponseFileSearchCallSearchingEvent),
	ResponseFunctionCallArgumentsDeltaEvent(ResponseFunctionCallArgumentsDeltaEvent),
	ResponseFunctionCallArgumentsDoneEvent(ResponseFunctionCallArgumentsDoneEvent),
	ResponseInProgressEvent(ResponseInProgressEvent),
	ResponseFailedEvent(ResponseFailedEvent),
	ResponseIncompleteEvent(ResponseIncompleteEvent),
	ResponseOutputItemAddedEvent(ResponseOutputItemAddedEvent),
	ResponseOutputItemDoneEvent(ResponseOutputItemDoneEvent),
	ResponseReasoningSummaryPartAddedEvent(ResponseReasoningSummaryPartAddedEvent),
	ResponseReasoningSummaryPartDoneEvent(ResponseReasoningSummaryPartDoneEvent),
	ResponseReasoningSummaryTextDeltaEvent(ResponseReasoningSummaryTextDeltaEvent),
	ResponseReasoningSummaryTextDoneEvent(ResponseReasoningSummaryTextDoneEvent),
	ResponseRefusalDeltaEvent(ResponseRefusalDeltaEvent),
	ResponseRefusalDoneEvent(ResponseRefusalDoneEvent),
	ResponseTextDeltaEvent(ResponseTextDeltaEvent),
	ResponseTextDoneEvent(ResponseTextDoneEvent),
	ResponseWebSearchCallCompletedEvent(ResponseWebSearchCallCompletedEvent),
	ResponseWebSearchCallInProgressEvent(ResponseWebSearchCallInProgressEvent),
	ResponseWebSearchCallSearchingEvent(ResponseWebSearchCallSearchingEvent),
	ResponseImageGenCallCompletedEvent(ResponseImageGenCallCompletedEvent),
	ResponseImageGenCallGeneratingEvent(ResponseImageGenCallGeneratingEvent),
	ResponseImageGenCallInProgressEvent(ResponseImageGenCallInProgressEvent),
	ResponseImageGenCallPartialImageEvent(ResponseImageGenCallPartialImageEvent),
	ResponseMCPCallArgumentsDeltaEvent(ResponseMCPCallArgumentsDeltaEvent),
	ResponseMCPCallArgumentsDoneEvent(ResponseMCPCallArgumentsDoneEvent),
	ResponseMCPCallCompletedEvent(ResponseMCPCallCompletedEvent),
	ResponseMCPCallFailedEvent(ResponseMCPCallFailedEvent),
	ResponseMCPCallInProgressEvent(ResponseMCPCallInProgressEvent),
	ResponseMCPListToolsCompletedEvent(ResponseMCPListToolsCompletedEvent),
	ResponseMCPListToolsFailedEvent(ResponseMCPListToolsFailedEvent),
	ResponseMCPListToolsInProgressEvent(ResponseMCPListToolsInProgressEvent),
	ResponseOutputTextAnnotationAddedEvent(ResponseOutputTextAnnotationAddedEvent),
	ResponseQueuedEvent(ResponseQueuedEvent),
	ResponseReasoningDeltaEvent(ResponseReasoningDeltaEvent),
	ResponseReasoningDoneEvent(ResponseReasoningDoneEvent),
	ResponseReasoningSummaryDeltaEvent(ResponseReasoningSummaryDeltaEvent),
	ResponseReasoningSummaryDoneEvent(ResponseReasoningSummaryDoneEvent),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseTextDeltaEventType {
	#[serde(rename="response.output_text.delta")]
	ResponseOutputTextDelta,
}

/** Emitted when there is an additional text delta. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseTextDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.output_text.delta`. */
	pub r#type: ResponseTextDeltaEventType,
	/** The ID of the output item that the text delta was added to. */
	pub item_id: String,
	/** The index of the output item that the text delta was added to. */
	pub output_index: u64,
	/** The index of the content part that the text delta was added to. */
	pub content_index: u64,
	/** The text delta that was added. */
	pub delta: String,
	/** The sequence number for this event. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseTextDoneEventType {
	#[serde(rename="response.output_text.done")]
	ResponseOutputTextDone,
}

/** Emitted when text content is finalized. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseTextDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.output_text.done`. */
	pub r#type: ResponseTextDoneEventType,
	/** The ID of the output item that the text content is finalized. */
	pub item_id: String,
	/** The index of the output item that the text content is finalized. */
	pub output_index: u64,
	/** The index of the content part that the text content is finalized. */
	pub content_index: u64,
	/** The text content that is finalized. */
	pub text: String,
	/** The sequence number for this event. */
	pub sequence_number: u64,
}

/** A detailed breakdown of the input tokens. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseUsageInputTokensDetails {
	/** The number of tokens that were retrieved from the cache. 
[More on prompt caching](/docs/guides/prompt-caching). */
	pub cached_tokens: u64,
}

/** A detailed breakdown of the output tokens. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseUsageOutputTokensDetails {
	/** The number of reasoning tokens. */
	pub reasoning_tokens: u64,
}

/** Represents token usage details including input tokens, output tokens,
a breakdown of output tokens, and the total tokens used. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseUsage {
	/** The number of input tokens. */
	pub input_tokens: u64,
	/** A detailed breakdown of the input tokens. */
	pub input_tokens_details: ResponseUsageInputTokensDetails,
	/** The number of output tokens. */
	pub output_tokens: u64,
	/** A detailed breakdown of the output tokens. */
	pub output_tokens_details: ResponseUsageOutputTokensDetails,
	/** The total number of tokens used. */
	pub total_tokens: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseWebSearchCallCompletedEventType {
	#[serde(rename="response.web_search_call.completed")]
	ResponseWebSearchCallCompleted,
}

/** Emitted when a web search call is completed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWebSearchCallCompletedEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.web_search_call.completed`. */
	pub r#type: ResponseWebSearchCallCompletedEventType,
	/** The index of the output item that the web search call is associated with. */
	pub output_index: u64,
	/** Unique ID for the output item associated with the web search call. */
	pub item_id: String,
	/** The sequence number of the web search call being processed. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseWebSearchCallInProgressEventType {
	#[serde(rename="response.web_search_call.in_progress")]
	ResponseWebSearchCallInProgress,
}

/** Emitted when a web search call is initiated. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWebSearchCallInProgressEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.web_search_call.in_progress`. */
	pub r#type: ResponseWebSearchCallInProgressEventType,
	/** The index of the output item that the web search call is associated with. */
	pub output_index: u64,
	/** Unique ID for the output item associated with the web search call. */
	pub item_id: String,
	/** The sequence number of the web search call being processed. */
	pub sequence_number: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ResponseWebSearchCallSearchingEventType {
	#[serde(rename="response.web_search_call.searching")]
	ResponseWebSearchCallSearching,
}

/** Emitted when a web search call is executing. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseWebSearchCallSearchingEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `response.web_search_call.searching`. */
	pub r#type: ResponseWebSearchCallSearchingEventType,
	/** The index of the output item that the web search call is associated with. */
	pub output_index: u64,
	/** Unique ID for the output item associated with the web search call. */
	pub item_id: String,
	/** The sequence number of the web search call being processed. */
	pub sequence_number: u64,
}

/** Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunCompletionUsage {
	/** Number of completion tokens used over the course of the run. */
	pub completion_tokens: u64,
	/** Number of prompt tokens used over the course of the run. */
	pub prompt_tokens: u64,
	/** Total number of tokens used (prompt + completion). */
	pub total_tokens: u64,
}

	/** The grader used for the fine-tuning job. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderRequestGrader(pub String);

	/** The dataset item provided to the grader. This will be used to populate 
the `item` namespace. See [the guide](/docs/guides/graders) for more details. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderRequestItem(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderRequest {
	/** The grader used for the fine-tuning job. */
	pub grader: RunGraderRequestGrader,
	/** The dataset item provided to the grader. This will be used to populate 
the `item` namespace. See [the guide](/docs/guides/graders) for more details. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub item: Option<RunGraderRequestItem>,
	/** The model sample to be evaluated. This value will be used to populate 
the `sample` namespace. See [the guide](/docs/guides/graders) for more details.
The `output_json` variable will be populated if the model sample is a 
valid JSON string. */
	pub model_sample: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderResponseMetadataErrors {
	pub formula_parse_error: bool,
	pub sample_parse_error: bool,
	pub truncated_observation_error: bool,
	pub unresponsive_reward_error: bool,
	pub invalid_variable_error: bool,
	pub other_error: bool,
	pub python_grader_server_error: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub python_grader_server_error_type: Option<String>,
	pub python_grader_runtime_error: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub python_grader_runtime_error_details: Option<String>,
	pub model_grader_server_error: bool,
	pub model_grader_refusal_error: bool,
	pub model_grader_parse_error: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model_grader_server_error_details: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderResponseMetadataScores(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderResponseMetadata {
	pub name: String,
	#[serde(rename="type")]
	pub r#type: String,
	pub errors: RunGraderResponseMetadataErrors,
	pub execution_time: f32,
	pub scores: RunGraderResponseMetadataScores,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token_usage: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sampled_model_name: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderResponseSubRewards(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderResponseModelGraderTokenUsagePerModel(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunGraderResponse {
	pub reward: f32,
	pub metadata: RunGraderResponseMetadata,
	pub sub_rewards: RunGraderResponseSubRewards,
	pub model_grader_token_usage_per_model: RunGraderResponseModelGraderTokenUsagePerModel,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunObjectObject {
	#[serde(rename="thread.run")]
	ThreadRun,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunObjectStatus {
	#[serde(rename="queued")]
	Queued,
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="requires_action")]
	RequiresAction,
	#[serde(rename="cancelling")]
	Cancelling,
	#[serde(rename="cancelled")]
	Cancelled,
	#[serde(rename="failed")]
	Failed,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
	#[serde(rename="expired")]
	Expired,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunObjectRequiredActionType {
	#[serde(rename="submit_tool_outputs")]
	SubmitToolOutputs,
}

/** Details on the tool outputs needed for this run to continue. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunObjectRequiredActionSubmitToolOutputs {
	/** A list of the relevant tool calls. */
	pub tool_calls: Vec<RunToolCallObject>,
}

/** Details on the action required to continue the run. Will be `null` if no action is required. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunObjectRequiredAction {
	#[serde(rename="type")]
	/** For now, this is always `submit_tool_outputs`. */
	pub r#type: RunObjectRequiredActionType,
	/** Details on the tool outputs needed for this run to continue. */
	pub submit_tool_outputs: RunObjectRequiredActionSubmitToolOutputs,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunObjectLastErrorCode {
	#[serde(rename="server_error")]
	ServerError,
	#[serde(rename="rate_limit_exceeded")]
	RateLimitExceeded,
	#[serde(rename="invalid_prompt")]
	InvalidPrompt,
}

/** The last error associated with this run. Will be `null` if there are no errors. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunObjectLastError {
	/** One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`. */
	pub code: RunObjectLastErrorCode,
	/** A human-readable description of the error. */
	pub message: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunObjectIncompleteDetailsReason {
	#[serde(rename="max_completion_tokens")]
	MaxCompletionTokens,
	#[serde(rename="max_prompt_tokens")]
	MaxPromptTokens,
}

/** Details on why the run is incomplete. Will be `null` if the run is not incomplete. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunObjectIncompleteDetails {
	/** The reason why the run is incomplete. This will point to which specific token limit was reached over the course of the run. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reason: Option<RunObjectIncompleteDetailsReason>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunObjectTools {
	AssistantToolsCode(AssistantToolsCode),
	AssistantToolsFileSearch(AssistantToolsFileSearch),
	AssistantToolsFunction(AssistantToolsFunction),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunObjectTruncationStrategy {
	#[serde(flatten)]
	pub truncation_object: TruncationObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunObjectToolChoice {
	#[serde(flatten)]
	pub assistants_api_tool_choice_option: AssistantsApiToolChoiceOption,
}

/** Represents an execution run on a [thread](/docs/api-reference/threads). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunObject {
	/** The identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `thread.run`. */
	pub object: RunObjectObject,
	/** The Unix timestamp (in seconds) for when the run was created. */
	pub created_at: u64,
	/** The ID of the [thread](/docs/api-reference/threads) that was executed on as a part of this run. */
	pub thread_id: String,
	/** The ID of the [assistant](/docs/api-reference/assistants) used for execution of this run. */
	pub assistant_id: String,
	/** The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`. */
	pub status: RunObjectStatus,
	/** Details on the action required to continue the run. Will be `null` if no action is required. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub required_action: Option<RunObjectRequiredAction>,
	/** The last error associated with this run. Will be `null` if there are no errors. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_error: Option<RunObjectLastError>,
	/** The Unix timestamp (in seconds) for when the run will expire. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the run was started. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub started_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the run was cancelled. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cancelled_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the run failed. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub failed_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the run was completed. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completed_at: Option<u64>,
	/** Details on why the run is incomplete. Will be `null` if the run is not incomplete. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incomplete_details: Option<RunObjectIncompleteDetails>,
	/** The model that the [assistant](/docs/api-reference/assistants) used for this run. */
	pub model: String,
	/** The instructions that the [assistant](/docs/api-reference/assistants) used for this run. */
	pub instructions: String,
	/** The list of tools that the [assistant](/docs/api-reference/assistants) used for this run. */
	pub tools: Vec<RunObjectTools>,
	pub metadata: Metadata,
	pub usage: RunCompletionUsage,
	/** The sampling temperature used for this run. If not set, defaults to 1. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub temperature: Option<f32>,
	/** The nucleus sampling value used for this run. If not set, defaults to 1. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub top_p: Option<f32>,
	/** The maximum number of prompt tokens specified to have been used over the course of the run. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_prompt_tokens: Option<u64>,
	/** The maximum number of completion tokens specified to have been used over the course of the run. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_completion_tokens: Option<u64>,
	pub truncation_strategy: RunObjectTruncationStrategy,
	pub tool_choice: RunObjectToolChoice,
	pub parallel_tool_calls: ParallelToolCalls,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub response_format: Option<AssistantsApiResponseFormatOption>,
}

/** Usage statistics related to the run step. This value will be `null` while the run step's status is `in_progress`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepCompletionUsage {
	/** Number of completion tokens used over the course of the run step. */
	pub completion_tokens: u64,
	/** Number of prompt tokens used over the course of the run step. */
	pub prompt_tokens: u64,
	/** Total number of tokens used (prompt + completion). */
	pub total_tokens: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDeltaObjectObject {
	#[serde(rename="thread.run.step.delta")]
	ThreadRunStepDelta,
}

	/** The details of the run step. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaObjectDeltaStepDetails(pub String);

/** The delta containing the fields that have changed on the run step. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaObjectDelta {
	/** The details of the run step. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub step_details: Option<RunStepDeltaObjectDeltaStepDetails>,
}

/** Represents a run step delta i.e. any changed fields on a run step during streaming. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaObject {
	/** The identifier of the run step, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `thread.run.step.delta`. */
	pub object: RunStepDeltaObjectObject,
	/** The delta containing the fields that have changed on the run step. */
	pub delta: RunStepDeltaObjectDelta,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDeltaStepDetailsMessageCreationObjectType {
	#[serde(rename="message_creation")]
	MessageCreation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
	/** The ID of the message that was created by this run step. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_id: Option<String>,
}

/** Details of the message creation by the run step. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsMessageCreationObject {
	#[serde(rename="type")]
	/** Always `message_creation`. */
	pub r#type: RunStepDeltaStepDetailsMessageCreationObjectType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message_creation: Option<RunStepDeltaStepDetailsMessageCreationObjectMessageCreation>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDeltaStepDetailsToolCallsCodeObjectType {
	#[serde(rename="code_interpreter")]
	CodeInterpreter,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputs(pub String);

/** The Code Interpreter tool call definition. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
	/** The input to the Code Interpreter tool call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input: Option<String>,
	/** The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub outputs: Option<Vec<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputs>>,
}

/** Details of the Code Interpreter tool call the run step was involved in. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObject {
	/** The index of the tool call in the tool calls array. */
	pub index: u64,
	/** The ID of the tool call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of tool call. This is always going to be `code_interpreter` for this type of tool call. */
	pub r#type: RunStepDeltaStepDetailsToolCallsCodeObjectType,
	/** The Code Interpreter tool call definition. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType {
	#[serde(rename="image")]
	Image,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
	/** The [file](/docs/api-reference/files) ID of the image. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
	/** The index of the output in the outputs array. */
	pub index: u64,
	#[serde(rename="type")]
	/** Always `image`. */
	pub r#type: RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image: Option<RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType {
	#[serde(rename="logs")]
	Logs,
}

/** Text output from the Code Interpreter tool call as part of a run step. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
	/** The index of the output in the outputs array. */
	pub index: u64,
	#[serde(rename="type")]
	/** Always `logs`. */
	pub r#type: RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType,
	/** The text output from the Code Interpreter tool call. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logs: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDeltaStepDetailsToolCallsFileSearchObjectType {
	#[serde(rename="file_search")]
	FileSearch,
}

	/** For now, this is always going to be an empty object. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObjectFileSearch(pub HashMap<String,String>);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
	/** The index of the tool call in the tool calls array. */
	pub index: u64,
	/** The ID of the tool call object. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of tool call. This is always going to be `file_search` for this type of tool call. */
	pub r#type: RunStepDeltaStepDetailsToolCallsFileSearchObjectType,
	/** For now, this is always going to be an empty object. */
	pub file_search: RunStepDeltaStepDetailsToolCallsFileSearchObjectFileSearch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDeltaStepDetailsToolCallsFunctionObjectType {
	#[serde(rename="function")]
	Function,
}

/** The definition of the function that was called. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
	/** The name of the function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The arguments passed to the function. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub arguments: Option<String>,
	/** The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
	/** The index of the tool call in the tool calls array. */
	pub index: u64,
	/** The ID of the tool call object. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename="type")]
	/** The type of tool call. This is always going to be `function` for this type of tool call. */
	pub r#type: RunStepDeltaStepDetailsToolCallsFunctionObjectType,
	/** The definition of the function that was called. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function: Option<RunStepDeltaStepDetailsToolCallsFunctionObjectFunction>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDeltaStepDetailsToolCallsObjectType {
	#[serde(rename="tool_calls")]
	ToolCalls,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStepDeltaStepDetailsToolCallsObjectToolCalls {
	RunStepDeltaStepDetailsToolCallsCodeObject(RunStepDeltaStepDetailsToolCallsCodeObject),
	RunStepDeltaStepDetailsToolCallsFileSearchObject(RunStepDeltaStepDetailsToolCallsFileSearchObject),
	RunStepDeltaStepDetailsToolCallsFunctionObject(RunStepDeltaStepDetailsToolCallsFunctionObject),
}

/** Details of the tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDeltaStepDetailsToolCallsObject {
	#[serde(rename="type")]
	/** Always `tool_calls`. */
	pub r#type: RunStepDeltaStepDetailsToolCallsObjectType,
	/** An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_calls: Option<Vec<RunStepDeltaStepDetailsToolCallsObjectToolCalls>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDetailsMessageCreationObjectType {
	#[serde(rename="message_creation")]
	MessageCreation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsMessageCreationObjectMessageCreation {
	/** The ID of the message that was created by this run step. */
	pub message_id: String,
}

/** Details of the message creation by the run step. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsMessageCreationObject {
	#[serde(rename="type")]
	/** Always `message_creation`. */
	pub r#type: RunStepDetailsMessageCreationObjectType,
	pub message_creation: RunStepDetailsMessageCreationObjectMessageCreation,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDetailsToolCallsCodeObjectType {
	#[serde(rename="code_interpreter")]
	CodeInterpreter,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputs(pub String);

/** The Code Interpreter tool call definition. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsCodeObjectCodeInterpreter {
	/** The input to the Code Interpreter tool call. */
	pub input: String,
	/** The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type. */
	pub outputs: Vec<RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputs>,
}

/** Details of the Code Interpreter tool call the run step was involved in. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsCodeObject {
	/** The ID of the tool call. */
	pub id: String,
	#[serde(rename="type")]
	/** The type of tool call. This is always going to be `code_interpreter` for this type of tool call. */
	pub r#type: RunStepDetailsToolCallsCodeObjectType,
	/** The Code Interpreter tool call definition. */
	pub code_interpreter: RunStepDetailsToolCallsCodeObjectCodeInterpreter,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDetailsToolCallsCodeOutputImageObjectType {
	#[serde(rename="image")]
	Image,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsCodeOutputImageObjectImage {
	/** The [file](/docs/api-reference/files) ID of the image. */
	pub file_id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsCodeOutputImageObject {
	#[serde(rename="type")]
	/** Always `image`. */
	pub r#type: RunStepDetailsToolCallsCodeOutputImageObjectType,
	pub image: RunStepDetailsToolCallsCodeOutputImageObjectImage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDetailsToolCallsCodeOutputLogsObjectType {
	#[serde(rename="logs")]
	Logs,
}

/** Text output from the Code Interpreter tool call as part of a run step. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
	#[serde(rename="type")]
	/** Always `logs`. */
	pub r#type: RunStepDetailsToolCallsCodeOutputLogsObjectType,
	/** The text output from the Code Interpreter tool call. */
	pub logs: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDetailsToolCallsFileSearchObjectType {
	#[serde(rename="file_search")]
	FileSearch,
}

/** For now, this is always going to be an empty object. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFileSearchObjectFileSearch {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranking_options: Option<RunStepDetailsToolCallsFileSearchRankingOptionsObject>,
	/** The results of the file search. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub results: Option<Vec<RunStepDetailsToolCallsFileSearchResultObject>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFileSearchObject {
	/** The ID of the tool call object. */
	pub id: String,
	#[serde(rename="type")]
	/** The type of tool call. This is always going to be `file_search` for this type of tool call. */
	pub r#type: RunStepDetailsToolCallsFileSearchObjectType,
	/** For now, this is always going to be an empty object. */
	pub file_search: RunStepDetailsToolCallsFileSearchObjectFileSearch,
}

/** The ranking options for the file search. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
	pub ranker: FileSearchRanker,
	/** The score threshold for the file search. All values must be a floating point number between 0 and 1. */
	pub score_threshold: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDetailsToolCallsFileSearchResultObjectContentType {
	#[serde(rename="text")]
	Text,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFileSearchResultObjectContent {
	#[serde(rename="type")]
	/** The type of the content. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<RunStepDetailsToolCallsFileSearchResultObjectContentType>,
	/** The text content of the file. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
}

/** A result instance of the file search. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFileSearchResultObject {
	/** The ID of the file that result was found in. */
	pub file_id: String,
	/** The name of the file that result was found in. */
	pub file_name: String,
	/** The score of the result. All values must be a floating point number between 0 and 1. */
	pub score: f32,
	/** The content of the result that was found. The content is only included if requested via the include query parameter. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub content: Option<Vec<RunStepDetailsToolCallsFileSearchResultObjectContent>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDetailsToolCallsFunctionObjectType {
	#[serde(rename="function")]
	Function,
}

/** The definition of the function that was called. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFunctionObjectFunction {
	/** The name of the function. */
	pub name: String,
	/** The arguments passed to the function. */
	pub arguments: String,
	/** The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsFunctionObject {
	/** The ID of the tool call object. */
	pub id: String,
	#[serde(rename="type")]
	/** The type of tool call. This is always going to be `function` for this type of tool call. */
	pub r#type: RunStepDetailsToolCallsFunctionObjectType,
	/** The definition of the function that was called. */
	pub function: RunStepDetailsToolCallsFunctionObjectFunction,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepDetailsToolCallsObjectType {
	#[serde(rename="tool_calls")]
	ToolCalls,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStepDetailsToolCallsObjectToolCalls {
	RunStepDetailsToolCallsCodeObject(RunStepDetailsToolCallsCodeObject),
	RunStepDetailsToolCallsFileSearchObject(RunStepDetailsToolCallsFileSearchObject),
	RunStepDetailsToolCallsFunctionObject(RunStepDetailsToolCallsFunctionObject),
}

/** Details of the tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepDetailsToolCallsObject {
	#[serde(rename="type")]
	/** Always `tool_calls`. */
	pub r#type: RunStepDetailsToolCallsObjectType,
	/** An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`. */
	pub tool_calls: Vec<RunStepDetailsToolCallsObjectToolCalls>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepObjectObject {
	#[serde(rename="thread.run.step")]
	ThreadRunStep,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepObjectType {
	#[serde(rename="message_creation")]
	MessageCreation,
	#[serde(rename="tool_calls")]
	ToolCalls,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepObjectStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="cancelled")]
	Cancelled,
	#[serde(rename="failed")]
	Failed,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="expired")]
	Expired,
}

	/** The details of the run step. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepObjectStepDetails(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunStepObjectLastErrorCode {
	#[serde(rename="server_error")]
	ServerError,
	#[serde(rename="rate_limit_exceeded")]
	RateLimitExceeded,
}

/** The last error associated with this run step. Will be `null` if there are no errors. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepObjectLastError {
	/** One of `server_error` or `rate_limit_exceeded`. */
	pub code: RunStepObjectLastErrorCode,
	/** A human-readable description of the error. */
	pub message: String,
}

/** Represents a step in execution of a run. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunStepObject {
	/** The identifier of the run step, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `thread.run.step`. */
	pub object: RunStepObjectObject,
	/** The Unix timestamp (in seconds) for when the run step was created. */
	pub created_at: u64,
	/** The ID of the [assistant](/docs/api-reference/assistants) associated with the run step. */
	pub assistant_id: String,
	/** The ID of the [thread](/docs/api-reference/threads) that was run. */
	pub thread_id: String,
	/** The ID of the [run](/docs/api-reference/runs) that this run step is a part of. */
	pub run_id: String,
	#[serde(rename="type")]
	/** The type of run step, which can be either `message_creation` or `tool_calls`. */
	pub r#type: RunStepObjectType,
	/** The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`. */
	pub status: RunStepObjectStatus,
	/** The details of the run step. */
	pub step_details: RunStepObjectStepDetails,
	/** The last error associated with this run step. Will be `null` if there are no errors. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_error: Option<RunStepObjectLastError>,
	/** The Unix timestamp (in seconds) for when the run step expired. A step is considered expired if the parent run is expired. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expired_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the run step was cancelled. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cancelled_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the run step failed. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub failed_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the run step completed. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completed_at: Option<u64>,
	pub metadata: Metadata,
	pub usage: RunStepCompletionUsage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStepStreamEvent {
	/** Occurs when a [run step](/docs/api-reference/run-steps/step-object) is created. */
	A {
		event: String,
		data: RunStepObject,
	},
	/** Occurs when a [run step](/docs/api-reference/run-steps/step-object) moves to an `in_progress` state. */
	B {
		event: String,
		data: RunStepObject,
	},
	/** Occurs when parts of a [run step](/docs/api-reference/run-steps/step-object) are being streamed. */
	C {
		event: String,
		data: RunStepDeltaObject,
	},
	/** Occurs when a [run step](/docs/api-reference/run-steps/step-object) is completed. */
	D {
		event: String,
		data: RunStepObject,
	},
	/** Occurs when a [run step](/docs/api-reference/run-steps/step-object) fails. */
	E {
		event: String,
		data: RunStepObject,
	},
	/** Occurs when a [run step](/docs/api-reference/run-steps/step-object) is cancelled. */
	F {
		event: String,
		data: RunStepObject,
	},
	/** Occurs when a [run step](/docs/api-reference/run-steps/step-object) expires. */
	G {
		event: String,
		data: RunStepObject,
	},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunStreamEvent {
	/** Occurs when a new [run](/docs/api-reference/runs/object) is created. */
	A {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) moves to a `queued` status. */
	B {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) moves to an `in_progress` status. */
	C {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) moves to a `requires_action` status. */
	D {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) is completed. */
	E {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) ends with status `incomplete`. */
	F {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) fails. */
	G {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) moves to a `cancelling` status. */
	H {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) is cancelled. */
	I {
		event: String,
		data: RunObject,
	},
	/** Occurs when a [run](/docs/api-reference/runs/object) expires. */
	J {
		event: String,
		data: RunObject,
	},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RunToolCallObjectType {
	#[serde(rename="function")]
	Function,
}

/** The function definition. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunToolCallObjectFunction {
	/** The name of the function. */
	pub name: String,
	/** The arguments that the model expects you to pass to the function. */
	pub arguments: String,
}

/** Tool call objects */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RunToolCallObject {
	/** The ID of the tool call. This ID must be referenced when you submit the tool outputs in using the [Submit tool outputs to run](/docs/api-reference/runs/submitToolOutputs) endpoint. */
	pub id: String,
	#[serde(rename="type")]
	/** The type of tool call the output is required for. For now, this is always `function`. */
	pub r#type: RunToolCallObjectType,
	/** The function definition. */
	pub function: RunToolCallObjectFunction,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ScreenshotType {
	#[serde(rename="screenshot")]
	Screenshot,
}

/** A screenshot action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Screenshot {
	#[serde(rename="type")]
	/** Specifies the event type. For a screenshot action, this property is 
always set to `screenshot`. */
	pub r#type: ScreenshotType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ScrollType {
	#[serde(rename="scroll")]
	Scroll,
}

/** A scroll action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Scroll {
	#[serde(rename="type")]
	/** Specifies the event type. For a scroll action, this property is 
always set to `scroll`. */
	pub r#type: ScrollType,
	/** The x-coordinate where the scroll occurred. */
	pub x: u64,
	/** The y-coordinate where the scroll occurred. */
	pub y: u64,
	/** The horizontal scroll distance. */
	pub scroll_x: u64,
	/** The vertical scroll distance. */
	pub scroll_y: u64,
}

/** Specifies the processing type used for serving the request.
  - If set to 'auto', then the request will be processed with the service tier configured in the Project settings. Unless otherwise configured, the Project will use 'default'.
  - If set to 'default', then the requset will be processed with the standard pricing and performance for the selected model.
  - If set to '[flex](/docs/guides/flex-processing)' or 'priority', then the request will be processed with the corresponding service tier. [Contact sales](https://openai.com/contact-sales) to learn more about Priority processing.
  - When not set, the default behavior is 'auto'.

  When the `service_tier` parameter is set, the response body will include the `service_tier` value based on the processing mode actually used to serve the request. This response value may be different from the value set in the parameter. */
pub type ServiceTier = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SpeechAudioDeltaEventType {
	#[serde(rename="speech.audio.delta")]
	SpeechAudioDelta,
}

/** Emitted for each chunk of audio data generated during speech synthesis. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpeechAudioDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `speech.audio.delta`. */
	pub r#type: SpeechAudioDeltaEventType,
	/** A chunk of Base64-encoded audio data. */
	pub audio: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SpeechAudioDoneEventType {
	#[serde(rename="speech.audio.done")]
	SpeechAudioDone,
}

/** Token usage statistics for the request. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpeechAudioDoneEventUsage {
	/** Number of input tokens in the prompt. */
	pub input_tokens: u64,
	/** Number of output tokens generated. */
	pub output_tokens: u64,
	/** Total number of tokens used (input + output). */
	pub total_tokens: u64,
}

/** Emitted when the speech synthesis is complete and all audio has been streamed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SpeechAudioDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `speech.audio.done`. */
	pub r#type: SpeechAudioDoneEventType,
	/** Token usage statistics for the request. */
	pub usage: SpeechAudioDoneEventUsage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticChunkingStrategy {
	/** The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`. */
	pub max_chunk_size_tokens: u64,
	/** The number of tokens that overlap between chunks. The default value is `400`.

Note that the overlap must not exceed half of `max_chunk_size_tokens`. */
	pub chunk_overlap_tokens: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StaticChunkingStrategyRequestParamType {
	#[serde(rename="static")]
	Static,
}

/** Customize your own chunking strategy by setting chunk size and chunk overlap. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticChunkingStrategyRequestParam {
	#[serde(rename="type")]
	/** Always `static`. */
	pub r#type: StaticChunkingStrategyRequestParamType,
	#[serde(rename="static")]
	pub r#static: StaticChunkingStrategy,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StaticChunkingStrategyResponseParamType {
	#[serde(rename="static")]
	Static,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct StaticChunkingStrategyResponseParam {
	#[serde(rename="type")]
	/** Always `static`. */
	pub r#type: StaticChunkingStrategyResponseParamType,
	#[serde(rename="static")]
	pub r#static: StaticChunkingStrategy,
}

/** Not supported with latest reasoning models `o3` and `o4-mini`.

Up to 4 sequences where the API will stop generating further tokens. The
returned text will not contain the stop sequence. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopConfiguration {
	String(String),
	ArrayString(Vec<String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitToolOutputsRunRequestToolOutputs {
	/** The ID of the tool call in the `required_action` object within the run object the output is being submitted for. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_call_id: Option<String>,
	/** The output of the tool call to be submitted to continue the run. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitToolOutputsRunRequest {
	/** A list of tools for which the outputs are being submitted. */
	pub tool_outputs: Vec<SubmitToolOutputsRunRequestToolOutputs>,
	/** If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub stream: Option<bool>,
}

/** An object specifying the format that the model must output.

Configuring `{ "type": "json_schema" }` enables Structured Outputs, 
which ensures the model will match your supplied JSON schema. Learn more in the 
[Structured Outputs guide](/docs/guides/structured-outputs).

The default format is `{ "type": "text" }` with no additional options.

**Not recommended for gpt-4o and newer models:**

Setting to `{ "type": "json_object" }` enables the older JSON mode, which
ensures the message the model generates is valid JSON. Using `json_schema`
is preferred for models that support it. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextResponseFormatConfiguration {
	ResponseFormatText(ResponseFormatText),
	TextResponseFormatJsonSchema(TextResponseFormatJsonSchema),
	ResponseFormatJsonObject(ResponseFormatJsonObject),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TextResponseFormatJsonSchemaType {
	#[serde(rename="json_schema")]
	JsonSchema,
}

/** JSON Schema response format. Used to generate structured JSON responses.
Learn more about [Structured Outputs](/docs/guides/structured-outputs). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TextResponseFormatJsonSchema {
	#[serde(rename="type")]
	/** The type of response format being defined. Always `json_schema`. */
	pub r#type: TextResponseFormatJsonSchemaType,
	/** A description of what the response format is for, used by the model to
determine how to respond in the format. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	/** The name of the response format. Must be a-z, A-Z, 0-9, or contain
underscores and dashes, with a maximum length of 64. */
	pub name: String,
	pub schema: ResponseFormatJsonSchemaSchema,
	/** Whether to enable strict schema adherence when generating the output.
If set to true, the model will always follow the exact schema defined
in the `schema` field. Only a subset of JSON Schema is supported when
`strict` is `true`. To learn more, read the [Structured Outputs
guide](/docs/guides/structured-outputs). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub strict: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ThreadObjectObject {
	#[serde(rename="thread")]
	Thread,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadObjectToolResourcesCodeInterpreter {
	/** A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_ids: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadObjectToolResourcesFileSearch {
	/** The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vector_store_ids: Option<Vec<String>>,
}

/** A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadObjectToolResources {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code_interpreter: Option<ThreadObjectToolResourcesCodeInterpreter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_search: Option<ThreadObjectToolResourcesFileSearch>,
}

/** Represents a thread that contains [messages](/docs/api-reference/messages). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ThreadObject {
	/** The identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `thread`. */
	pub object: ThreadObjectObject,
	/** The Unix timestamp (in seconds) for when the thread was created. */
	pub created_at: u64,
	/** A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tool_resources: Option<ThreadObjectToolResources>,
	pub metadata: Metadata,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThreadStreamEvent {
	/** Occurs when a new [thread](/docs/api-reference/threads/object) is created. */
	A {
		enabled: bool,
		event: String,
		data: ThreadObject,
	},
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ToggleCertificatesRequest {
	pub certificate_ids: Vec<String>,
}

/** A tool that can be used to generate a response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tool {
	FunctionTool(FunctionTool),
	FileSearchTool(FileSearchTool),
	WebSearchPreviewTool(WebSearchPreviewTool),
	ComputerUsePreviewTool(ComputerUsePreviewTool),
	MCPTool(MCPTool),
	CodeInterpreterTool(CodeInterpreterTool),
	ImageGenTool(ImageGenTool),
	LocalShellTool(LocalShellTool),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ToolChoiceFunctionType {
	#[serde(rename="function")]
	Function,
}

/** Use this option to force the model to call a specific function. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolChoiceFunction {
	#[serde(rename="type")]
	/** For function calling, the type is always `function`. */
	pub r#type: ToolChoiceFunctionType,
	/** The name of the function to call. */
	pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ToolChoiceMCPType {
	#[serde(rename="mcp")]
	Mcp,
}

/** Use this option to force the model to call a specific tool on a remote MCP server. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolChoiceMCP {
	#[serde(rename="type")]
	/** For MCP tools, the type is always `mcp`. */
	pub r#type: ToolChoiceMCPType,
	/** The label of the MCP server to use. */
	pub server_label: String,
	/** The name of the tool to call on the server. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
}

/** Controls which (if any) tool is called by the model.

`none` means the model will not call any tool and instead generates a message.

`auto` means the model can pick between generating a message or calling one or
more tools.

`required` means the model must call one or more tools. */
pub type ToolChoiceOptions = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ToolChoiceTypesType {
	#[serde(rename="file_search")]
	FileSearch,
	#[serde(rename="web_search_preview")]
	WebSearchPreview,
	#[serde(rename="computer_use_preview")]
	ComputerUsePreview,
	#[serde(rename="web_search_preview_2025_03_11")]
	WebSearchPreview20250311,
	#[serde(rename="image_generation")]
	ImageGeneration,
	#[serde(rename="code_interpreter")]
	CodeInterpreter,
}

/** Indicates that the model should use a built-in tool to generate a response.
[Learn more about built-in tools](/docs/guides/tools). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ToolChoiceTypes {
	#[serde(rename="type")]
	/** The type of hosted tool the model should to use. Learn more about
[built-in tools](/docs/guides/tools).

Allowed values are:
- `file_search`
- `web_search_preview`
- `computer_use_preview`
- `code_interpreter`
- `image_generation` */
	pub r#type: ToolChoiceTypesType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TranscriptTextDeltaEventType {
	#[serde(rename="transcript.text.delta")]
	TranscriptTextDelta,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptTextDeltaEventLogprobs {
	/** The token that was used to generate the log probability. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
	/** The log probability of the token. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprob: Option<f32>,
	/** The bytes that were used to generate the log probability. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bytes: Option<Vec<u64>>,
}

/** Emitted when there is an additional text delta. This is also the first event emitted when the transcription starts. Only emitted when you [create a transcription](/docs/api-reference/audio/create-transcription) with the `Stream` parameter set to `true`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptTextDeltaEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `transcript.text.delta`. */
	pub r#type: TranscriptTextDeltaEventType,
	/** The text delta that was additionally transcribed. */
	pub delta: String,
	/** The log probabilities of the delta. Only included if you [create a transcription](/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<Vec<TranscriptTextDeltaEventLogprobs>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TranscriptTextDoneEventType {
	#[serde(rename="transcript.text.done")]
	TranscriptTextDone,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptTextDoneEventLogprobs {
	/** The token that was used to generate the log probability. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
	/** The log probability of the token. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprob: Option<f32>,
	/** The bytes that were used to generate the log probability. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bytes: Option<Vec<u64>>,
}

/** Emitted when the transcription is complete. Contains the complete transcription text. Only emitted when you [create a transcription](/docs/api-reference/audio/create-transcription) with the `Stream` parameter set to `true`. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptTextDoneEvent {
	#[serde(rename="type")]
	/** The type of the event. Always `transcript.text.done`. */
	pub r#type: TranscriptTextDoneEventType,
	/** The text that was transcribed. */
	pub text: String,
	/** The log probabilities of the individual tokens in the transcription. Only included if you [create a transcription](/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<Vec<TranscriptTextDoneEventLogprobs>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub usage: Option<TranscriptTextUsageTokens>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TranscriptTextUsageDurationType {
	#[serde(rename="duration")]
	Duration,
}

/** Usage statistics for models billed by audio input duration. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptTextUsageDuration {
	#[serde(rename="type")]
	/** The type of the usage object. Always `duration` for this variant. */
	pub r#type: TranscriptTextUsageDurationType,
	/** Duration of the input audio in seconds. */
	pub duration: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TranscriptTextUsageTokensType {
	#[serde(rename="tokens")]
	Tokens,
}

/** Details about the input tokens billed for this request. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptTextUsageTokensInputTokenDetails {
	/** Number of text tokens billed for this request. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text_tokens: Option<u64>,
	/** Number of audio tokens billed for this request. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub audio_tokens: Option<u64>,
}

/** Usage statistics for models billed by token usage. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptTextUsageTokens {
	#[serde(rename="type")]
	/** The type of the usage object. Always `tokens` for this variant. */
	pub r#type: TranscriptTextUsageTokensType,
	/** Number of input tokens billed for this request. */
	pub input_tokens: u64,
	/** Details about the input tokens billed for this request. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_token_details: Option<TranscriptTextUsageTokensInputTokenDetails>,
	/** Number of output tokens generated. */
	pub output_tokens: u64,
	/** Total number of tokens used (input + output). */
	pub total_tokens: u64,
}

/** Controls how the audio is cut into chunks. When set to `"auto"`, the
server first normalizes loudness and then uses voice activity detection (VAD) to
choose boundaries. `server_vad` object can be provided to tweak VAD detection
parameters manually. If unset, the audio is transcribed as a single block. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranscriptionChunkingStrategy {
	/** Automatically set chunking parameters based on the audio. Must be set to `"auto"`. */
	#[serde(rename="auto")]
	Auto,
	VadConfig(VadConfig),
}

pub type TranscriptionInclude = String;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionSegment {
	/** Unique identifier of the segment. */
	pub id: u64,
	/** Seek offset of the segment. */
	pub seek: u64,
	/** Start time of the segment in seconds. */
	pub start: f32,
	/** End time of the segment in seconds. */
	pub end: f32,
	/** Text content of the segment. */
	pub text: String,
	/** Array of token IDs for the text content. */
	pub tokens: Vec<u64>,
	/** Temperature parameter used for generating the segment. */
	pub temperature: f32,
	/** Average logprob of the segment. If the value is lower than -1, consider the logprobs failed. */
	pub avg_logprob: f32,
	/** Compression ratio of the segment. If the value is greater than 2.4, consider the compression failed. */
	pub compression_ratio: f32,
	/** Probability of no speech in the segment. If the value is higher than 1.0 and the `avg_logprob` is below -1, consider this segment silent. */
	pub no_speech_prob: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranscriptionWord {
	/** The text content of the word. */
	pub word: String,
	/** Start time of the word in seconds. */
	pub start: f32,
	/** End time of the word in seconds. */
	pub end: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TruncationObjectType {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="last_messages")]
	LastMessages,
}

/** Controls for how a thread will be truncated prior to the run. Use this to control the intial context window of the run. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TruncationObject {
	#[serde(rename="type")]
	/** The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`. */
	pub r#type: TruncationObjectType,
	/** The number of most recent messages from the thread when constructing the context for the run. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_messages: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum TypeType {
	#[serde(rename="type")]
	Type,
}

/** An action to type in text. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Type {
	#[serde(rename="type")]
	/** Specifies the event type. For a type action, this property is 
always set to `type`. */
	pub r#type: TypeType,
	/** The text to type. */
	pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVectorStoreFileAttributesRequest {
	pub attributes: VectorStoreFileAttributes,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVectorStoreRequestExpiresAfter {
	#[serde(flatten)]
	pub vector_store_expiration_after: VectorStoreExpirationAfter,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateVectorStoreRequest {
	/** The name of the vector store. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_after: Option<UpdateVectorStoreRequestExpiresAfter>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub metadata: Option<Metadata>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UploadStatus {
	#[serde(rename="pending")]
	Pending,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="cancelled")]
	Cancelled,
	#[serde(rename="expired")]
	Expired,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UploadObject {
	#[serde(rename="upload")]
	Upload,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadFile {
	#[serde(flatten)]
	pub open_a_i_file: OpenAIFile,
}

/** The Upload object can accept byte chunks in the form of Parts. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Upload {
	/** The Upload unique identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The Unix timestamp (in seconds) for when the Upload was created. */
	pub created_at: u64,
	/** The name of the file to be uploaded. */
	pub filename: String,
	/** The intended number of bytes to be uploaded. */
	pub bytes: u64,
	/** The intended purpose of the file. [Please refer here](/docs/api-reference/files/object#files/object-purpose) for acceptable values. */
	pub purpose: String,
	/** The status of the Upload. */
	pub status: UploadStatus,
	/** The Unix timestamp (in seconds) for when the Upload will expire. */
	pub expires_at: u64,
	/** The object type, which is always "upload". */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<UploadObject>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file: Option<UploadFile>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadCertificateRequest {
	/** An optional name for the certificate */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	/** The certificate content in PEM format */
	pub content: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UploadPartObject {
	#[serde(rename="upload.part")]
	UploadPart,
}

/** The upload Part represents a chunk of bytes we can add to an Upload object. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UploadPart {
	/** The upload Part unique identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The Unix timestamp (in seconds) for when the Part was created. */
	pub created_at: u64,
	/** The ID of the Upload object that this Part was added to. */
	pub upload_id: String,
	/** The object type, which is always `upload.part`. */
	pub object: UploadPartObject,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageAudioSpeechesResultObject {
	#[serde(rename="organization.usage.audio_speeches.result")]
	OrganizationUsageAudioSpeechesResult,
}

/** The aggregated audio speeches usage details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAudioSpeechesResult {
	pub object: UsageAudioSpeechesResultObject,
	/** The number of characters processed. */
	pub characters: u64,
	/** The count of requests made to the model. */
	pub num_model_requests: u64,
	/** When `group_by=project_id`, this field provides the project ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
	/** When `group_by=user_id`, this field provides the user ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<String>,
	/** When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_id: Option<String>,
	/** When `group_by=model`, this field provides the model name of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageAudioTranscriptionsResultObject {
	#[serde(rename="organization.usage.audio_transcriptions.result")]
	OrganizationUsageAudioTranscriptionsResult,
}

/** The aggregated audio transcriptions usage details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageAudioTranscriptionsResult {
	pub object: UsageAudioTranscriptionsResultObject,
	/** The number of seconds processed. */
	pub seconds: u64,
	/** The count of requests made to the model. */
	pub num_model_requests: u64,
	/** When `group_by=project_id`, this field provides the project ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
	/** When `group_by=user_id`, this field provides the user ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<String>,
	/** When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_id: Option<String>,
	/** When `group_by=model`, this field provides the model name of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageCodeInterpreterSessionsResultObject {
	#[serde(rename="organization.usage.code_interpreter_sessions.result")]
	OrganizationUsageCodeInterpreterSessionsResult,
}

/** The aggregated code interpreter sessions usage details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCodeInterpreterSessionsResult {
	pub object: UsageCodeInterpreterSessionsResultObject,
	/** The number of code interpreter sessions. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub num_sessions: Option<u64>,
	/** When `group_by=project_id`, this field provides the project ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageCompletionsResultObject {
	#[serde(rename="organization.usage.completions.result")]
	OrganizationUsageCompletionsResult,
}

/** The aggregated completions usage details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageCompletionsResult {
	pub object: UsageCompletionsResultObject,
	/** The aggregated number of text input tokens used, including cached tokens. For customers subscribe to scale tier, this includes scale tier tokens. */
	pub input_tokens: u64,
	/** The aggregated number of text input tokens that has been cached from previous requests. For customers subscribe to scale tier, this includes scale tier tokens. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_cached_tokens: Option<u64>,
	/** The aggregated number of text output tokens used. For customers subscribe to scale tier, this includes scale tier tokens. */
	pub output_tokens: u64,
	/** The aggregated number of audio input tokens used, including cached tokens. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub input_audio_tokens: Option<u64>,
	/** The aggregated number of audio output tokens used. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub output_audio_tokens: Option<u64>,
	/** The count of requests made to the model. */
	pub num_model_requests: u64,
	/** When `group_by=project_id`, this field provides the project ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
	/** When `group_by=user_id`, this field provides the user ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<String>,
	/** When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_id: Option<String>,
	/** When `group_by=model`, this field provides the model name of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
	/** When `group_by=batch`, this field tells whether the grouped usage result is batch or not. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub batch: Option<bool>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageEmbeddingsResultObject {
	#[serde(rename="organization.usage.embeddings.result")]
	OrganizationUsageEmbeddingsResult,
}

/** The aggregated embeddings usage details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageEmbeddingsResult {
	pub object: UsageEmbeddingsResultObject,
	/** The aggregated number of input tokens used. */
	pub input_tokens: u64,
	/** The count of requests made to the model. */
	pub num_model_requests: u64,
	/** When `group_by=project_id`, this field provides the project ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
	/** When `group_by=user_id`, this field provides the user ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<String>,
	/** When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_id: Option<String>,
	/** When `group_by=model`, this field provides the model name of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageImagesResultObject {
	#[serde(rename="organization.usage.images.result")]
	OrganizationUsageImagesResult,
}

/** The aggregated images usage details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageImagesResult {
	pub object: UsageImagesResultObject,
	/** The number of images processed. */
	pub images: u64,
	/** The count of requests made to the model. */
	pub num_model_requests: u64,
	/** When `group_by=source`, this field provides the source of the grouped usage result, possible values are `image.generation`, `image.edit`, `image.variation`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub source: Option<String>,
	/** When `group_by=size`, this field provides the image size of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub size: Option<String>,
	/** When `group_by=project_id`, this field provides the project ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
	/** When `group_by=user_id`, this field provides the user ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<String>,
	/** When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_id: Option<String>,
	/** When `group_by=model`, this field provides the model name of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageModerationsResultObject {
	#[serde(rename="organization.usage.moderations.result")]
	OrganizationUsageModerationsResult,
}

/** The aggregated moderations usage details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageModerationsResult {
	pub object: UsageModerationsResultObject,
	/** The aggregated number of input tokens used. */
	pub input_tokens: u64,
	/** The count of requests made to the model. */
	pub num_model_requests: u64,
	/** When `group_by=project_id`, this field provides the project ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
	/** When `group_by=user_id`, this field provides the user ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_id: Option<String>,
	/** When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_key_id: Option<String>,
	/** When `group_by=model`, this field provides the model name of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub model: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageResponseObject {
	#[serde(rename="page")]
	Page,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageResponse {
	pub object: UsageResponseObject,
	pub data: Vec<UsageTimeBucket>,
	pub has_more: bool,
	pub next_page: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageTimeBucketObject {
	#[serde(rename="bucket")]
	Bucket,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsageTimeBucketResult {
	UsageCompletionsResult(UsageCompletionsResult),
	UsageEmbeddingsResult(UsageEmbeddingsResult),
	UsageModerationsResult(UsageModerationsResult),
	UsageImagesResult(UsageImagesResult),
	UsageAudioSpeechesResult(UsageAudioSpeechesResult),
	UsageAudioTranscriptionsResult(UsageAudioTranscriptionsResult),
	UsageVectorStoresResult(UsageVectorStoresResult),
	UsageCodeInterpreterSessionsResult(UsageCodeInterpreterSessionsResult),
	CostsResult(CostsResult),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageTimeBucket {
	pub object: UsageTimeBucketObject,
	pub start_time: u64,
	pub end_time: u64,
	pub result: Vec<UsageTimeBucketResult>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UsageVectorStoresResultObject {
	#[serde(rename="organization.usage.vector_stores.result")]
	OrganizationUsageVectorStoresResult,
}

/** The aggregated vector stores usage details of the specific time bucket. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UsageVectorStoresResult {
	pub object: UsageVectorStoresResultObject,
	/** The vector stores usage in bytes. */
	pub usage_bytes: u64,
	/** When `group_by=project_id`, this field provides the project ID of the grouped usage result. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UserObject {
	#[serde(rename="organization.user")]
	OrganizationUser,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UserRole {
	#[serde(rename="owner")]
	Owner,
	#[serde(rename="reader")]
	Reader,
}

/** Represents an individual `user` within an organization. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
	/** The object type, which is always `organization.user` */
	pub object: UserObject,
	/** The identifier, which can be referenced in API endpoints */
	pub id: String,
	/** The name of the user */
	pub name: String,
	/** The email address of the user */
	pub email: String,
	/** `owner` or `reader` */
	pub role: UserRole,
	/** The Unix timestamp (in seconds) of when the user was added. */
	pub added_at: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UserDeleteResponseObject {
	#[serde(rename="organization.user.deleted")]
	OrganizationUserDeleted,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserDeleteResponse {
	pub object: UserDeleteResponseObject,
	pub id: String,
	pub deleted: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UserListResponseObject {
	#[serde(rename="list")]
	List,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserListResponse {
	pub object: UserListResponseObject,
	pub data: Vec<User>,
	pub first_id: String,
	pub last_id: String,
	pub has_more: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UserRoleUpdateRequestRole {
	#[serde(rename="owner")]
	Owner,
	#[serde(rename="reader")]
	Reader,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserRoleUpdateRequest {
	/** `owner` or `reader` */
	pub role: UserRoleUpdateRequestRole,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VadConfigType {
	#[serde(rename="server_vad")]
	ServerVad,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VadConfig {
	#[serde(rename="type")]
	/** Must be set to `server_vad` to enable manual chunking using server side VAD. */
	pub r#type: VadConfigType,
	/** Amount of audio to include before the VAD detected speech (in 
milliseconds). */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub prefix_padding_ms: Option<u64>,
	/** Duration of silence to detect speech stop (in milliseconds).
With shorter values the model will respond more quickly, 
but may jump in on short pauses from the user. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub silence_duration_ms: Option<u64>,
	/** Sensitivity threshold (0.0 to 1.0) for voice activity detection. A 
higher threshold will require louder audio to activate the model, and 
thus might perform better in noisy environments. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold: Option<f32>,
}

	/** The grader used for the fine-tuning job. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateGraderRequestGrader(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateGraderRequest {
	/** The grader used for the fine-tuning job. */
	pub grader: ValidateGraderRequestGrader,
}

	/** The grader used for the fine-tuning job. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateGraderResponseGrader(pub String);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidateGraderResponse {
	/** The grader used for the fine-tuning job. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub grader: Option<ValidateGraderResponseGrader>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreExpirationAfterAnchor {
	#[serde(rename="last_active_at")]
	LastActiveAt,
}

/** The expiration policy for a vector store. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreExpirationAfter {
	/** Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`. */
	pub anchor: VectorStoreExpirationAfterAnchor,
	/** The number of days after the anchor time that the vector store will expire. */
	pub days: u64,
}

/** Set of 16 key-value pairs that can be attached to an object. This can be 
useful for storing additional information about the object in a structured 
format, and querying for objects via API or the dashboard. Keys are strings 
with a maximum length of 64 characters. Values are strings with a maximum 
length of 512 characters, booleans, or numbers. */
pub type VectorStoreFileAttributes = HashMap<String, String>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreFileBatchObjectObject {
	#[serde(rename="vector_store.files_batch")]
	VectorStoreFilesBatch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreFileBatchObjectStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="cancelled")]
	Cancelled,
	#[serde(rename="failed")]
	Failed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreFileBatchObjectFileCounts {
	/** The number of files that are currently being processed. */
	pub in_progress: u64,
	/** The number of files that have been processed. */
	pub completed: u64,
	/** The number of files that have failed to process. */
	pub failed: u64,
	/** The number of files that where cancelled. */
	pub cancelled: u64,
	/** The total number of files. */
	pub total: u64,
}

/** A batch of files attached to a vector store. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreFileBatchObject {
	/** The identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `vector_store.file_batch`. */
	pub object: VectorStoreFileBatchObjectObject,
	/** The Unix timestamp (in seconds) for when the vector store files batch was created. */
	pub created_at: u64,
	/** The ID of the [vector store](/docs/api-reference/vector-stores/object) that the [File](/docs/api-reference/files) is attached to. */
	pub vector_store_id: String,
	/** The status of the vector store files batch, which can be either `in_progress`, `completed`, `cancelled` or `failed`. */
	pub status: VectorStoreFileBatchObjectStatus,
	pub file_counts: VectorStoreFileBatchObjectFileCounts,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreFileContentResponseObject {
	#[serde(rename="vector_store.file_content.page")]
	VectorStoreFileContentPage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreFileContentResponseData {
	#[serde(rename="type")]
	/** The content type (currently only `"text"`) */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<String>,
	/** The text content */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub text: Option<String>,
}

/** Represents the parsed content of a vector store file. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreFileContentResponse {
	/** The object type, which is always `vector_store.file_content.page` */
	pub object: VectorStoreFileContentResponseObject,
	/** Parsed content of the file. */
	pub data: Vec<VectorStoreFileContentResponseData>,
	/** Indicates if there are more content pages to fetch. */
	pub has_more: bool,
	/** The token for the next page, if any. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub next_page: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreFileObjectObject {
	#[serde(rename="vector_store.file")]
	VectorStoreFile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreFileObjectStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="cancelled")]
	Cancelled,
	#[serde(rename="failed")]
	Failed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreFileObjectLastErrorCode {
	#[serde(rename="server_error")]
	ServerError,
	#[serde(rename="unsupported_file")]
	UnsupportedFile,
	#[serde(rename="invalid_file")]
	InvalidFile,
}

/** The last error associated with this vector store file. Will be `null` if there are no errors. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreFileObjectLastError {
	/** One of `server_error` or `rate_limit_exceeded`. */
	pub code: VectorStoreFileObjectLastErrorCode,
	/** A human-readable description of the error. */
	pub message: String,
}

	/** The strategy used to chunk the file. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreFileObjectChunkingStrategy(pub String);

/** A list of files attached to a vector store. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreFileObject {
	/** The identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `vector_store.file`. */
	pub object: VectorStoreFileObjectObject,
	/** The total vector store usage in bytes. Note that this may be different from the original file size. */
	pub usage_bytes: u64,
	/** The Unix timestamp (in seconds) for when the vector store file was created. */
	pub created_at: u64,
	/** The ID of the [vector store](/docs/api-reference/vector-stores/object) that the [File](/docs/api-reference/files) is attached to. */
	pub vector_store_id: String,
	/** The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use. */
	pub status: VectorStoreFileObjectStatus,
	/** The last error associated with this vector store file. Will be `null` if there are no errors. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_error: Option<VectorStoreFileObjectLastError>,
	/** The strategy used to chunk the file. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub chunking_strategy: Option<VectorStoreFileObjectChunkingStrategy>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub attributes: Option<VectorStoreFileAttributes>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreObjectObject {
	#[serde(rename="vector_store")]
	VectorStore,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreObjectFileCounts {
	/** The number of files that are currently being processed. */
	pub in_progress: u64,
	/** The number of files that have been successfully processed. */
	pub completed: u64,
	/** The number of files that have failed to process. */
	pub failed: u64,
	/** The number of files that were cancelled. */
	pub cancelled: u64,
	/** The total number of files. */
	pub total: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreObjectStatus {
	#[serde(rename="expired")]
	Expired,
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
}

/** A vector store is a collection of processed files can be used by the `file_search` tool. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreObject {
	/** The identifier, which can be referenced in API endpoints. */
	pub id: String,
	/** The object type, which is always `vector_store`. */
	pub object: VectorStoreObjectObject,
	/** The Unix timestamp (in seconds) for when the vector store was created. */
	pub created_at: u64,
	/** The name of the vector store. */
	pub name: String,
	/** The total number of bytes used by the files in the vector store. */
	pub usage_bytes: u64,
	pub file_counts: VectorStoreObjectFileCounts,
	/** The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use. */
	pub status: VectorStoreObjectStatus,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_after: Option<VectorStoreExpirationAfter>,
	/** The Unix timestamp (in seconds) for when the vector store will expire. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expires_at: Option<u64>,
	/** The Unix timestamp (in seconds) for when the vector store was last active. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_active_at: Option<u64>,
	pub metadata: Metadata,
}

/** A query string for a search */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorStoreSearchRequestQuery {
	String(String),
	ArrayString(Vec<String>),
}

/** A filter to apply based on file attributes. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VectorStoreSearchRequestFilters {
	ComparisonFilter(ComparisonFilter),
	CompoundFilter(CompoundFilter),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreSearchRequestRankingOptionsRanker {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="default-2024-11-15")]
	Default20241115,
}

/** Ranking options for search. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreSearchRequestRankingOptions {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranker: Option<VectorStoreSearchRequestRankingOptionsRanker>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub score_threshold: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreSearchRequest {
	/** A query string for a search */
	pub query: VectorStoreSearchRequestQuery,
	/** Whether to rewrite the natural language query for vector search. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub rewrite_query: Option<bool>,
	/** The maximum number of results to return. This number should be between 1 and 50 inclusive. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_num_results: Option<u64>,
	/** A filter to apply based on file attributes. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filters: Option<VectorStoreSearchRequestFilters>,
	/** Ranking options for search. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranking_options: Option<VectorStoreSearchRequestRankingOptions>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreSearchResultContentObjectType {
	#[serde(rename="text")]
	Text,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreSearchResultContentObject {
	#[serde(rename="type")]
	/** The type of content. */
	pub r#type: VectorStoreSearchResultContentObjectType,
	/** The text content returned from search. */
	pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreSearchResultItem {
	/** The ID of the vector store file. */
	pub file_id: String,
	/** The name of the vector store file. */
	pub filename: String,
	/** The similarity score for the result. */
	pub score: f32,
	pub attributes: VectorStoreFileAttributes,
	/** Content chunks from the file. */
	pub content: Vec<VectorStoreSearchResultContentObject>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum VectorStoreSearchResultsPageObject {
	#[serde(rename="vector_store.search_results.page")]
	VectorStoreSearchResultsPage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct VectorStoreSearchResultsPage {
	/** The object type, which is always `vector_store.search_results.page` */
	pub object: VectorStoreSearchResultsPageObject,
	pub search_query: Vec<String>,
	/** The list of search result items. */
	pub data: Vec<VectorStoreSearchResultItem>,
	/** Indicates if there are more results to fetch. */
	pub has_more: bool,
	/** The token for the next page, if any. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub next_page: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VoiceIdsShared {
	String(String),
	#[serde(rename="alloy")]
	Alloy,
	#[serde(rename="ash")]
	Ash,
	#[serde(rename="ballad")]
	Ballad,
	#[serde(rename="coral")]
	Coral,
	#[serde(rename="echo")]
	Echo,
	#[serde(rename="fable")]
	Fable,
	#[serde(rename="onyx")]
	Onyx,
	#[serde(rename="nova")]
	Nova,
	#[serde(rename="sage")]
	Sage,
	#[serde(rename="shimmer")]
	Shimmer,
	#[serde(rename="verse")]
	Verse,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WaitType {
	#[serde(rename="wait")]
	Wait,
}

/** A wait action. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Wait {
	#[serde(rename="type")]
	/** Specifies the event type. For a wait action, this property is 
always set to `wait`. */
	pub r#type: WaitType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebSearchActionFindType {
	#[serde(rename="find")]
	Find,
}

/** Action type "find": Searches for a pattern within a loaded page. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchActionFind {
	#[serde(rename="type")]
	/** The action type. */
	pub r#type: WebSearchActionFindType,
	/** The URL of the page searched for the pattern. */
	pub url: String,
	/** The pattern or text to search for within the page. */
	pub pattern: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebSearchActionOpenPageType {
	#[serde(rename="open_page")]
	OpenPage,
}

/** Action type "open_page" - Opens a specific URL from search results. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchActionOpenPage {
	#[serde(rename="type")]
	/** The action type. */
	pub r#type: WebSearchActionOpenPageType,
	/** The URL opened by the model. */
	pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebSearchActionSearchType {
	#[serde(rename="search")]
	Search,
}

/** Action type "search" - Performs a web search query. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchActionSearch {
	#[serde(rename="type")]
	/** The action type. */
	pub r#type: WebSearchActionSearchType,
	/** The search query. */
	pub query: String,
}

/** High level guidance for the amount of context window space to use for the 
search. One of `low`, `medium`, or `high`. `medium` is the default. */
pub type WebSearchContextSize = String;

/** Approximate location parameters for the search. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchLocation {
	/** The two-letter 
[ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user,
e.g. `US`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub country: Option<String>,
	/** Free text input for the region of the user, e.g. `California`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<String>,
	/** Free text input for the city of the user, e.g. `San Francisco`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub city: Option<String>,
	/** The [IANA timezone](https://timeapi.io/documentation/iana-timezones) 
of the user, e.g. `America/Los_Angeles`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timezone: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebSearchToolCallType {
	#[serde(rename="web_search_call")]
	WebSearchCall,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebSearchToolCallStatus {
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="searching")]
	Searching,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="failed")]
	Failed,
}

	/** An object describing the specific action taken in this web search call.
Includes details on how the model used the web (search, open_page, find). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchToolCallAction(pub String);

/** The results of a web search tool call. See the 
[web search guide](/docs/guides/tools-web-search) for more information. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchToolCall {
	/** The unique ID of the web search tool call. */
	pub id: String,
	#[serde(rename="type")]
	/** The type of the web search tool call. Always `web_search_call`. */
	pub r#type: WebSearchToolCallType,
	/** The status of the web search tool call. */
	pub status: WebSearchToolCallStatus,
	/** An object describing the specific action taken in this web search call.
Includes details on how the model used the web (search, open_page, find). */
	pub action: WebSearchToolCallAction,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchCancelledData {
	/** The unique ID of the batch API request. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookBatchCancelledObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookBatchCancelledType {
	#[serde(rename="batch.cancelled")]
	BatchCancelled,
}

/** Sent when a batch API request has been cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchCancelled {
	/** The Unix timestamp (in seconds) of when the batch API request was cancelled. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookBatchCancelledData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookBatchCancelledObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `batch.cancelled`. */
	pub r#type: WebhookBatchCancelledType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchCompletedData {
	/** The unique ID of the batch API request. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookBatchCompletedObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookBatchCompletedType {
	#[serde(rename="batch.completed")]
	BatchCompleted,
}

/** Sent when a batch API request has been completed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchCompleted {
	/** The Unix timestamp (in seconds) of when the batch API request was completed. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookBatchCompletedData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookBatchCompletedObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `batch.completed`. */
	pub r#type: WebhookBatchCompletedType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchExpiredData {
	/** The unique ID of the batch API request. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookBatchExpiredObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookBatchExpiredType {
	#[serde(rename="batch.expired")]
	BatchExpired,
}

/** Sent when a batch API request has expired. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchExpired {
	/** The Unix timestamp (in seconds) of when the batch API request expired. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookBatchExpiredData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookBatchExpiredObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `batch.expired`. */
	pub r#type: WebhookBatchExpiredType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchFailedData {
	/** The unique ID of the batch API request. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookBatchFailedObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookBatchFailedType {
	#[serde(rename="batch.failed")]
	BatchFailed,
}

/** Sent when a batch API request has failed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookBatchFailed {
	/** The Unix timestamp (in seconds) of when the batch API request failed. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookBatchFailedData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookBatchFailedObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `batch.failed`. */
	pub r#type: WebhookBatchFailedType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEvalRunCanceledData {
	/** The unique ID of the eval run. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookEvalRunCanceledObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookEvalRunCanceledType {
	#[serde(rename="eval.run.canceled")]
	EvalRunCanceled,
}

/** Sent when an eval run has been canceled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEvalRunCanceled {
	/** The Unix timestamp (in seconds) of when the eval run was canceled. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookEvalRunCanceledData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookEvalRunCanceledObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `eval.run.canceled`. */
	pub r#type: WebhookEvalRunCanceledType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEvalRunFailedData {
	/** The unique ID of the eval run. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookEvalRunFailedObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookEvalRunFailedType {
	#[serde(rename="eval.run.failed")]
	EvalRunFailed,
}

/** Sent when an eval run has failed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEvalRunFailed {
	/** The Unix timestamp (in seconds) of when the eval run failed. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookEvalRunFailedData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookEvalRunFailedObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `eval.run.failed`. */
	pub r#type: WebhookEvalRunFailedType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEvalRunSucceededData {
	/** The unique ID of the eval run. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookEvalRunSucceededObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookEvalRunSucceededType {
	#[serde(rename="eval.run.succeeded")]
	EvalRunSucceeded,
}

/** Sent when an eval run has succeeded. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookEvalRunSucceeded {
	/** The Unix timestamp (in seconds) of when the eval run succeeded. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookEvalRunSucceededData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookEvalRunSucceededObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `eval.run.succeeded`. */
	pub r#type: WebhookEvalRunSucceededType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookFineTuningJobCancelledData {
	/** The unique ID of the fine-tuning job. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookFineTuningJobCancelledObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookFineTuningJobCancelledType {
	#[serde(rename="fine_tuning.job.cancelled")]
	FineTuningJobCancelled,
}

/** Sent when a fine-tuning job has been cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookFineTuningJobCancelled {
	/** The Unix timestamp (in seconds) of when the fine-tuning job was cancelled. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookFineTuningJobCancelledData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookFineTuningJobCancelledObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `fine_tuning.job.cancelled`. */
	pub r#type: WebhookFineTuningJobCancelledType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookFineTuningJobFailedData {
	/** The unique ID of the fine-tuning job. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookFineTuningJobFailedObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookFineTuningJobFailedType {
	#[serde(rename="fine_tuning.job.failed")]
	FineTuningJobFailed,
}

/** Sent when a fine-tuning job has failed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookFineTuningJobFailed {
	/** The Unix timestamp (in seconds) of when the fine-tuning job failed. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookFineTuningJobFailedData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookFineTuningJobFailedObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `fine_tuning.job.failed`. */
	pub r#type: WebhookFineTuningJobFailedType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookFineTuningJobSucceededData {
	/** The unique ID of the fine-tuning job. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookFineTuningJobSucceededObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookFineTuningJobSucceededType {
	#[serde(rename="fine_tuning.job.succeeded")]
	FineTuningJobSucceeded,
}

/** Sent when a fine-tuning job has succeeded. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookFineTuningJobSucceeded {
	/** The Unix timestamp (in seconds) of when the fine-tuning job succeeded. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookFineTuningJobSucceededData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookFineTuningJobSucceededObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `fine_tuning.job.succeeded`. */
	pub r#type: WebhookFineTuningJobSucceededType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponseCancelledData {
	/** The unique ID of the model response. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookResponseCancelledObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookResponseCancelledType {
	#[serde(rename="response.cancelled")]
	ResponseCancelled,
}

/** Sent when a background response has been cancelled. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponseCancelled {
	/** The Unix timestamp (in seconds) of when the model response was cancelled. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookResponseCancelledData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookResponseCancelledObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `response.cancelled`. */
	pub r#type: WebhookResponseCancelledType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponseCompletedData {
	/** The unique ID of the model response. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookResponseCompletedObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookResponseCompletedType {
	#[serde(rename="response.completed")]
	ResponseCompleted,
}

/** Sent when a background response has been completed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponseCompleted {
	/** The Unix timestamp (in seconds) of when the model response was completed. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookResponseCompletedData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookResponseCompletedObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `response.completed`. */
	pub r#type: WebhookResponseCompletedType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponseFailedData {
	/** The unique ID of the model response. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookResponseFailedObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookResponseFailedType {
	#[serde(rename="response.failed")]
	ResponseFailed,
}

/** Sent when a background response has failed. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponseFailed {
	/** The Unix timestamp (in seconds) of when the model response failed. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookResponseFailedData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookResponseFailedObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `response.failed`. */
	pub r#type: WebhookResponseFailedType,
}

/** Event data payload. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponseIncompleteData {
	/** The unique ID of the model response. */
	pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookResponseIncompleteObject {
	#[serde(rename="event")]
	Event,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebhookResponseIncompleteType {
	#[serde(rename="response.incomplete")]
	ResponseIncomplete,
}

/** Sent when a background response has been interrupted. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebhookResponseIncomplete {
	/** The Unix timestamp (in seconds) of when the model response was interrupted. */
	pub created_at: u64,
	/** The unique ID of the event. */
	pub id: String,
	/** Event data payload. */
	pub data: WebhookResponseIncompleteData,
	/** The object of the event. Always `event`. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub object: Option<WebhookResponseIncompleteObject>,
	#[serde(rename="type")]
	/** The type of the event. Always `response.incomplete`. */
	pub r#type: WebhookResponseIncompleteType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputTextContentType {
	#[serde(rename="input_text")]
	InputText,
}

/** A text input to the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputTextContent {
	#[serde(rename="type")]
	/** The type of the input item. Always `input_text`. */
	pub r#type: InputTextContentType,
	/** The text input to the model. */
	pub text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputImageContentType {
	#[serde(rename="input_image")]
	InputImage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputImageContentImageUrl {
	/** The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputImageContentFileId {
	/** The ID of the file to be sent to the model. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputImageContentDetail {
	#[serde(rename="low")]
	Low,
	#[serde(rename="high")]
	High,
	#[serde(rename="auto")]
	Auto,
}

/** An image input to the model. Learn about [image inputs](/docs/guides/vision). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputImageContent {
	#[serde(rename="type")]
	/** The type of the input item. Always `input_image`. */
	pub r#type: InputImageContentType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<InputImageContentImageUrl>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<InputImageContentFileId>,
	/** The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`. */
	pub detail: InputImageContentDetail,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputFileContentType {
	#[serde(rename="input_file")]
	InputFile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputFileContentFileId {
	/** The ID of the file to be sent to the model. */
	String(String),
	None,
}

/** A file input to the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct InputFileContent {
	#[serde(rename="type")]
	/** The type of the input item. Always `input_file`. */
	pub r#type: InputFileContentType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_id: Option<InputFileContentFileId>,
	/** The name of the file to be sent to the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filename: Option<String>,
	/** The content of the file to be sent to the model. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub file_data: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FunctionToolType {
	#[serde(rename="function")]
	Function,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionToolDescription {
	/** A description of the function. Used by the model to determine whether or not to call the function. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionToolParameters {
	/** A JSON schema object describing the parameters of the function. */
	Map(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionToolStrict {
	/** Whether to enforce strict parameter validation. Default `true`. */
	Boolean(bool),
	None,
}

/** Defines a function in your own code the model can choose to call. Learn more about [function calling](https://platform.openai.com/docs/guides/function-calling). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionTool {
	#[serde(rename="type")]
	/** The type of the function tool. Always `function`. */
	pub r#type: FunctionToolType,
	/** The name of the function to call. */
	pub name: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<FunctionToolDescription>,
	pub parameters: FunctionToolParameters,
	pub strict: FunctionToolStrict,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RankingOptionsRanker {
	#[serde(rename="auto")]
	Auto,
	#[serde(rename="default-2024-11-15")]
	Default20241115,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RankingOptions {
	/** The ranker to use for the file search. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranker: Option<RankingOptionsRanker>,
	/** The score threshold for the file search, a number between 0 and 1. Numbers closer to 1 will attempt to return only the most relevant results, but may return fewer results. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub score_threshold: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Filters {
	ComparisonFilter(ComparisonFilter),
	CompoundFilter(CompoundFilter),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FileSearchToolType {
	#[serde(rename="file_search")]
	FileSearch,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileSearchToolFilters {
	/** A filter to apply. */
	Filters(Filters),
	None,
}

/** A tool that searches for relevant content from uploaded files. Learn more about the [file search tool](https://platform.openai.com/docs/guides/tools-file-search). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileSearchTool {
	#[serde(rename="type")]
	/** The type of the file search tool. Always `file_search`. */
	pub r#type: FileSearchToolType,
	/** The IDs of the vector stores to search. */
	pub vector_store_ids: Vec<String>,
	/** The maximum number of results to return. This number should be between 1 and 50 inclusive. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_num_results: Option<u64>,
	/** Ranking options for search. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ranking_options: Option<RankingOptions>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filters: Option<FileSearchToolFilters>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApproximateLocationType {
	#[serde(rename="approximate")]
	Approximate,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproximateLocationCountry {
	/** The two-letter [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user, e.g. `US`. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproximateLocationRegion {
	/** Free text input for the region of the user, e.g. `California`. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproximateLocationCity {
	/** Free text input for the city of the user, e.g. `San Francisco`. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproximateLocationTimezone {
	/** The [IANA timezone](https://timeapi.io/documentation/iana-timezones) of the user, e.g. `America/Los_Angeles`. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApproximateLocation {
	#[serde(rename="type")]
	/** The type of location approximation. Always `approximate`. */
	pub r#type: ApproximateLocationType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub country: Option<ApproximateLocationCountry>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub region: Option<ApproximateLocationRegion>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub city: Option<ApproximateLocationCity>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timezone: Option<ApproximateLocationTimezone>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebSearchPreviewToolType {
	#[serde(rename="web_search_preview")]
	WebSearchPreview,
	#[serde(rename="web_search_preview_2025_03_11")]
	WebSearchPreview20250311,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WebSearchPreviewToolUserLocation {
	/** The user's location. */
	ApproximateLocation(ApproximateLocation),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum WebSearchPreviewToolSearchContextSize {
	#[serde(rename="low")]
	Low,
	#[serde(rename="medium")]
	Medium,
	#[serde(rename="high")]
	High,
}

/** This tool searches the web for relevant results to use in a response. Learn more about the [web search tool](https://platform.openai.com/docs/guides/tools-web-search). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct WebSearchPreviewTool {
	#[serde(rename="type")]
	/** The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`. */
	pub r#type: WebSearchPreviewToolType,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_location: Option<WebSearchPreviewToolUserLocation>,
	/** High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default. */
	#[serde(skip_serializing_if = "Option::is_none")]
	pub search_context_size: Option<WebSearchPreviewToolSearchContextSize>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputerUsePreviewToolType {
	#[serde(rename="computer_use_preview")]
	ComputerUsePreview,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputerUsePreviewToolEnvironment {
	#[serde(rename="windows")]
	Windows,
	#[serde(rename="mac")]
	Mac,
	#[serde(rename="linux")]
	Linux,
	#[serde(rename="ubuntu")]
	Ubuntu,
	#[serde(rename="browser")]
	Browser,
}

/** A tool that controls a virtual computer. Learn more about the [computer tool](https://platform.openai.com/docs/guides/tools-computer-use). */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerUsePreviewTool {
	#[serde(rename="type")]
	/** The type of the computer use tool. Always `computer_use_preview`. */
	pub r#type: ComputerUsePreviewToolType,
	/** The type of computer environment to control. */
	pub environment: ComputerUsePreviewToolEnvironment,
	/** The width of the computer display. */
	pub display_width: u64,
	/** The height of the computer display. */
	pub display_height: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FileCitationBodyType {
	#[serde(rename="file_citation")]
	FileCitation,
}

/** A citation to a file. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FileCitationBody {
	#[serde(rename="type")]
	/** The type of the file citation. Always `file_citation`. */
	pub r#type: FileCitationBodyType,
	/** The ID of the file. */
	pub file_id: String,
	/** The index of the file in the list of files. */
	pub index: u64,
	/** The filename of the file cited. */
	pub filename: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UrlCitationBodyType {
	#[serde(rename="url_citation")]
	UrlCitation,
}

/** A citation for a web resource used to generate a model response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UrlCitationBody {
	#[serde(rename="type")]
	/** The type of the URL citation. Always `url_citation`. */
	pub r#type: UrlCitationBodyType,
	/** The URL of the web resource. */
	pub url: String,
	/** The index of the first character of the URL citation in the message. */
	pub start_index: u64,
	/** The index of the last character of the URL citation in the message. */
	pub end_index: u64,
	/** The title of the web resource. */
	pub title: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerFileCitationBodyType {
	#[serde(rename="container_file_citation")]
	ContainerFileCitation,
}

/** A citation for a container file used to generate a model response. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerFileCitationBody {
	#[serde(rename="type")]
	/** The type of the container file citation. Always `container_file_citation`. */
	pub r#type: ContainerFileCitationBodyType,
	/** The ID of the container file. */
	pub container_id: String,
	/** The ID of the file. */
	pub file_id: String,
	/** The index of the first character of the container file citation in the message. */
	pub start_index: u64,
	/** The index of the last character of the container file citation in the message. */
	pub end_index: u64,
	/** The filename of the container file cited. */
	pub filename: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Annotation {
	FileCitationBody(FileCitationBody),
	UrlCitationBody(UrlCitationBody),
	ContainerFileCitationBody(ContainerFileCitationBody),
	FilePath(FilePath),
}

/** The top log probability of a token. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TopLogProb {
	pub token: String,
	pub logprob: f32,
	pub bytes: Vec<u64>,
}

/** The log probability of a token. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LogProb {
	pub token: String,
	pub logprob: f32,
	pub bytes: Vec<u64>,
	pub top_logprobs: Vec<TopLogProb>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum OutputTextContentType {
	#[serde(rename="output_text")]
	OutputText,
}

/** A text output from the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OutputTextContent {
	#[serde(rename="type")]
	/** The type of the output text. Always `output_text`. */
	pub r#type: OutputTextContentType,
	/** The text output from the model. */
	pub text: String,
	/** The annotations of the text output. */
	pub annotations: Vec<Annotation>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub logprobs: Option<Vec<LogProb>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum RefusalContentType {
	#[serde(rename="refusal")]
	Refusal,
}

/** A refusal from the model. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RefusalContent {
	#[serde(rename="type")]
	/** The type of the refusal. Always `refusal`. */
	pub r#type: RefusalContentType,
	/** The refusal explanationfrom the model. */
	pub refusal: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputerCallSafetyCheckParamCode {
	/** The type of the pending safety check. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputerCallSafetyCheckParamMessage {
	/** Details about the pending safety check. */
	String(String),
	None,
}

/** A pending safety check for the computer call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerCallSafetyCheckParam {
	/** The ID of the pending safety check. */
	pub id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub code: Option<ComputerCallSafetyCheckParamCode>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub message: Option<ComputerCallSafetyCheckParamMessage>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputerCallOutputItemParamId {
	/** The ID of the computer tool call output. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ComputerCallOutputItemParamType {
	#[serde(rename="computer_call_output")]
	ComputerCallOutput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputerCallOutputItemParamAcknowledgedSafetyChecks {
	/** The safety checks reported by the API that have been acknowledged by the developer. */
	ArrayList(Vec<ComputerCallSafetyCheckParam>),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComputerCallOutputItemParamStatus {
	/** The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API. */
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
	None,
}

/** The output of a computer tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputerCallOutputItemParam {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<ComputerCallOutputItemParamId>,
	/** The ID of the computer tool call that produced the output. */
	pub call_id: String,
	#[serde(rename="type")]
	/** The type of the computer tool call output. Always `computer_call_output`. */
	pub r#type: ComputerCallOutputItemParamType,
	pub output: ComputerScreenshotImage,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub acknowledged_safety_checks: Option<ComputerCallOutputItemParamAcknowledgedSafetyChecks>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<ComputerCallOutputItemParamStatus>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCallOutputItemParamId {
	/** The unique ID of the function tool call output. Populated when this item is returned via API. */
	String(String),
	None,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum FunctionCallOutputItemParamType {
	#[serde(rename="function_call_output")]
	FunctionCallOutput,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionCallOutputItemParamStatus {
	/** The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API. */
	#[serde(rename="in_progress")]
	InProgress,
	#[serde(rename="completed")]
	Completed,
	#[serde(rename="incomplete")]
	Incomplete,
	None,
}

/** The output of a function tool call. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FunctionCallOutputItemParam {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub id: Option<FunctionCallOutputItemParamId>,
	/** The unique ID of the function tool call generated by the model. */
	pub call_id: String,
	#[serde(rename="type")]
	/** The type of the function tool call output. Always `function_call_output`. */
	pub r#type: FunctionCallOutputItemParamType,
	/** A JSON string of the output of the function tool call. */
	pub output: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub status: Option<FunctionCallOutputItemParamStatus>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemReferenceParamType {
	/** The type of item to reference. Always `item_reference`. */
	#[serde(rename="item_reference")]
	ItemReference,
	None,
}

/** An internal identifier for an item to reference. */
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemReferenceParam {
	#[serde(rename="type")]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub r#type: Option<ItemReferenceParamType>,
	/** The ID of the item to reference. */
	pub id: String,
}

