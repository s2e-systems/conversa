use std::{
    fs::{File, read_to_string},
    io::Write,
};

use yaml_rust::{Yaml, YamlLoader};

const OPENAI_YML_FILE_PATH: &str = "./openapi.documented.yml";

fn camel_to_snake(s: &str) -> String {
    let mut snake = String::new();

    for (i, ch) in s.char_indices() {
        if ch.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            snake.extend(ch.to_lowercase());
        } else {
            snake.push(ch);
        }
    }

    snake
}

fn str_to_camel_case(s: &str) -> String {
    let mut camel = String::new();

    let mut next_is_uppercase = false;
    for (i, ch) in s.char_indices() {
        if ch == '[' || ch == ']' {
            continue;
        }

        if i == 0 {
            if ch.is_numeric() {
                camel.push_str("Size");
                camel.push(ch);
            } else if ch == '/' {
                next_is_uppercase = true;
            } else {
                camel.extend(ch.to_uppercase())
            }
        } else if ch == '_' || ch == '.' || ch == '-' || ch == '/' {
            next_is_uppercase = true;
        } else if next_is_uppercase {
            camel.extend(ch.to_uppercase());
            next_is_uppercase = false;
        } else {
            camel.push(ch);
        }
    }

    camel
}

fn get_object_name_from_reference(reference: &str) -> &str {
    let i = reference.rfind("/").unwrap();
    &reference[i + 1..]
}

fn str_to_snake_case(s: &str) -> String {
    let mut snake = String::new();

    for (i, ch) in s.char_indices() {
        if ch == '_' || ch == '.' || ch == '-' {
            snake.push('_');
        } else {
            if ch.is_uppercase() && i != 0 {
                snake.push('_');
            }
            snake.extend(ch.to_lowercase())
        }
    }

    snake
}

fn generate_inner_object_name(object_name: &str, field_name: &str) -> String {
    let camel_field_name = str_to_camel_case(field_name);
    format!("{}{}", object_name, camel_field_name)
}

fn parse_string_enum(name: &str, schema: &Yaml, output_file: &mut File) {
    let enum_items = schema
        .as_hash()
        .unwrap()
        .get(&Yaml::String("enum".to_string()))
        .unwrap()
        .as_vec()
        .unwrap();
    writeln!(
        output_file,
        "#[derive(Debug, PartialEq, Serialize, Deserialize)]"
    )
    .unwrap();
    writeln!(output_file, "pub enum {} {{", name).unwrap();

    for item in enum_items {
        writeln!(
            output_file,
            "\t#[serde(rename=\"{}\")]",
            item.as_str().unwrap()
        )
        .unwrap();
        writeln!(
            output_file,
            "\t{},",
            str_to_camel_case(item.as_str().unwrap())
        )
        .unwrap();
    }

    writeln!(output_file, "}}\n").unwrap();
}

fn parse_typedef_type(name: &str, schema: &Yaml, output_file: &mut File) {
    if let Some(doc) = schema["description"].as_str() {
        writeln!(
            output_file,
            "/** {} */",
            doc.trim_end().replace("```", "***")
        )
        .unwrap();
    }

    let type_name = schema["type"].as_str().unwrap();
    match type_name {
        "string" => {
            if let Some("map") = schema["x-oaiTypeLabel"].as_str() {
                writeln!(
                    output_file,
                    "pub type {} = HashMap<String, String>;\n",
                    name
                )
                .unwrap()
            } else {
                writeln!(output_file, "pub type {} = String;\n", name).unwrap()
            }
        }
        "array" => {
            let array_items = schema["items"].as_hash().unwrap();
            if let Some(typeref) = array_items.get(&Yaml::String("$ref".to_string())) {
                writeln!(
                    output_file,
                    "pub type {} = Vec<{}>;\n",
                    name,
                    get_object_name_from_reference(typeref.as_str().unwrap())
                )
                .unwrap();
            } else if array_items
                .get(&Yaml::String("type".to_string()))
                .unwrap()
                .as_str()
                .unwrap()
                == "string"
            {
                writeln!(output_file, "pub type {} = Vec<String>;\n", name,).unwrap();
            }
        }
        "boolean" => writeln!(output_file, "pub type {} = bool;\n", name).unwrap(),
        _ => unimplemented!("{}", type_name),
    }
}

