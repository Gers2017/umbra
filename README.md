# Umbra

Cli powered by [zenode](https://github.com/Gers2017/zenode) to send p2panda operations to a node

## Quick start

```
USAGE:
    umbra [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    create-instance    Creates a schema instance, requires schema_id and fields
    create-schema      Creates a schema, requires name, description and fields
    delete-instance    Deletes a schema instance, requires schema_id, last view_id
    help               Print this message or the help of the given subcommand(s)
    update-instance    Updates a schema instance, requires schema_id, view_id, and fields to update
```

## Clone and run [aquadoggo](https://github.com/p2panda/aquadoggo)

```sh
git clone https://github.com/p2panda/aquadoggo.git
```

and run aquadoggo

```sh
RUST_LOG=aquadoggo=info cargo run
```

### Create a schema

```
umbra create-schema -n <schema-name> -d <schema-description> -f <field-name>:<field-type>
```

For example:

```
umbra create-schema -n bears -d 'cute bears' -f id:int -f name:str -f hidden:bool
```

### Create an instance

```
umbra create-instance -s <schema_id> -f id:1 -f name:Bob -f hidden:false
```

### Update an instance

```
umbra update-instance -s <schema_id> -v <view_id> -f id:2 -f name:Yogi
```

### Delete an instance

```
umbra delete-instance -s <schema_id> -v <view_id>
```
