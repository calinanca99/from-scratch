# `text-count`

CLI tool similar to `wc`. `text-count` allows you to:

- read data from either `stdin` or a file
- write data to either `stdout` or a file

## Building

```
cargo build --release
```

## Flags

- `--file` or `-f` to specify the input path. If missing, then `text_count` will read from `stdin`
- `--output` or `-o` to specify the output path. If missing, then `text_count` will write to `stdout`

## Examples

### Read from a file and write the results to another file

```
./target/release/text_count -f <input_file> -o <output_file>
```

### Pipelining

```
./target/release/text_count -f <input_file> | cat
```

```
cat <input_file> | ./target/release/text_count
```

### Reading user input

```
./target/release/text_count
> Hi
> How are you?
> Press CTRL+D to escape
```
