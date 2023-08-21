[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua)

# Hide

Hide is a command-line tool that safeguards sensitive information in JSON files by identifying and replacing specific keys, such as personal details, with a customizable placeholder.

## Overview

Hide is designed to assist developers, data analysts, and anyone working with JSON files containing potentially sensitive information. By targeting specific keys like "name", "surname", "phone_number", and "email", Hide replaces their corresponding values with a "[hidden]" placeholder, or other customizable options.

### WIP: Features

- **Keyword Management**: Hide comes with predefined keywords and allows users to easily add or remove keywords through the command-line interface or by manually editing a configuration file.
- **Customizable Output**: By default, Hide saves the modified JSON to a new file, but users can specify custom output paths or even overwrite the original file.
- **Future-Proof Design**: While the current version uses "[hidden]" as the placeholder, future updates may include encryption, different placeholders, or other mechanisms for hiding sensitive data.
- **Debugging and Error Handling**: Hide provides clear error messages and optional stack traces for debugging, ensuring a smooth user experience.

## WIP: Usage

Basic usage to hide sensitive information in a JSON file:

```bash
hide file.json
```

Add or remove keywords:

```bash
hide --add-words [word1, word2, ...]
```

For a complete list of options and detailed instructions, please refer to the [User Guide](link-to-user-guide).

## TODO: Installation

Provide instructions on how to install Hide, including any dependencies or platform-specific details.

## TODO: Contributing

Contributions to Hide are welcome! Whether it's bug reporting, feature requests, or code contributions, please see our [Contributing Guide](link-to-contributing-guide) for details.

## TODO: 3rd party libraries

TODO:

## License

Hide is licensed under the [MIT License](./LICENSE.MD).
