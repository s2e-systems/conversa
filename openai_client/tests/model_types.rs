use conversa_openai_client::types::{
    DeleteModelResponse, ListModelsResponse, ListModelsResponseObject, Model, ModelObject,
};

#[test]
pub fn model_json_serialize() {
    let model = Model {
        id: "VAR_chat_model_id".to_string(),
        object: ModelObject::Model,
        created: 1686935002,
        owned_by: "openai".to_string(),
    };
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "id": "VAR_chat_model_id",
            "object": "model",
            "created": 1686935002,
            "owned_by": "openai"
          }
    "#,
    )
    .unwrap();

    let serialized_value = serde_json::to_value(model).unwrap();

    assert_eq!(serialized_value, expected);
}

#[test]
pub fn list_model_response_json_serialize() {
    let list_model_response = ListModelsResponse {
        object: ListModelsResponseObject::List,
        data: vec![
            Model {
                id: "VAR_chat_model_id".to_string(),
                object: ModelObject::Model,
                created: 1686935002,
                owned_by: "openai".to_string(),
            },
            Model {
                id: "chat_model".to_string(),
                object: ModelObject::Model,
                created: 1234567890,
                owned_by: "s2e".to_string(),
            },
        ],
    };

    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "object": "list",
            "data": [
                {
                    "id": "VAR_chat_model_id",
                    "object": "model",
                    "created": 1686935002,
                    "owned_by": "openai"
                },
                {
                    "id": "chat_model",
                    "object": "model",
                    "created": 1234567890,
                    "owned_by": "s2e"
                } 
            ]
          }
    "#,
    )
    .unwrap();

    let serialized_value = serde_json::to_value(list_model_response).unwrap();

    assert_eq!(serialized_value, expected);
}

#[test]
pub fn delete_model_response_json_serialize() {
    let delete_model_response = DeleteModelResponse {
        id: "ft:gpt-4o-mini:acemeco:suffix:abc123".to_string(),
        object: "model".to_string(),
        deleted: true,
    };
    let expected: serde_json::Value = serde_json::from_str(
        r#"
        {
            "id": "ft:gpt-4o-mini:acemeco:suffix:abc123",
            "object": "model",
            "deleted": true
          }
    "#,
    )
    .unwrap();

    let serialized_value = serde_json::to_value(delete_model_response).unwrap();

    assert_eq!(serialized_value, expected);
}
