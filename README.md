# Language Translator Bot

This repository hosts a Language Translator Discord bot written in Rust. It uses the Serenity library for interaction with Discord and DeepL API for translation. The bot detects the language of a message and translates it accordingly.

## Features

- Listens to every message in the channels it has access to.
- Detects the language of a message from a predefined list of languages.
- Translates messages to the desired language.
- Replies with the translated text.

## Prerequisites

- Rust 1.54.0 or later
- A Discord bot token
- A DeepL API token

## Installation

1. Clone the repository.

   ```
   git clone https://github.com/<YourUserName>/language-translator-bot.git
   ```

2. Navigate to the project directory.

   ```
   cd language-translator-bot
   ```

3. Compile and build the project.

   ```
   cargo build --release
   ```

## Configuration

You need to provide your Discord bot token and DeepL API token in the code:

1. Open `src/main.rs` with your favorite text editor.

2. Replace `"YOUR_DISCORD_BOT_TOKEN_HERE"` with your Discord bot token.

3. Replace `"YOUR_DEEPL_API_TOKEN_HERE"` with your DeepL API token.

To change the languages being translated, you need to adjust the dependencies and code:

1. Open `Cargo.toml` with your text editor.

2. Update the `lingua` dependency's features with the desired languages. For example, if you want the bot to translate from French to German, you would change the line to:

   ```
   lingua = { version = "1.4.0", default-features = false, features = ["french", "german"] }
   ```

3. Open `src/main.rs` again.

4. Update the `languages` vector and the if-else block inside the `message` function with the corresponding `Language` enum variants and `Lang` constants.

## Usage

To start the bot, run:

```
cargo run
```

Now the bot is ready to translate messages on your Discord server. The bot uses `~` as a prefix, though it doesn't need any commands to start translating messages. It automatically translates messages in all channels it has access to.
