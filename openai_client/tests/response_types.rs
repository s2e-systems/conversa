use std::collections::HashMap;

use conversa_openai_client::types::{
    CreateModelResponseProperties, CreateModelResponsePropertiesObject, CreateResponse,
    CreateResponseObject, CreateResponseObjectInput, ModelIdsResponses, ModelIdsShared,
    ModelResponseProperties, Response, ResponseProperties,
};

#[test]
pub fn model_response_properties_with_default_serialize() {
    let model_response_properties = ModelResponseProperties {
        metadata: None,
        top_logprobs: None,
        temperature: None,
        top_p: None,
        user: Some(format!("me")),
        service_tier: None,
    };
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "user": "me"
        }
    "#,
    )
    .unwrap();

    let serialized_value = serde_json::to_value(model_response_properties).unwrap();

    assert_eq!(serialized_value, expected);
}

#[test]
pub fn model_response_properties_serialize() {
    let mut metadata = HashMap::new();
    metadata.insert("a".to_string(), "b".to_string());
    let model_response_properties = ModelResponseProperties {
        metadata: Some(metadata),
        top_logprobs: Some(1),
        temperature: Some(0.5),
        top_p: Some(1.0),
        user: Some(format!("me")),
        service_tier: None,
    };
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "metadata": {"a":"b"},
            "top_logprobs": 1,
            "temperature": 0.5,
            "top_p": 1.0,
            "user": "me"
        }
    "#,
    )
    .unwrap();

    let serialized_value = serde_json::to_value(model_response_properties).unwrap();

    assert_eq!(serialized_value, expected);
}

#[test]
pub fn create_model_response_properties_serialize() {
    let mut metadata = HashMap::new();
    metadata.insert("a".to_string(), "b".to_string());
    let create_model_response_properties = CreateModelResponseProperties {
        model_response_properties: ModelResponseProperties {
            metadata: Some(metadata),
            top_logprobs: Some(1),
            temperature: Some(0.5),
            top_p: Some(1.0),
            user: Some(format!("me")),
            service_tier: None,
        },
        object: CreateModelResponsePropertiesObject {
            top_logprobs: Some(10),
        },
    };
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "metadata": {"a":"b"},
            "top_logprobs": 1,
            "temperature": 0.5,
            "top_p": 1.0,
            "user": "me",
            "top_logprobs": 10
        }
    "#,
    )
    .unwrap();

    let serialized_value = serde_json::to_value(create_model_response_properties).unwrap();

    assert_eq!(serialized_value, expected);
}

#[test]
pub fn create_response_serialize() {
    let create_response = CreateResponse {
        create_model_response_properties: CreateModelResponseProperties {
            model_response_properties: ModelResponseProperties {
                metadata: None,
                top_logprobs: None,
                temperature: None,
                top_p: None,
                user: None,
                service_tier: None,
            },
            object: CreateModelResponsePropertiesObject { top_logprobs: None },
        },
        response_properties: ResponseProperties {
            previous_response_id: None,
            model: Some(ModelIdsResponses::ModelIdsShared(ModelIdsShared::String(
                "gpt-4o".to_string(),
            ))),
            reasoning: None,
            background: None,
            max_output_tokens: None,
            max_tool_calls: None,
            text: None,
            tools: None,
            tool_choice: None,
            prompt: None,
            truncation: None,
        },
        object: CreateResponseObject {
            input: Some(CreateResponseObjectInput::String(
                "What is the capital of France?".to_string(),
            )),
            include: None,
            parallel_tool_calls: None,
            store: None,
            instructions: None,
            stream: None,
        },
    };
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "model": "gpt-4o",
            "input": "What is the capital of France?"
        }
    "#,
    )
    .unwrap();

    let serialized_value = serde_json::to_value(create_response).unwrap();
    assert_eq!(serialized_value, expected);
}

#[test]
pub fn create_response_deserialize_response() {
    let response = "{\n  \"id\": \"resp_68775ff0850081a0b3234aeece4b9bd60742b21b8afa9f8d\",\n  \"object\": \"response\",\n  \"created_at\": 1752653808,\n  \"status\": \"completed\",\n  \"background\": false,\n  \"error\": null,\n  \"incomplete_details\": null,\n  \"instructions\": null,\n  \"max_output_tokens\": null,\n  \"max_tool_calls\": null,\n  \"model\": \"gpt-4.1-2025-04-14\",\n  \"output\": [\n    {\n      \"id\": \"msg_68775ff0f62c81a0bec78fba7fa8219d0742b21b8afa9f8d\",\n      \"type\": \"message\",\n      \"status\": \"completed\",\n      \"content\": [\n        {\n          \"type\": \"output_text\",\n          \"annotations\": [],\n          \"logprobs\": [],\n          \"text\": \"The capital of France is **Paris**.\"\n        }\n      ],\n      \"role\": \"assistant\"\n    }\n  ],\n  \"parallel_tool_calls\": true,\n  \"previous_response_id\": null,\n  \"reasoning\": {\n    \"effort\": null,\n    \"summary\": null\n  },\n  \"service_tier\": \"default\",\n  \"store\": true,\n  \"temperature\": 1.0,\n  \"text\": {\n    \"format\": {\n      \"type\": \"text\"\n    }\n  },\n  \"tool_choice\": \"auto\",\n  \"tools\": [],\n  \"top_logprobs\": 0,\n  \"top_p\": 1.0,\n  \"truncation\": \"disabled\",\n  \"usage\": {\n    \"input_tokens\": 14,\n    \"input_tokens_details\": {\n      \"cached_tokens\": 0\n    },\n    \"output_tokens\": 10,\n    \"output_tokens_details\": {\n      \"reasoning_tokens\": 0\n    },\n    \"total_tokens\": 24\n  },\n  \"user\": null,\n  \"metadata\": {}\n}";
    serde_json::from_str::<Response>(response).unwrap();
}