fn parse_object_type(name: &str, schema: &Yaml, output_file: &mut File) {
    let schema_map = schema.as_hash().unwrap();
    if let Some(schema_properties) = schema_map.get(&Yaml::String("properties".to_string())) {
        let schema_properties_map = schema_properties.as_hash().unwrap();

        // We have to iterate through all the fields and start by generating all the inner object types
        for (property_name, property_value) in schema_properties_map {
            let property_name = property_name.as_str().unwrap();
            let property_hash = property_value.as_hash().unwrap();
            if let Some(property_type) = property_hash.get(&Yaml::String("type".to_string())) {
                if property_type == &Yaml::String("object".to_string()) {
                    parse_object_type(
                        &generate_inner_object_name(name, property_name),
                        property_value,
                        output_file,
                    )
                } else if property_type == &Yaml::String("string".to_string()) {
                    if property_hash
                        .get(&Yaml::String("enum".to_string()))
                        .is_some()
                    {
                        parse_string_enum(
                            &generate_inner_object_name(name, property_name),
                            property_value,
                            output_file,
                        );
                    }
                } else if property_type == &Yaml::String("array".to_string()) {
                    let property_items = property_hash
                        .get(&Yaml::String("items".to_string()))
                        .unwrap();
                    let items_hash = property_items.as_hash().unwrap();

                    if items_hash.get(&Yaml::String("type".to_string()))
                        == Some(&Yaml::String("object".to_string()))
                    {
                        parse_object_type(
                            &generate_inner_object_name(name, property_name),
                            property_value,
                            output_file,
                        );
                    } else if items_hash.get(&Yaml::String("oneOf".to_string())).is_some() {
                        parse_oneof_type(
                            &generate_inner_object_name(name, property_name),
                            property_items,
                            output_file,
                        );
                    } else if items_hash.get(&Yaml::String("allOf".to_string())).is_some() {
                        parse_allof_type(
                            &generate_inner_object_name(name, property_name),
                            property_items,
                            output_file,
                        );
                    }
                }
            } else if property_hash
                .get(&Yaml::String("oneOf".to_string()))
                .is_some()
                || property_hash
                    .get(&Yaml::String("anyOf".to_string()))
                    .is_some()
            {
                parse_oneof_type(
                    &generate_inner_object_name(name, property_name),
                    property_value,
                    output_file,
                );
            } else if property_hash
                .get(&Yaml::String("allOf".to_string()))
                .is_some()
            {
                parse_allof_type(
                    &generate_inner_object_name(name, property_name),
                    property_value,
                    output_file,
                );
            }
        }

        if let Some(doc) = schema_map
            .get(&Yaml::String("description".to_string()))
            .map(|x| x.as_str().unwrap().trim_end().replace("```", "***"))
        {
            writeln!(output_file, "/** {} */", doc).unwrap();
        }

        writeln!(
            output_file,
            "#[derive(Debug, PartialEq, Serialize, Deserialize)]"
        )
        .unwrap();
        writeln!(output_file, "pub struct {} {{", name).unwrap();

        let object_required_list = schema_map
            .get(&Yaml::String("required".to_string()))
            .map(|s| s.as_vec().unwrap().clone())
            .unwrap_or_default();

        for (property_name, property_value) in schema_properties_map {
            let mut field_name = property_name.as_str().unwrap().to_string();
            let property_hash = property_value.as_hash().unwrap();
            let field_type = if let Some(field_type_yaml) =
                property_hash.get(&Yaml::String("type".to_string()))
            {
                let field_type_str = field_type_yaml.as_str().unwrap();
                match field_type_str {
                    "string" => {
                        if property_hash
                            .get(&Yaml::String("enum".to_string()))
                            .is_some()
                        {
                            generate_inner_object_name(name, &field_name)
                        } else if property_hash.get(&Yaml::String("format".to_string()))
                            == Some(&Yaml::String("binary".to_string()))
                        {
                            "Vec<u8>".to_string()
                        } else {
                            "String".to_string()
                        }
                    }
                    "integer" => "u64".to_string(),
                    "object" => generate_inner_object_name(name, &field_name),
                    "array" => {
                        let items_hash = property_hash
                            .get(&Yaml::String("items".to_string()))
                            .unwrap()
                            .as_hash()
                            .unwrap();

                        if let Some(item_type) = items_hash.get(&Yaml::String("type".to_string())) {
                            let item_type = item_type.as_str().unwrap();
                            match item_type {
                                "string" => "Vec<String>".to_string(),
                                "integer" => "Vec<u64>".to_string(),
                                "number" => "Vec<f32>".to_string(),
                                "object" => {
                                    format!(
                                        "Vec<{}>",
                                        generate_inner_object_name(name, &field_name)
                                    )
                                }
                                _ => unimplemented!("Array variant with type {}", item_type),
                            }
                        } else if let Some(item_ref) =
                            items_hash.get(&Yaml::String("$ref".to_string()))
                        {
                            format!(
                                "Vec<{}>",
                                get_object_name_from_reference(item_ref.as_str().unwrap())
                            )
                        } else if items_hash.get(&Yaml::String("oneOf".to_string())).is_some()
                            || items_hash.get(&Yaml::String("allOf".to_string())).is_some()
                        {
                            format!("Vec<{}>", generate_inner_object_name(name, &field_name))
                        } else {
                            unimplemented!()
                        }
                    }
                    "boolean" => "bool".to_string(),
                    "number" => "f32".to_string(),
                    _ => unimplemented!("Object {} with field type {}", name, field_type_str),
                }
            } else if let Some(field_ref) = property_hash.get(&Yaml::String("$ref".to_string())) {
                get_object_name_from_reference(field_ref.as_str().unwrap()).to_string()
            } else if property_hash
                .get(&Yaml::String("oneOf".to_string()))
                .is_some()
                || property_hash
                    .get(&Yaml::String("anyOf".to_string()))
                    .is_some()
                || property_hash
                    .get(&Yaml::String("allOf".to_string()))
                    .is_some()
            {
                generate_inner_object_name(name, property_name.as_str().unwrap()).to_string()
            } else {
                unimplemented!("{:?} {:?}", property_name, property_value)
            };
            if field_name == "type" {
                writeln!(output_file, "\t#[serde(rename=\"{}\")]", field_name,).unwrap();
                field_name = "r#type".to_string();
            } else if field_name == "static" {
                writeln!(output_file, "\t#[serde(rename=\"{}\")]", field_name,).unwrap();
                field_name = "r#static".to_string();
            } else if field_name.contains('.') {
                writeln!(output_file, "\t#[serde(rename=\"{}\")]", field_name,).unwrap();
                field_name = field_name.replace('.', "_");
            } else if field_name.contains("[]") {
                writeln!(output_file, "\t#[serde(rename=\"{}\")]", field_name,).unwrap();
                field_name = field_name.replace("[]", "");
            }

            if let Some(doc) = property_hash
                .get(&Yaml::String("description".to_string()))
                .map(|x| x.as_str().unwrap().trim_end().replace("```", "***"))
            {
                writeln!(output_file, "\t/** {} */", doc).unwrap();
            }

            let is_nullable = if let Some(Yaml::Boolean(n)) =
                property_hash.get(&Yaml::String("nullable".to_string()))
            {
                *n
            } else {
                false
            };

            if object_required_list.contains(property_name) && !is_nullable {
                writeln!(output_file, "\tpub {}: {},", field_name, field_type).unwrap();
            } else {
                writeln!(
                    output_file,
                    "\t#[serde(skip_serializing_if = \"Option::is_none\")]"
                )
                .unwrap();
                writeln!(output_file, "\tpub {}: Option<{}>,", field_name, field_type).unwrap();
            }
        }
        writeln!(output_file, "}}\n").unwrap();
    } else {
        // We assume structs without properties are just typically to wrap strings containing a Json map
        if let Some(doc) = schema_map
            .get(&Yaml::String("description".to_string()))
            .map(|x| x.as_str().unwrap().trim_end().replace("```", "***"))
        {
            writeln!(output_file, "\t/** {} */", doc).unwrap();
        }

        writeln!(
            output_file,
            "#[derive(Debug, PartialEq, Serialize, Deserialize)]"
        )
        .unwrap();
        writeln!(output_file, "pub struct {}(pub String);\n", name).unwrap();
    }
}

