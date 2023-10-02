#include <fstream>
#include <cstring>

#include "huffman.h"

long unsigned int get_file_size(std::ifstream *input) {
  input->seekg(0, std::ios::end);
  long unsigned int file_size = input->tellg();
  return file_size;
}

uint32_t get_file_id(std::ifstream *input) {
  input->seekg(0, std::ios::beg);
  char file_id[4];
  input->read(file_id, 4);
  uint32_t file_id_int = file_id[3] << 24 |
                         file_id[2] << 16 |
                         file_id[1] << 8  |
                         file_id[0];
  return file_id_int;
}

uint16_t get_protocol_version(std::ifstream *input) {
  input->seekg(4, std::ios::beg);
  char protocol_version[2];
  input->read(protocol_version, 2);
  uint16_t protocol_version_int = protocol_version[1] << 8 |
                                  protocol_version[0];
  return protocol_version_int;
}

int write_header(std::ifstream *input, FILE *output, long unsigned int &bytes_seeked) {
  uint32_t file_id = get_file_id(input);
  fwrite(&file_id, sizeof(file_id), 1, output);
  uint16_t protocol_version = get_protocol_version(input);
  fwrite(&protocol_version, sizeof(protocol_version), 1, output);
  bytes_seeked = 6;
  return 0;
}

int write_decompressed_data(std::ifstream *input, FILE *output, long unsigned int &bytes_seeked, long unsigned int file_size) {
  std::vector<uint8_t> message;
  input->seekg(bytes_seeked, std::ios::beg);
  while (bytes_seeked < file_size) {
    unsigned char initial_message_size_byte;
    long unsigned int message_size;
    unsigned char remaining_bytes[5];
    remaining_bytes[4] = '\0';
    input->read(reinterpret_cast<char *>(&initial_message_size_byte), 1);
    bytes_seeked += 1;

    if (initial_message_size_byte < 254) {
      message_size = initial_message_size_byte;
    } else if (initial_message_size_byte == 254) {
      input->read(reinterpret_cast<char *>(remaining_bytes), 2);
      bytes_seeked += 2;
      message_size = remaining_bytes[1] << 8 |
                     remaining_bytes[0];
    } else {
      input->read(reinterpret_cast<char *>(remaining_bytes), 4);
      bytes_seeked += 4;
      message_size = remaining_bytes[3] << 24 |
                     remaining_bytes[2] << 16 |
                     remaining_bytes[1] << 8 |
                     remaining_bytes[0];
    }
    unsigned char byte;
    if (message_size > 0) {
      for (unsigned long int i=0; i<message_size; i++) {
        input->read(reinterpret_cast<char *>(&byte), 1);
        message.push_back(byte);
      }
      bytes_seeked += message_size;
      Huffman huffman;
      std::vector<uint8_t> result = huffman.decompress(message);

      auto len32 = static_cast<uint32_t>(result.size());
      {
        uint8_t len8;
        if (len32 < 254) {
          len8 = (uint8_t)len32;
        } else if (len32 < 65535) {
          len8 = 254;
        } else {
          len8 = 255;
        }
        fwrite(&len8, 1, 1, output);

      }
      if (len32 >= 254) {
        if (len32 <= 65535) {
          uint16_t len16 = (uint16_t)len32;
          char bytes[sizeof len16];
          std::copy(static_cast<const char*>(static_cast<const void*>(&len16)),
                    static_cast<const char*>(static_cast<const void*>(&len16)) + sizeof len16,
                    bytes);
          fwrite(&len16, 2, 1, output);
        } else {
          char bytes[sizeof len32];
          std::copy(static_cast<const char*>(static_cast<const void*>(&len32)),
                    static_cast<const char*>(static_cast<const void*>(&len32)) + sizeof len32,
                    bytes);
          fwrite(&len32, 4, 1, output);
        }
      }
      fwrite(&(result[0]), result.size(), 1, output);
      message.clear();
    }
  }

  return 0;
}

extern "C" int decompress_replay_file(char *input_path, char *output_path) {
  std::ifstream input(input_path, std::ios::binary);
  FILE *output = fopen(output_path, "wb");
  long unsigned int bytes_seeked = 0;

  write_header(&input, output, bytes_seeked);
  long unsigned int file_size = get_file_size(&input);
  write_decompressed_data(&input, output, bytes_seeked, file_size);

  input.close();
  fclose(output);

  return 0;
}

int main(int argc, char *argv[]) {
  if (argc < 3) {
    std::cout << "Usage: " << argv[0] << " <input_replay_file> <output_replay_file>" << std::endl;
    return 1;
  }

  decompress_replay_file(argv[1], argv[2]);
  return 0;
}
