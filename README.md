# bs-replay-analyzer

Does some magic. ✨

## Compiling

```bash
$ git clone https://github.com/ritiek/bs-replay-analyzer
$ cd ./bs-replay-analyzer
$ cargo build
```

## Usage

```bash
$ cargo run -- ~/.bombsquad/replays/__lastReplay.brp ~/.bombsquad/replays/test.brp
```

## License

[huffman.cpp](/libdecompress/src/huffman.cpp), [huffman.h](/libdecompress/src/huffman.h) are stripped off versions from the core game ([here](https://github.com/efroemling/ballistica/blob/50f91361f41c69cc4e87eeba32217ff9558ada3e/src/ballistica/base/support/huffman.cc), and [here](https://github.com/efroemling/ballistica/blob/master/src/ballistica/base/support/huffman.h) respectively), and are [originally licensed under MIT](https://github.com/efroemling/ballistica/blob/50f91361f41c69cc4e87eeba32217ff9558ada3e/LICENSE).

All other code in this repository is also licensed under MIT. See [LICENSE](/LICENSE) for more info.