fn parse_oneof_type(name: &str, schema: &Yaml, output_file: &mut File) {
    let schema_map = schema.as_hash().unwrap();
    let one_of_list = schema_map
        .get(&Yaml::String("oneOf".to_string()))
        .or(schema_map.get(&Yaml::String("anyOf".to_string())))
        .unwrap()
        .as_vec()
        .unwrap();

    // First iterate through the variants to create the inner object types if needed
    for one_of_variant in one_of_list {
        let one_of_variant_hash = one_of_variant.as_hash().unwrap();
        if one_of_variant_hash.get(&Yaml::String("type".to_string()))
            == Some(&Yaml::String("object".to_string()))
        {
            if let Some(Yaml::Hash(schema_properties)) =
                one_of_variant_hash.get(&Yaml::String("properties".to_string()))
            {
                for (property_name, property_value) in schema_properties {
                    let property_hash = property_value.as_hash().unwrap();
                    if property_hash.get(&Yaml::String("type".to_string()))
                        == Some(&Yaml::String("array".to_string()))
                    {
                        let property_items = property_hash
                            .get(&Yaml::String("items".to_string()))
                            .unwrap();
                        if property_items
                            .as_hash()
                            .unwrap()
                            .get(&Yaml::String("oneOf".to_string()))
                            .is_some()
                        {
                            parse_oneof_type(
                                &generate_inner_object_name(name, property_name.as_str().unwrap()),
                                property_items,
                                output_file,
                            );
                        }
                    } else if property_hash.get(&Yaml::String("type".to_string()))
                        == Some(&Yaml::String("object".to_string()))
                    {
                        parse_object_type(
                            &generate_inner_object_name(name, property_name.as_str().unwrap()),
                            property_value,
                            output_file,
                        );
                    }
                }
            }
        } else if one_of_variant_hash.get(&Yaml::String("type".to_string()))
            == Some(&Yaml::String("array".to_string()))
        {
            let property_items = one_of_variant_hash
                .get(&Yaml::String("items".to_string()))
                .unwrap();
            if property_items
                .as_hash()
                .unwrap()
                .get(&Yaml::String("oneOf".to_string()))
                .is_some()
            {
                parse_oneof_type(
                    &generate_inner_object_name(name, "Array"),
                    property_items,
                    output_file,
                );
            }
        }
    }

    if let Some(doc) = schema_map
        .get(&Yaml::String("description".to_string()))
        .map(|x| x.as_str().unwrap().trim_end().replace("```", "***"))
    {
        writeln!(output_file, "/** {} */", doc).unwrap();
    }

    writeln!(
        output_file,
        "#[derive(Debug, PartialEq, Serialize, Deserialize)]"
    )
    .unwrap();
    writeln!(output_file, "#[serde(untagged)]").unwrap();
    writeln!(output_file, "pub enum {} {{", name).unwrap();

    for (index, one_of_variant) in one_of_list.iter().enumerate() {
        let one_of_variant_hash = one_of_variant.as_hash().unwrap();
        if let Some(doc) = one_of_variant_hash
            .get(&Yaml::String("description".to_string()))
            .map(|x| x.as_str().unwrap().trim_end().replace("```", "***"))
        {
            writeln!(output_file, "\t/** {} */", doc).unwrap();
        }

        if let Some(variant_ref) = one_of_variant_hash.get(&Yaml::String("$ref".to_string())) {
            let variant_name = get_object_name_from_reference(variant_ref.as_str().unwrap());
            writeln!(output_file, "\t{}({}),", variant_name, variant_name).unwrap();
        } else if one_of_variant_hash
            .get(&Yaml::String("$recursiveRef".to_string()))
            .is_some()
        {
            // This is hardcoded since it is the single usage and it would be complicated to
            // walk back the tree to retrieve the name.
            writeln!(output_file, "\tRecursive(CompoundFilter),").unwrap();
        } else if let Some(Yaml::String(variant_type)) =
            one_of_variant_hash.get(&Yaml::String("type".to_string()))
        {
            match variant_type.as_str() {
                "string" => {
                    // Some variants have two String types to account for enumerations but for
                    // our type this is not necessary because all String representations are
                    // equal
                    if let Some(Yaml::Array(enum_list)) =
                        one_of_variant_hash.get(&Yaml::String("enum".to_string()))
                    {
                        for string_variant in enum_list {
                            writeln!(
                                output_file,
                                "\t#[serde(rename=\"{}\")]",
                                string_variant.as_str().unwrap()
                            )
                            .unwrap();
                            writeln!(
                                output_file,
                                "\t{},",
                                str_to_camel_case(string_variant.as_str().unwrap())
                            )
                            .unwrap();
                        }
                    } else {
                        writeln!(output_file, "\tString(String),").unwrap();
                    }
                }
                "integer" => {
                    writeln!(output_file, "\tInteger(u64),").unwrap();
                }
                "number" => {
                    writeln!(output_file, "\tNumber(f32),").unwrap();
                }
                "boolean" => {
                    writeln!(output_file, "\tBoolean(bool),").unwrap();
                }
                "null" => {
                    writeln!(output_file, "\tNone,").unwrap();
                }
                "object" => {
                    let variant_title = if let Some(t) =
                        one_of_variant_hash.get(&Yaml::String("title".to_string()))
                    {
                        t.as_str().unwrap().to_string().replace(" ", "")
                    } else {
                        (char::from(b'A' + index as u8)).to_string()
                    };
                    let schema_properties = if let Some(s) =
                        one_of_variant_hash.get(&Yaml::String("properties".to_string()))
                    {
                        s
                    } else {
                        // If there are no properties we simply assume this is a map
                        writeln!(output_file, "\tMap(String),").unwrap();
                        continue;
                    };

                    writeln!(output_file, "\t{} {{", variant_title).unwrap();
                    let schema_properties_map = schema_properties.as_hash().unwrap();
                    for (property_name, property_value) in schema_properties_map {
                        let mut property_name = property_name.as_str().unwrap();

                        let property_hash = property_value.as_hash().unwrap();
                        let property_type = if let Some(Yaml::String(property_type)) =
                            property_hash.get(&Yaml::String("type".to_string()))
                        {
                            match property_type.as_str() {
                                "string" => "String".to_string(),
                                "boolean" => "bool".to_string(),
                                "array" => {
                                    let items_hash = property_hash
                                        .get(&Yaml::String("items".to_string()))
                                        .unwrap()
                                        .as_hash()
                                        .unwrap();
                                    if let Some(Yaml::String(item_type)) =
                                        items_hash.get(&Yaml::String("type".to_string()))
                                    {
                                        match item_type.as_str() {
                                            "string" => "Vec<String>".to_string(),
                                            _ => unimplemented!("{}", item_type),
                                        }
                                    } else if items_hash
                                        .get(&Yaml::String("oneOf".to_string()))
                                        .is_some()
                                    {
                                        format!(
                                            "Vec<{}>",
                                            generate_inner_object_name(name, property_name)
                                        )
                                    } else {
                                        unimplemented!("{:?}", items_hash)
                                    }
                                }
                                "object" => generate_inner_object_name(name, property_name),
                                _ => unimplemented!("{:?}", property_type),
                            }
                        } else if let Some(property_ref) =
                            property_hash.get(&Yaml::String("$ref".to_string()))
                        {
                            get_object_name_from_reference(property_ref.as_str().unwrap())
                                .to_string()
                        } else {
                            unimplemented!(
                                "Variant object {:?} from {:?} with properties {:?}",
                                property_name,
                                name,
                                property_hash
                            )
                        };
                        if property_name == "type" {
                            writeln!(output_file, "\t\t#[serde(rename=\"{}\")]", property_name)
                                .unwrap();
                            property_name = "r#type";
                        }
                        writeln!(output_file, "\t\t{}: {},", property_name, property_type).unwrap();
                    }

                    writeln!(output_file, "\t}},",).unwrap();
                }
                "array" => {
                    let items_hash = one_of_variant_hash
                        .get(&Yaml::String("items".to_string()))
                        .unwrap()
                        .as_hash()
                        .unwrap();

                    if let Some(Yaml::String(item_type)) =
                        items_hash.get(&Yaml::String("type".to_string()))
                    {
                        match item_type.as_str() {
                            "string" => {
                                writeln!(output_file, "\tArrayString(Vec<String>),").unwrap()
                            }
                            "integer" => writeln!(output_file, "\tArrayNumber(Vec<u64>),").unwrap(),
                            "array" => {
                                let items_items_hash = items_hash
                                    .get(&Yaml::String("items".to_string()))
                                    .unwrap()
                                    .as_hash()
                                    .unwrap();
                                let items_items_type = items_items_hash
                                    .get(&Yaml::String("type".to_string()))
                                    .unwrap()
                                    .as_str()
                                    .unwrap();
                                match items_items_type {
                                    "integer" => {
                                        writeln!(output_file, "\tArrayListNumber(Vec<Vec<u64>>),")
                                            .unwrap()
                                    }
                                    _ => unimplemented!("{}", items_items_type),
                                }
                            }
                            _ => unimplemented!("Array variant with type {}", item_type),
                        }
                    } else if let Some(Yaml::String(item_ref)) =
                        items_hash.get(&Yaml::String("$ref".to_string()))
                    {
                        let variant_name = get_object_name_from_reference(item_ref);
                        writeln!(output_file, "\tArrayList(Vec<{}>),", variant_name).unwrap();
                    } else if items_hash.get(&Yaml::String("oneOf".to_string())).is_some() {
                        writeln!(
                            output_file,
                            "\tArrayList(Vec<{}>),",
                            generate_inner_object_name(name, "Array")
                        )
                        .unwrap();
                    } else {
                        unimplemented!("{:?}", items_hash)
                    }
                }
                _ => panic!("{:?}", variant_type),
            }
        } else {
            unimplemented!("{:?}", one_of_variant_hash)
        }
    }
    writeln!(output_file, "}}\n").unwrap();
}

