[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://stand-with-ukraine.pp.ua)

# Hide

Hide is a command-line tool that mask sensitive information in JSON file. Gave it a file, tell it the keys and their values will be changed to placeholder value.

**Why do I need it?** I often used ChatGPT to process JSON, but I don't want to share any sensitive information (and you shouldn't) - that's why I decided to write a simple tool that will hide such information. 

Also, I am learning Rust, and as we know, the best way to learn code is to code.  

## Overview

Hide is designed to assist developers, data analysts, and anyone working with JSON files containing potentially sensitive information. By targeting specific keys like "name", "surname", "phone_number", etc, Hide replaces their corresponding values with a "[value_type]" placeholder, for example we have the following example.json:
```json
{
    "name": "Leeeeroy",
    "age": 20,
    "is_cool": true
}
```
Then this command
```bash
hide -i example.json --add-keys name,age,is_cool
```

Will print the following: 
```json
{
    "name": "String",
    "age": "Number",
    "is_cool": "Bool"
}
```

### Features

- **Keyword Management**: Hide allows users to easily add or remove keywords through the command-line interface or by manually editing a configuration file.
- **Customizable Output**: By default, Hide prints the modified JSON to a stdout, but users can specify output file path.
- **Debugging and Error Handling**: Hide provides clear error messages and optional stack traces for debugging, ensuring a smooth user experience.

More to come. (Don't hesitate to suggest your idea through 'issues')

## Usage

Basic usage to hide sensitive information in a JSON file and print result to std output:

```bash
hide -i file.json
```

You can also specify keys which values you want to hide: 
```bash
hide -i file.json --add-keys [key1,key2,...]
```
Keys are stored in config file and are not unique for each app run, therefore you can add or remove keys without processing JSON:

```bash
hide --add-keys [key1,key2,...] --remove-keys [key1,key2,...]
```

Specify output file:
```bash
hide -i file.json -o output.json
```

For a complete list of options, use `hide --help`

## TODO: Installation

Provide instructions on how to install Hide, including any dependencies or platform-specific details.

## Contributing

Contributions to Hide are welcome! Whether it's bug reporting, feature requests, or code contributions.

## 3rd party libraries

[anyhow](https://docs.rs/anyhow/latest/anyhow/) - This library provides anyhow::Error, a trait object-based error type for easy idiomatic error handling in Rust applications.

[clap](https://docs.rs/clap/latest/clap/) - parsing arguments, generating --help

[confy](https://docs.rs/confy/latest/confy/) - storing and reading config file


[serde](https://serde.rs/) - serializing and deserializing json into rust structures and vice versa

[serde_json](https://docs.rs/serde_json/latest/serde_json/) - serde for json

[log](https://docs.rs/log/latest/log/), [simple_logger](https://docs.rs/simple_logger/latest/simple_logger/) - logging

[assert_cmd](https://docs.rs/assert_cmd/latest/assert_cmd/) - running cli command in tests 

[assert_fs](https://docs.rs/assert_fs/latest/assert_fs/) - Filesystem fixtures and assertions for testing

[predicates](https://docs.rs/predicates/latest/predicates/) - writing assertions that assert_cmd can test against (and that have great error messages)

## License

Hide is licensed under the [MIT License](./LICENSE.MD).
