use std::{fs::File, io::Write};

use conversa_openai_client::{
    OpenAIClient, OpenAIClientBuilder,
    client::{CreateChatCompletionResponse, CreateResponseResponse},
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
        CreateChatCompletionRequestObject, CreateImageRequest, CreateImageRequestModel,
        CreateImageRequestOutputFormat, CreateImageRequestSize, CreateModelResponseProperties,
        CreateModelResponsePropertiesObject, CreateResponse, CreateResponseObject,
        CreateResponseObjectInput, ModelIdsResponses, ModelIdsShared, ModelResponseProperties,
        OutputContent, OutputItem, ResponseProperties,
    },
};

fn create_openai_client() -> OpenAIClient {
    OpenAIClientBuilder::new(
        "https://api.openai.com/v1".to_string(),
        std::env::var("OPENAI_API_KEY").unwrap(),
    )
    .build()
    .unwrap()
}

#[tokio::test]
#[ignore]
async fn get_models() {
    let client = create_openai_client();
    let list_models = client.list_models().await.unwrap();
    assert!(list_models.data.iter().any(|m| m.id == "gpt-4.1"));
}

#[tokio::test]
#[ignore]
async fn create_response() {
    let client = create_openai_client();
    let request_body = CreateResponse {
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
                "gpt-4.1-nano".to_string(),
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

    let response = client.create_response(request_body).await.unwrap();
    if let CreateResponseResponse::ApplicationJson(json_response) = response {
        assert_eq!(json_response.object.output.len(), 1);
        let output_object = &json_response.object.output[0];
        if let OutputItem::OutputMessage(output_message) = output_object {
            assert_eq!(output_message.content.len(), 1);
            let output_message_content = &output_message.content[0];
            if let OutputContent::OutputTextContent(output_text_content) = output_message_content {
                assert_eq!(output_text_content.text, "The capital of France is Paris.");
            } else {
                panic!("Invalid output message content: {output_message_content:?}",)
            }
        } else {
            panic!("Invalid output object: {output_object:?}",)
        }
    } else {
        panic!("Invalid response: {response:?}",)
    }
}

#[tokio::test]
#[ignore]
async fn create_image() {
    let client = create_openai_client();
    let request_body = CreateImageRequest {
        prompt: "Generate an image of a cleanly designed hardware in the loop (HIL) simulator connected to an external control board with a flat ribbon cable. Add a screen showing plots of typical current and speed curves typically observed in motor control.".to_string(),
        model: Some(CreateImageRequestModel::String("gpt-image-1".to_string())),
        n: Some(1),
        quality: None,
        response_format: None,
        output_format: Some(CreateImageRequestOutputFormat::Webp),
        output_compression: None,
        size: Some(CreateImageRequestSize::Size1024x1024),
        moderation: None,
        background: None,
        style: None,
        user: None,
    };

    let response = client.create_image(request_body).await.unwrap();
    let mut file = File::create("image_response_output.txt").unwrap();
    write!(file, "{response:?}",).unwrap();
}

#[tokio::test]
#[ignore]
async fn create_chat_completion() {
    let client = create_openai_client();
    let request_body = CreateChatCompletionRequest {
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
        object: CreateChatCompletionRequestObject {
            messages: vec![
                ChatCompletionRequestMessage::ChatCompletionRequestUserMessage(
                    ChatCompletionRequestUserMessage {
                        content: ChatCompletionRequestUserMessageContent::String(String::from("What is the capital of France?")),
                        role: conversa_openai_client::types::ChatCompletionRequestUserMessageRole::User,
                        name: None
                    },
                ),
            ],
            model: ModelIdsShared::String("gpt-4.1".to_string()),
            modalities: None,
            reasoning_effort: None,
            max_completion_tokens: None,
            frequency_penalty: None,
            presence_penalty: None,
            web_search_options: None,
            top_logprobs: None,
            response_format: None,
            audio: None,
            store: None,
            stream: None,
            stop: None,
            logit_bias: None,
            logprobs: None,
            max_tokens: None,
            n: None,
            prediction: None,
            seed: None,
            stream_options: None,
            tools: None,
            tool_choice: None,
            parallel_tool_calls: None,
            function_call: None,
            functions: None,
        },
    };

    let response = client.create_chat_completion(request_body).await.unwrap();
    if let CreateChatCompletionResponse::ApplicationJson(json_response) = response {
        assert_eq!(json_response.choices.len(), 1);
        let output_choice = &json_response.choices[0];
        assert_eq!(
            output_choice.message.content,
            Some(String::from("The capital of France is **Paris**."))
        );
    } else {
        panic!("Invalid response: {response:?}",)
    }
}
