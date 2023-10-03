# Decompress Bombsquad Replay Files

[Ballistica](https://github.com/efroemling/ballistica) engine uses Huffman coding to compress the game's
.brp replay files. This tool decodes them back to game stream.

## Compiling

This library is automatically statically compiled through `cargo build`, but you can also separately compile
it with:

```bash
$ git clone https://github.com/ritiek/bs-replay-analyzer
$ cd ./bs-replay-analyzer/libdecompress
$ make
```

The resulting static and shared library targets will be present under `./lib` directory and the binary can be
found in `./bin/decompress`.

## Usage

```bash
Usage: ./bin/decompress <input_replay_file> <output_replay_file>
```
For example:
```
$ ./bin/decompress ~/.bombsquad/replays/__lastReplay.brp ~/.bombsquad/replays/decoded_last_replay.brp
```

Now you can launch the game and choose to replay `decoded_last_replay` from the Watch section and it should
play fine since the game can read both Huffman encoded replay files as well as raw replay files.

## License

[huffman.cpp](/src/huffman.cpp), [huffman.h](/src/huffman.h) are stripped off versions from the core game ([here](https://github.com/efroemling/ballistica/blob/50f91361f41c69cc4e87eeba32217ff9558ada3e/src/ballistica/base/support/huffman.cc), and [here](https://github.com/efroemling/ballistica/blob/master/src/ballistica/base/support/huffman.h) respectively), and are [originally licensed under MIT](https://github.com/efroemling/ballistica/blob/50f91361f41c69cc4e87eeba32217ff9558ada3e/LICENSE).

All other code in this repository is also licensed under MIT. See [LICENSE](/LICENSE) for more info.
