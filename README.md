# spit

Git clone, following along with _[Write yourself a git!](https://wyag.thb.lt/#org4a4112c)_.

## Install
```sh
cargo build && alias spit=$PWD/target/debug/spit
```

## Example usages
```sh
spit init testrepo
cd testrepo
echo "test content" > file1.txt
spit hash-object file1.txt
spit cat-file <file1 sha>
```
