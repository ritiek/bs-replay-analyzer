#include "decompress.h"

int main(int argc, char *argv[]) {
  if (argc < 3) {
    std::cerr << "Usage: ./" << argv[0] << " <input_replay_file> <output_replay_file>" << std::endl;
    return 1;
  }
  decompress_replay_file(argv[1], argv[2]);
  return 0;
}
