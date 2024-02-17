# GPT Commander

GPT Commander is a Rust application that leverages the OpenAI GPT-3.5 model to provide chat completion functionality. It allows users to interact with the GPT-3.5 model through a simple GUI interface where they can input text prompts and receive AI-generated responses.

## Features
- Utilizes the OpenAI GPT-3.5 model for chat completion
- Provides a minimalistic GUI interface for user interaction
- Supports copying the generated response to the clipboard
- Ability to customize the viewport size and position

## How to Use
1. Ensure you have Rust installed on your system.
2. Set the `OPENAI_API_KEY` environment variable with your OpenAI API key.
3. Run the application with optional arguments for window position:
   ```bash
   cargo run --release -- <x_position> <y_position>
   ```
4. Input your prompts in the text area and click "A-OK-dokie" to generate a response.
5. You can copy the response to the clipboard by clicking the button.

## Dependencies
- `clipboard_rs`: For interacting with the system clipboard
- `eframe`: A framework for creating GUI applications in Rust
- `openai_api_rs`: Rust client for the OpenAI API

## Additional Notes
- The application hides the console window on Windows in release mode.
- The chat completion request is sent asynchronously to prevent blocking the UI thread.
- The chat completion messages are predefined in the `send_request` function.