fn parse_allof_type(name: &str, schema: &Yaml, output_file: &mut File) {
    let schema_map = schema.as_hash().unwrap();
    let all_of_list = schema_map
        .get(&Yaml::String("allOf".to_string()))
        .unwrap()
        .as_vec()
        .unwrap();

    for all_of_item in all_of_list {
        let all_of_item_hash = all_of_item.as_hash().unwrap();
        if all_of_item_hash.get(&Yaml::String("type".to_string()))
            == Some(&Yaml::String("object".to_string()))
        {
            parse_object_type(
                &generate_inner_object_name(name, "Object"),
                all_of_item,
                output_file,
            );
        }
    }

    if let Some(doc) = schema_map
        .get(&Yaml::String("description".to_string()))
        .map(|x| x.as_str().unwrap().trim_end().replace("```", "***"))
    {
        writeln!(output_file, "/** {} */", doc).unwrap();
    }

    writeln!(
        output_file,
        "#[derive(Debug, PartialEq, Serialize, Deserialize)]"
    )
    .unwrap();
    writeln!(output_file, "pub struct {} {{", name).unwrap();

    for all_of_item in all_of_list {
        let all_of_item_hash = all_of_item.as_hash().unwrap();
        if let Some(item_ref) = all_of_item_hash.get(&Yaml::String("$ref".to_string())) {
            let item_name = get_object_name_from_reference(item_ref.as_str().unwrap());
            let field_name = camel_to_snake(item_name);
            writeln!(output_file, "\t#[serde(flatten)]").unwrap();
            writeln!(output_file, "\tpub {}: {},", field_name, item_name).unwrap();
        } else if let Some(Yaml::String(variant_type)) =
            all_of_item_hash.get(&Yaml::String("type".to_string()))
        {
            match variant_type.as_str() {
                "object" => {
                    writeln!(output_file, "\t#[serde(flatten)]").unwrap();
                    writeln!(
                        output_file,
                        "\tpub object: {},",
                        generate_inner_object_name(name, "Object")
                    )
                    .unwrap();
                }
                _ => unimplemented!("allOf variant type: {}", variant_type),
            }
        }
    }

    writeln!(output_file, "}}\n").unwrap();
}

