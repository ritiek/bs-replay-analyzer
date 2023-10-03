#include <fstream>
#include <cstring>

#include "huffman.h"

long unsigned int get_file_size(std::ifstream *input);
uint32_t get_file_id(std::ifstream *input);
uint16_t get_protocol_version(std::ifstream *input);
int write_header(std::ifstream *input, FILE *output, long unsigned int &bytes_seeked);
int write_decompressed_data(std::ifstream *input, FILE *output, long unsigned int &bytes_seeked, long unsigned int file_size);

extern "C" int decompress_replay_file(char *input_path, char *output_path);
