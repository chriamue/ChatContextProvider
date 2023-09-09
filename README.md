# ChatContextProvider

`ChatContextProvider` is a powerful ChatGPT plugin that offers a deeper understanding of GitHub projects by analyzing their structure and, for Rust projects, providing insights into structs and traits.

## Features:

- **Web Frontend**: A user-friendly interface to input GitHub URLs.
- **Repository Cloning**: Clones the provided GitHub repository for analysis.
- **File Structure Overview**: Displays the actual structure of files in the repository.
- **Rust Code Analysis**: For Rust projects, extracts and displays information about structs and traits.

## Getting Started:

### Prerequisites:

- Rust and Cargo installed on your machine.
- A server or hosting solution for deployment.

### Installation:

1. Clone this repository:
   ```bash
   git clone https://github.com/chriamue/chat-context-provider.git
   ```

2. Navigate to the project directory:
   ```bash
   cd chat-context-provider
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

### Usage:

1. Run the server:
   ```bash
   cargo run
   ```

2. Open a web browser and navigate to the provided server address (e.g., `http://localhost:8000`).
3. Paste a GitHub URL into the input field and submit.
4. View the analyzed results, including file structure and, if applicable, Rust structs and traits information.


## License:

This project is licensed under the MIT License. See [LICENSE](LICENSE) for more information.
