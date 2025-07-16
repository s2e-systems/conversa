# conversa_openai_client

A native Rust client for the complete OpenAI REST API.

[![Crates.io](https://img.shields.io/crates/v/conversa_openai_client.svg)](https://crates.io/crates/conversa_openai_client)
[![Docs.rs](https://docs.rs/conversa_openai_client/badge.svg)](https://docs.rs/conversa_openai_client)
[![License](https://img.shields.io/crates/l/conversa_openai_client.svg)](./LICENSE)

---

## Features

- Full support for the **entire OpenAI API**, including:
  - ChatGPT (chat completions)
  - DALL·E (image generation and editing)
  - Whisper (speech-to-text)
  - Embeddings
  - Assistants API (threads, runs, tools, files)
  - Fine-tuning, models, moderation, and more
- **Up-to-date and complete**: Automatically generated from OpenAI's official OpenAPI YAML spec.
- Code generation is performed locally at **build time** via a build.rs script. No external tools required post-clone.
- Generated types and methods use idiomatic Rust naming conventions

---

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
conversa_openai_client = "0.1"
```

---

## License

Licensed under the Apache License, Version 2.0.

---

## Contributing

Bug reports, and feature requests are welcome and appreciated!
Feel free to open issues on GitHub.