fn parse_component_schema(schema_name: &str, schema_value: &Yaml, output_file: &mut File) {
    let schema_value_map = schema_value.as_hash().unwrap();
    if let Some(Yaml::String(schema_type)) = schema_value_map.get(&Yaml::String("type".to_string()))
    {
        match schema_type.as_str() {
            "object" => {
                parse_object_type(schema_name, schema_value, output_file);
            }
            "string" | "array" | "boolean" => {
                parse_typedef_type(schema_name, schema_value, output_file);
            }
            _ => unimplemented!(),
        }
    } else if let Some(_schema_all_of) = schema_value_map.get(&Yaml::String("allOf".to_string())) {
        parse_allof_type(schema_name, schema_value, output_file);
    } else if schema_value_map
        .get(&Yaml::String("oneOf".to_string()))
        .is_some()
        || schema_value_map
            .get(&Yaml::String("anyOf".to_string()))
            .is_some()
    {
        parse_oneof_type(schema_name, schema_value, output_file);
    } else {
        unimplemented!("Invalid object")
    }
}

fn parse_endpoint_path(path_schema: &Yaml, client_output_file: &mut File) {
    writeln!(
        client_output_file,
        "use crate::{{ConversaError, ConversaResult, OpenAIClient}};"
    )
    .unwrap();
    writeln!(client_output_file, "use crate::types::*;").unwrap();

    writeln!(
        client_output_file,
        "use serde::{{Serialize, Deserialize}};\n"
    )
    .unwrap();

    let schema_list = path_schema.as_hash().unwrap();

    // Before implementing the functions the additional types need to be defined
    for (path_name, path_hash) in schema_list {
        println!("{:?}", path_name);
        let path_operations = path_hash.as_hash().unwrap();
        for (_, path_operation_hash) in path_operations {
            let operation_name =
                str_to_snake_case(path_operation_hash["operationId"].as_str().unwrap());

            if let Some(Yaml::Array(parameters_list)) = path_operation_hash
                .as_hash()
                .unwrap()
                .get(&Yaml::String("parameters".to_string()))
            {
                for parameter in parameters_list {
                    let parameter_schema = parameter["schema"].as_hash().unwrap();
                    if let Some(Yaml::String(schema_type)) =
                        parameter_schema.get(&Yaml::String("type".to_string()))
                    {
                        if schema_type == "object" {
                            parse_object_type(
                                &str_to_camel_case(&format!("{operation_name}_query")),
                                &parameter["schema"],
                                client_output_file,
                            );
                        }
                    }
                }
            }
            if let Some(request_body_hash) = path_operation_hash
                .as_hash()
                .unwrap()
                .get(&Yaml::String("requestBody".to_string()))
            {
                let request_body_content = request_body_hash["content"].as_hash().unwrap();
                assert_eq!(request_body_content.len(), 1);
                let body_content_schema = &request_body_content.front().unwrap().1["schema"];
                if body_content_schema
                    .as_hash()
                    .unwrap()
                    .get(&Yaml::String("type".to_string()))
                    == Some(&Yaml::String("object".to_string()))
                {
                    parse_object_type(
                        &str_to_camel_case(&format!("{operation_name}_request_body")),
                        body_content_schema,
                        client_output_file,
                    );
                }
            }

            let responses_hash = path_operation_hash["responses"].as_hash().unwrap();
            if let Some(ok_response) = responses_hash.get(&Yaml::String("200".to_string())) {
                if let Some(Yaml::Hash(response_content_hash)) = ok_response
                    .as_hash()
                    .unwrap()
                    .get(&Yaml::String("content".to_string()))
                {
                    if response_content_hash.len() == 1 {
                        let response_schema_hash =
                            &response_content_hash.front().unwrap().1["schema"];
                        if response_schema_hash
                            .as_hash()
                            .unwrap()
                            .get(&Yaml::String("oneOf".to_string()))
                            .is_some()
                        {
                            parse_oneof_type(
                                &str_to_camel_case(&format!("{operation_name}_response")),
                                response_schema_hash,
                                client_output_file,
                            );
                        } else if response_schema_hash
                            .as_hash()
                            .unwrap()
                            .get(&Yaml::String("type".to_string()))
                            == Some(&Yaml::String("object".to_string()))
                        {
                            parse_object_type(
                                &str_to_camel_case(&format!("{operation_name}_response")),
                                response_schema_hash,
                                client_output_file,
                            );
                        }
                    } else {
                        writeln!(
                            client_output_file,
                            "#[derive(Debug, PartialEq, Serialize, Deserialize)]"
                        )
                        .unwrap();

                        writeln!(
                            client_output_file,
                            "pub enum {} {{",
                            str_to_camel_case(&format!("{operation_name}_response"))
                        )
                        .unwrap();
                        for (response_name, response_hash) in response_content_hash {
                            let response_variant_hash = response_hash["schema"].as_hash().unwrap();

                            let response_variant_type = if let Some(Yaml::String(response_type)) =
                                response_variant_hash.get(&Yaml::String("type".to_string()))
                            {
                                if response_type == "string" {
                                    let response_string_format = response_variant_hash
                                        .get(&Yaml::String("format".to_string()))
                                        .unwrap()
                                        .as_str()
                                        .unwrap();
                                    if response_string_format == "binary" {
                                        "Vec<u8>"
                                    } else {
                                        unimplemented!()
                                    }
                                } else {
                                    unimplemented!()
                                }
                            } else if let Some(Yaml::String(response_ref)) =
                                response_variant_hash.get(&Yaml::String("$ref".to_string()))
                            {
                                &format!(
                                    "crate::types::{}",
                                    get_object_name_from_reference(response_ref)
                                )
                            } else if let Some(Yaml::Array(response_oneof)) =
                                response_variant_hash.get(&Yaml::String("oneOf".to_string()))
                            {
                                // HACK: This is fixed to a size of 2 and only works for the specific
                                // case where it is currently used.
                                assert_eq!(response_oneof.len(), 2);
                                writeln!(
                                    client_output_file,
                                    "\t{}(crate::types::{}),",
                                    str_to_camel_case(response_name.as_str().unwrap()),
                                    get_object_name_from_reference(
                                        response_oneof[0]["$ref"].as_str().unwrap()
                                    )
                                )
                                .unwrap();
                                writeln!(
                                    client_output_file,
                                    "\t{}Verbose(crate::types::{}),",
                                    str_to_camel_case(response_name.as_str().unwrap()),
                                    get_object_name_from_reference(
                                        response_oneof[1]["$ref"].as_str().unwrap()
                                    )
                                )
                                .unwrap();
                                continue;
                            } else {
                                unimplemented!()
                            };
                            writeln!(
                                client_output_file,
                                "\t{}({}),",
                                str_to_camel_case(response_name.as_str().unwrap()),
                                response_variant_type,
                            )
                            .unwrap();
                        }
                        writeln!(client_output_file, "}}",).unwrap();
                    }
                }
            } else if let Some(created_response) =
                responses_hash.get(&Yaml::String("201".to_string()))
            {
                let response_content_hash = created_response["content"].as_hash().unwrap();
                if response_content_hash.len() == 1 {
                    let response_schema_hash = &response_content_hash.front().unwrap().1["schema"];
                    if response_schema_hash
                        .as_hash()
                        .unwrap()
                        .get(&Yaml::String("oneOf".to_string()))
                        .is_some()
                    {
                        parse_oneof_type(
                            &str_to_camel_case(&format!("{operation_name}_response")),
                            response_schema_hash,
                            client_output_file,
                        );
                    } else if response_schema_hash
                        .as_hash()
                        .unwrap()
                        .get(&Yaml::String("object".to_string()))
                        .is_some()
                    {
                        parse_object_type(
                            &str_to_camel_case(&format!("{operation_name}_response")),
                            response_schema_hash,
                            client_output_file,
                        );
                    }
                } else {
                    unimplemented!()
                }
            }
        }
    }

    // Now that all the additional types needed for the API are defined we can start
    // implementing the API functions. There is one function per endpoint+operation combination
    writeln!(client_output_file, "impl OpenAIClient {{").unwrap();

    for (path_name, path_hash) in schema_list {
        println!("{:?}", path_name);
        let path_operations = path_hash.as_hash().unwrap();
        for (path_operation_name, path_operation_hash) in path_operations {
            let operation_name =
                str_to_snake_case(path_operation_hash["operationId"].as_str().unwrap());
            if let Some(d) = path_operation_hash
                .as_hash()
                .unwrap()
                .get(&Yaml::String("summary".to_string()))
            {
                writeln!(
                    client_output_file,
                    "\t/** {} */",
                    d.as_str().unwrap().trim_end()
                )
                .unwrap();
            }
            write!(
                client_output_file,
                "\tpub async fn {operation_name}(&self, ",
            )
            .unwrap();

            if let Some(parameters_list) = path_operation_hash
                .as_hash()
                .unwrap()
                .get(&Yaml::String("parameters".to_string()))
            {
                for parameter in parameters_list.as_vec().unwrap() {
                    let parameter_hash = parameter.as_hash().unwrap();
                    let parameter_schema = parameter["schema"].as_hash().unwrap();
                    let mut parameter_type = if let Some(Yaml::String(schema_type)) =
                        parameter_schema.get(&Yaml::String("type".to_string()))
                    {
                        match schema_type.as_str() {
                            "string" => "&str".to_string(),
                            "integer" => "u64".to_string(),
                            "object" => str_to_camel_case(&format!("{operation_name}_query")),
                            "array" => {
                                let array_items = parameter_schema
                                    .get(&Yaml::String("items".to_string()))
                                    .unwrap();
                                if let Some(array_type) = array_items["type"].as_str() {
                                    match array_type {
                                        "string" => "&[String]".to_string(),
                                        _ => unimplemented!("{:?}", array_items),
                                    }
                                } else if let Some(array_ref) = array_items["$ref"].as_str() {
                                    format!(
                                        "&[crate::types::{}]",
                                        get_object_name_from_reference(array_ref)
                                    )
                                } else {
                                    unimplemented!()
                                }
                            }
                            "boolean" => "bool".to_string(),
                            _ => unimplemented!("{:?}", schema_type),
                        }
                    } else if let Some(Yaml::String(schema_ref)) =
                        parameter_schema.get(&Yaml::String("$ref".to_string()))
                    {
                        format!(
                            "&crate::types::{}",
                            get_object_name_from_reference(schema_ref.as_str())
                        )
                    } else {
                        unimplemented!("{:?}", parameter_schema)
                    };
                    let parameter_required = if let Some(Yaml::Boolean(parameter_required)) =
                        parameter_hash.get(&Yaml::String("required".to_string()))
                    {
                        *parameter_required
                    } else {
                        false
                    };

                    if !parameter_required {
                        parameter_type = format!("Option<{}>", parameter_type);
                    }

                    write!(
                        client_output_file,
                        "{}: {}, ",
                        parameter["name"].as_str().unwrap().replace("[]", ""),
                        parameter_type
                    )
                    .unwrap();
                }
            }

            if let Some(request_body_hash) = path_operation_hash
                .as_hash()
                .unwrap()
                .get(&Yaml::String("requestBody".to_string()))
            {
                let request_body_type = {
                    let request_body_content = request_body_hash["content"].as_hash().unwrap();
                    assert_eq!(request_body_content.len(), 1);
                    let body_content_schema = request_body_content.front().unwrap().1["schema"]
                        .as_hash()
                        .unwrap();
                    if let Some(Yaml::String(body_content_ref)) =
                        body_content_schema.get(&Yaml::String("$ref".to_string()))
                    {
                        format!(
                            "crate::types::{}",
                            get_object_name_from_reference(body_content_ref)
                        )
                    } else if body_content_schema.get(&Yaml::String("type".to_string()))
                        == Some(&Yaml::String("object".to_string()))
                    {
                        str_to_camel_case(&format!("{operation_name}_request_body"))
                    } else {
                        unimplemented!("{:?}", body_content_schema)
                    }
                };
                let request_body_is_required =
                    request_body_hash["required"].as_bool().unwrap_or(false);
                write!(client_output_file, "request_body: ").unwrap();
                if request_body_is_required {
                    write!(client_output_file, "{}, ", request_body_type).unwrap();
                } else {
                    write!(client_output_file, "Option<{}>, ", request_body_type).unwrap();
                }
            }

            let responses_hash = path_operation_hash["responses"].as_hash().unwrap();
            let result_type = if let Some(ok_response) =
                responses_hash.get(&Yaml::String("200".to_string()))
            {
                if let Some(Yaml::Hash(response_content_hash)) = ok_response
                    .as_hash()
                    .unwrap()
                    .get(&Yaml::String("content".to_string()))
                {
                    if response_content_hash.len() == 1 {
                        let response_schema_hash =
                            response_content_hash.front().unwrap().1["schema"]
                                .as_hash()
                                .unwrap();
                        if let Some(Yaml::String(response_ref)) =
                            response_schema_hash.get(&Yaml::String("$ref".to_string()))
                        {
                            format!(
                                "crate::types::{}",
                                get_object_name_from_reference(response_ref.as_str())
                            )
                        } else if response_schema_hash
                            .get(&Yaml::String("oneOf".to_string()))
                            .is_some()
                            || response_schema_hash.get(&Yaml::String("type".to_string()))
                                == Some(&Yaml::String("object".to_string()))
                        {
                            str_to_camel_case(&format!("{operation_name}_response"))
                        } else if response_schema_hash.get(&Yaml::String("type".to_string()))
                            == Some(&Yaml::String("string".to_string()))
                        {
                            "String".to_string()
                        } else {
                            unimplemented!("{:?}", response_schema_hash)
                        }
                    } else {
                        str_to_camel_case(&format!("{operation_name}_response"))
                    }
                } else {
                    "()".to_string()
                }
            } else if let Some(created_response) =
                responses_hash.get(&Yaml::String("201".to_string()))
            {
                let response_content_hash = created_response["content"].as_hash().unwrap();
                if response_content_hash.len() == 1 {
                    let response_schema_hash = response_content_hash.front().unwrap().1["schema"]
                        .as_hash()
                        .unwrap();
                    if let Some(Yaml::String(response_ref)) =
                        response_schema_hash.get(&Yaml::String("$ref".to_string()))
                    {
                        format!(
                            "crate::types::{}",
                            get_object_name_from_reference(response_ref.as_str())
                        )
                    } else if response_schema_hash
                        .get(&Yaml::String("oneOf".to_string()))
                        .is_some()
                        || response_schema_hash
                            .get(&Yaml::String("object".to_string()))
                            .is_some()
                    {
                        str_to_camel_case(&format!("{operation_name}_response"))
                    } else {
                        unimplemented!("{:?}", response_schema_hash)
                    }
                } else {
                    str_to_camel_case(&format!("{operation_name}_response"))
                }
            } else {
                unimplemented!("{:?}", responses_hash)
            };
            writeln!(client_output_file, ") -> ConversaResult<{result_type}> {{",).unwrap();

            writeln!(
                client_output_file,
                "\t\tlet address = format!(\"{{}}{}\", self.base_address);",
                path_name.as_str().unwrap()
            )
            .unwrap();
            writeln!(
                client_output_file,
                "\t\tlet mut request = self.client.{}(&address);",
                path_operation_name.as_str().unwrap(),
            )
            .unwrap();
            writeln!(
                client_output_file,
                "\t\trequest = request.bearer_auth(&self.api_key);",
            )
            .unwrap();

            // Add the query arguments to the request
            if let Some(Yaml::Array(parameters_list)) = path_operation_hash
                .as_hash()
                .unwrap()
                .get(&Yaml::String("parameters".to_string()))
            {
                for parameter in parameters_list {
                    if parameter["in"].as_str().unwrap() == "query" {
                        let parameter_required = parameter["required"].as_bool().unwrap_or(false);
                        if parameter_required {
                            writeln!(
                                client_output_file,
                                "\t\trequest = request.query(&{});",
                                parameter["name"].as_str().unwrap().replace("[]", "")
                            )
                            .unwrap();
                        } else {
                            writeln!(
                                client_output_file,
                                "\t\tif let Some(q) = {} {{\n\t\t\trequest = request.query(&q);\n\t\t}}",
                                parameter["name"].as_str().unwrap().replace("[]", "")
                            )
                            .unwrap();
                        }
                    }
                }
            }

            if let Some(request_body_hash) = path_operation_hash
                .as_hash()
                .unwrap()
                .get(&Yaml::String("requestBody".to_string()))
            {
                let request_body_is_required =
                    request_body_hash["required"].as_bool().unwrap_or(false);

                let request_body_content = request_body_hash["content"].as_hash().unwrap();
                debug_assert!(request_body_content.len() == 1);
                // TODO: It requires different handling depending on the type of request body (application/json or multipart/form-data)
                let request_body_content_type =
                    request_body_content.front().unwrap().0.as_str().unwrap();
                if request_body_content_type == "application/json" {
                    if request_body_is_required {
                        writeln!(
                            client_output_file,
                            "\t\trequest = request.body(serde_json::to_string(&request_body)?);",
                        )
                        .unwrap();
                    } else {
                        writeln!(
                        client_output_file,
                        "\t\tif let Some(b) = request_body {{\n\t\t\trequest = request.body(serde_json::to_string(&b)?);\n\t\t}}",
                    )
                    .unwrap();
                    }
                } else if request_body_content_type == "multipart/form-data" {
                    writeln!(client_output_file, "\t\ttodo!();").unwrap();
                } else {
                    unimplemented!("Request body type: {}", request_body_content_type);
                }
            }

            writeln!(
                client_output_file,
                "\t\tlet result = request.send().await?;",
            )
            .unwrap();

            writeln!(
                client_output_file,
                "\t\tlet status_code = result.status().as_u16();",
            )
            .unwrap();

            writeln!(
                client_output_file,
                "\t\tlet _content_type = result.headers()[reqwest::header::CONTENT_TYPE].to_str()?.to_string();"
            )
            .unwrap();

            writeln!(
                client_output_file,
                "\t\tlet response_bytes = result.bytes().await?;",
            )
            .unwrap();

            let responses_hash = path_operation_hash["responses"].as_hash().unwrap();

            // Handle the Error cases first so that we can have an early return
            writeln!(
                    client_output_file,
                    "\t\tif status_code == 400 {{\n\t\t\treturn Err(ConversaError::ErrorResponse(serde_json::from_slice(&response_bytes)?))\n\t\t}}"
                )
                .unwrap();

            writeln!(
                    client_output_file,
                    "\t\tif status_code == 404 {{\n\t\t\treturn Err(ConversaError::Error(serde_json::from_slice(&response_bytes)?))\n\t\t}}"
                )
                .unwrap();

            // After checking for error check that the status code is actually what we are expecting
            let (ok_response_code, ok_response) = responses_hash
                .iter()
                .find(|(x, _)| x.as_str().unwrap() == "200" || x.as_str().unwrap() == "201")
                .unwrap();

            writeln!(
                    client_output_file,
                    "\t\tif status_code != {} {{\n\t\t\treturn Err(ConversaError::UnexpectedStatusCode{{code: status_code, response: String::from_utf8(response_bytes.to_vec())?}})\n\t\t}}",
                    ok_response_code.as_str().unwrap()
                )
                .unwrap();

            if let Some(response_content_hash) = ok_response["content"].as_hash() {
                if response_content_hash.len() == 1 {
                    writeln!(
                        client_output_file,
                        "\t\tOk(serde_json::from_slice(&response_bytes)?)"
                    )
                    .unwrap();
                } else {
                    writeln!(client_output_file, "\t\tmatch _content_type.as_str() {{").unwrap();
                    for (response_name, _) in response_content_hash {
                        writeln!(
                            client_output_file,
                            "\t\t\t\"{}\" => Ok({}::{}(serde_json::from_slice(&response_bytes)?)),",
                            response_name.as_str().unwrap(),
                            str_to_camel_case(&format!("{operation_name}_response")),
                            str_to_camel_case(response_name.as_str().unwrap()),
                        )
                        .unwrap();
                    }
                    writeln!(
                        client_output_file,
                        "\t\t\t_ => Err(ConversaError::UnexpectedContentType(_content_type)),"
                    )
                    .unwrap();
                    writeln!(client_output_file, "\t\t}}").unwrap();
                }
            } else {
                writeln!(client_output_file, "\t\tOk(())").unwrap();
            }

            writeln!(client_output_file, "\t}}\n",).unwrap();
            println!("\t{:?}", path_operation_name);
        }
    }

    writeln!(client_output_file, "\n}}").unwrap();
}

fn main() {
    println!("cargo::rerun-if-changed=./openapi.documented.yml");
    println!("cargo::rerun-if-changed=src/lib.rs");

    let openai_yml_file = read_to_string(OPENAI_YML_FILE_PATH).unwrap();
    let openai_yml = YamlLoader::load_from_str(&openai_yml_file).unwrap();
    let mut output_file = File::create("src/types.rs").unwrap();

    let schema_list = openai_yml[0]["components"]["schemas"].as_hash().unwrap();

    writeln!(output_file, "use std::collections::HashMap;").unwrap();
    writeln!(output_file, "use serde::{{Deserialize, Serialize}};\n").unwrap();

    for (schema_name, schema_value) in schema_list {
        let name = schema_name.as_str().unwrap();
        parse_component_schema(name, schema_value, &mut output_file);
    }

    let mut client_output_file = File::create("src/client.rs").unwrap();
    let path_schema = &openai_yml[0]["paths"];
    parse_endpoint_path(path_schema, &mut client_output_file);
}
