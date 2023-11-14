# Polestar CLI tool

## About The Project

The Polestar CLI tool is a command-line interface tool for Polestar. You can use the CLI tool if you want to try Polestar without installing the Polestar app.

## Getting Started

You can install the Polestar CLI tool by cargo:

```sh
cargo install polestar-cli
```

## Usage

> Notice the `open_ai.rs` file has a request Token, you can replace your own token.

You can run it by:

```sh
polestar-cli
```

### `help` command

And you can input `help` to get all commands:

```txt
Welcome to Polestar!
polestar-cli〉help
An AI Q&A chat util written using Rust

Usage: app [COMMAND]

Commands:
  channel
  msg
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### `channel` command

#### `channel all`

You can use the `channel all` command to get all channels. It will show a channel list with channel ID and channel name.

```txt
polestar-cli〉channel all
71ee9810-e6bb-4bba-bacd-e00aabf981c9: quick launcher
```

#### `channel add <name> [desc]`

You can use the `channel add` command to add a new channel. Channel name must exist, and channel description is optional.

```txt
polestar-cli〉channel add "name"
```

```txt
polestar-cli〉channel add "name" "description"
```

#### `channel remove <id>`

You can use the `channel remove` command to remove a channel by channel ID.

When removing a channel, select a channel from the channel list. The channel ID is a unique UUID string that ensures accuracy and reliability.

```txt
polestar-cli〉channel remove
? Operate ›
> 71ee9810-e6bb-4bba-bacd-e00aabf981c9: quick launcher
  ab89fc4d-f18b-4737-81cb-0b0733c66df5: test1
[↑↓ to move, enter to select, type to filter, Esc to quit]
```

#### `channel switch <id>`

You can use the `channel switch` command to switch the current channel by channel ID.

The channel ID is represented by a UUID string. You can switch channels by selecting one from the channel list.

```txt
polestar-cli〉channel switch
? Operate ›
> 71ee9810-e6bb-4bba-bacd-e00aabf981c9: quick launcher
  ab89fc4d-f18b-4737-81cb-0b0733c66df5: test1
[↑↓ to move, enter to select, type to filter, Esc to quit]
```

#### `channel current`

You can get current channel info by the `channel current` command.

```txt
polestar-cli〉channel current
71ee9810-e6bb-4bba-bacd-e00aabf981c9: quick launcher
```

### `msg` command

#### `msg send <content>`

You can use the `msg send` command to send a message to the current channel.

```txt
polestar-cli〉msg send "hello"
Hello! How can I assist you today?
```

## Roadmap
