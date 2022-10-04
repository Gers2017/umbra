# Umbra

Cli powered by [zenode](https://github.com/Gers2017/zenode) to send operations to a p2panda node

## Quick start

```
Usage: umbra [COMMAND]

Commands:
  cs    Create schema, requires name, description and fields
  ci    Create instance, requires schema_id and fields
  ui    Update instance, requires schema_id, view_id, and fields to update
  di    Delete instance, requires schema_id, last view_id
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

## Clone and run [aquadoggo](https://github.com/p2panda/aquadoggo)

```sh
git clone https://github.com/p2panda/aquadoggo.git
```

and run aquadoggo

```sh
RUST_LOG=aquadoggo=info cargo run
```

## Examples

### Create a schema

```sh
umbra cs -n <schema-name> -d <schema-description> -f <field-name>:<field-type>

```

For example:

```sh
umbra cs -n bears -d 'cute bears' -f id:int -f name:str -f hidden:bool --log
```

> Pro tip: Use the --log or -l flag to print your operation with shiny colors 🤫

### Create an instance

```sh
umbra ci -s <schema_id> -f id:1 -f name:Bob -f hidden:false
```

### Update an instance

```sh
umbra ui -s <schema_id> -v <view_id> -f id:2 -f name:Yogi
```

### Delete an instance

```sh
umbra di -s <schema_id> -v <view_id>
```
