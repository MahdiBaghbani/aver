You have to install SeaORM CLI for migration tasks.

```shell
cargo install sea-orm-cli
```

```shell
sea-orm-cli migrate -d ./crates/migration generate users 
```

```shell
sea-orm-cli migrate -d ./crates/migration -u postgres://aver:aver@127.0.0.1:5432/aver
```

```shell
sea-orm-cli generate entity --lib --expanded-format --with-serde "both" -o ./crates/entity/src -u postgres://aver:aver@127.0.0.1:5432/aver
```


