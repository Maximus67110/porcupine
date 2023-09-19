# Voice Assistant with Porcupine

This project is a simple voice assistant implemented in Rust using the Porcupine wake word detection engine. It listens for specific wake words and performs actions when a wake word is detected.

## Table of Contents

- [Description](#description)
- [Getting Started](#getting-started)
    - [Installation](#installation)
- [Usage](#usage)

## Description

This voice assistant is designed to recognize specific wake words and trigger actions when these words are detected in audio input. It uses the Porcupine engine to perform wake word detection.

## Getting Started

### Installation

To set up the voice assistant, follow these steps:

1. Clone this repository:

   ```bash
   git clone https://github.com/Maximus67110/porcupine
   cd porcupine
   ```

2. Install Rust if you haven't already:

   ```
   # Install Rust using rustup: https://rustup.rs/
   ```

3. Install the required dependencies by adding them to your `Cargo.toml` file and running:

   ```bash
   cargo build
   ```

## Usage

To use the voice assistant, follow these steps:

1. Set up your environment by creating a `.env` file and adding your access token:

   ```plaintext
   ACCESS_TOKEN=your_access_token_here
   ```

2. Run the program:

   ```bash
   cargo run
   ```

3. Follow the prompts to select the audio device input, language, and keywords for wake word detection.

4. The voice assistant will listen for the specified wake words and perform actions when they are detected.