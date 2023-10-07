#include <fstream>
#include <cstring>

#include "huffman.h"

struct ReplayHeader {
  uint32_t file_id;
  uint16_t protocol_version;
};

class Replay {
public:
  Replay(char *input_path);
  ~Replay();
  ReplayHeader get_header();
  long unsigned int get_file_size();
  int decompress_to(char *output_path);

private:
  std::ifstream *input;
  long unsigned int bytes_seeked = 0;
  int write_header(FILE *output);
  int write_decompressed_data(FILE *output);
};

extern "C" int decompress_replay_file(char *input_path, char *output_path);
