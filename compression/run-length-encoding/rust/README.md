# `Run-Length Encoding (RLE)`

Implements the [run-length encoding](https://en.wikipedia.org/wiki/Run-length_encoding) data compression algorithm.

> Run-length encoding (RLE) is a form of lossless data compression in which runs of data (sequences in which the same data value occurs in many consecutive data elements) are stored as a single data value and count, rather than as the original run.

## Example

> Consider a screen containing plain black text on a solid white background. There will be many long runs of white pixels in the blank space, and many short runs of black pixels within the text. A hypothetical scan line, with B representing a black pixel and W representing white, might read as follows:
>
> WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW
>
> With a run-length encoding (RLE) data compression algorithm applied to the above hypothetical scan line, it can be rendered as follows:
>
> 12W1B12W3B24W1B14W

## Running the tests

```
cargo run test
```

### Note about testing

There is only one test that checks that the following [property](https://en.wikipedia.org/wiki/Software_testing#Property_testing) of the algorithms holds: _compressing and decompressing data should result must leave the original data unaffected_. This property results from the fact that RLE is a [lossless](https://en.wikipedia.org/wiki/Lossless_compression) data compression algorithm.
