# Polestar CLI tool

## About The Project

Polestar CLI tool is a command line tool for Polestar. If you want to try Polestar but don't like to install Polestar App, you can try it.

## Getting Started

You can install Polestar CLI tool by cargo.

```sh
cargo install polestar-cli
```

## Usage

> Notice `open_ai.rs` file has request Token, you can replace your own token.

Polestar-cli is a interactive command line tool. You can run it by:

```sh
polestar-cli
```

### `help` command

And you can input `help` get all commands.

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

You can use `channel all` command to get all channels. It will show channels list with channel id and channel name.

```txt
polestar-cli〉channel all
71ee9810-e6bb-4bba-bacd-e00aabf981c9: quick launcher
```

#### `channel add <name> [desc]`

You can use `channel add` command to add a new channel. Channel name must be exist, and channel description is optional.

```txt
polestar-cli〉channel add "name"
```

```txt
polestar-cli〉channel add "name" "description"
```

#### `channel remove <id>`

You can use `channel remove` command to remove a channel by channel id.

Channel id is uuid string, when you remove a channel, you can select a channel from channel list.

```txt
polestar-cli〉channel remove
? Operate ›
> 71ee9810-e6bb-4bba-bacd-e00aabf981c9: quick launcher
  ab89fc4d-f18b-4737-81cb-0b0733c66df5: test1
[↑↓ to move, enter to select, type to filter, Esc to quit]
```

#### `channel switch <id>`

You can use `channel switch` command to switch current channel by channel id.

Channel id is uuid string, when you switch a channel, you can select a channel from channel list.

```txt
polestar-cli〉channel switch
? Operate ›
> 71ee9810-e6bb-4bba-bacd-e00aabf981c9: quick launcher
  ab89fc4d-f18b-4737-81cb-0b0733c66df5: test1
[↑↓ to move, enter to select, type to filter, Esc to quit]
```

#### `channel current`

You can get current channel info by `channel current` command.

```txt
polestar-cli〉channel current
71ee9810-e6bb-4bba-bacd-e00aabf981c9: quick launcher
```

### `msg` command

#### `msg send <content>`

You can use `msg send` command to send a message to current channel.

```txt
polestar-cli〉msg send "hello"
Hello! How can I assist you today?
```

## Roadmap
