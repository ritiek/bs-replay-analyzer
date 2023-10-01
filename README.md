# Decompress Bombsquad Replay Files

[Ballistica](https://github.com/efroemling/ballistica) engine uses Huffman coding to compress the game's
.brp replay files. This tool decodes them back to game stream.

## Compiling

```bash
$ git clone https://github.com/ritiek/bs-replay-analyzer
$ make
```

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

[huffman.cpp], [huffman.h] are stripped off versions from the core game ([here](https://github.com/efroemling/ballistica/blob/50f91361f41c69cc4e87eeba32217ff9558ada3e/src/ballistica/base/support/huffman.cc), and [here](https://github.com/efroemling/ballistica/blob/master/src/ballistica/base/support/huffman.h) respectively), and are [originally licensed under MIT](https://github.com/efroemling/ballistica/blob/50f91361f41c69cc4e87eeba32217ff9558ada3e/LICENSE).

All other code in this repository is also licensed under MIT.
