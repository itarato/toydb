# ToyDB

## Use

Starting the server:

```
cargo run --bin server -- [-d DUMP_TQL_FILE] [-v] [-V] [--help]
```

Starting the client:

```
cargo run --bin client
```

## Toy Query Language (TQL)

Create table: `+ TABLENAME (FIELDNAME TYPE)+ (: (INDICES)+)`

Select query: `? (FIELD_NAME)+ > TABLENAME (: (FIELD_NAME OP VALUE)+)`

Insert query: `> TABLENAME (FIELD_NAME VALUE)+`

Describe database: `:db`

Example:

```
+ users id int name varchar 255 age int : id
+ booking id int user_id int book varchar 255

> users id 0 name Steve age 30
> users id 1 name John age 26
> users id 2 name Maya age 89

> booking id 0 user_id 1 book WarOfWorlds
> booking id 1 user_id 1 book Sparta

? name > users
```
