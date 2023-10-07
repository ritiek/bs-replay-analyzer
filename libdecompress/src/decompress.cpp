#include "decompress.h"

Replay::Replay(char *input_path) {
  // std::ifstream input(input_path, std::ios::binary);
  // this->input = &input;
  std::ifstream *input = new std::ifstream(input_path, std::ios::binary);
  this->input = input;
}

Replay::~Replay() {
  this->input->close();
}

ReplayHeader Replay::get_header() {
  this->input->seekg(0, std::ios::beg);
  char file_id[4];
  this->input->read(file_id, 4);
  uint32_t file_id_int = file_id[3] << 24 |
                         file_id[2] << 16 |
                         file_id[1] << 8  |
                         file_id[0];

  this->input->seekg(4, std::ios::beg);
  char protocol_version[2];
  this->input->read(protocol_version, 2);
  uint16_t protocol_version_int = protocol_version[1] << 8 |
                                  protocol_version[0];

  return ReplayHeader {
    file_id_int,
    protocol_version_int,
  };
}

long unsigned int Replay::get_file_size() {
  this->input->seekg(0, std::ios::end);
  long unsigned int file_size = this->input->tellg();
  this->input->seekg(this->bytes_seeked, std::ios::beg);
  return file_size;
}

int Replay::decompress_to(char *output_path) {
  FILE *output = fopen(output_path, "wb");
  this->write_header(output);
  this->write_decompressed_data(output);
  fclose(output);
  return 0;
}

int Replay::write_header(FILE *output) {
  ReplayHeader header = this->get_header();
  fwrite(&header.file_id, sizeof(header.file_id), 1, output);
  fwrite(&header.protocol_version, sizeof(header.protocol_version), 1, output);
  this->bytes_seeked = sizeof(header.file_id) + sizeof(header.protocol_version);
  return 0;
}

int Replay::write_decompressed_data(FILE *output) {
  std::vector<uint8_t> message;
  long unsigned int file_size = this->get_file_size();

  while (this->bytes_seeked < file_size) {
    unsigned char initial_message_size_byte;
    long unsigned int message_size;
    unsigned char remaining_bytes[5];
    remaining_bytes[4] = '\0';
    this->input->read(reinterpret_cast<char *>(&initial_message_size_byte), 1);
    this->bytes_seeked += 1;

    if (initial_message_size_byte < 254) {
      message_size = initial_message_size_byte;
    } else if (initial_message_size_byte == 254) {
      this->input->read(reinterpret_cast<char *>(remaining_bytes), 2);
      this->bytes_seeked += 2;
      message_size = remaining_bytes[1] << 8 |
                     remaining_bytes[0];
    } else {
      this->input->read(reinterpret_cast<char *>(remaining_bytes), 4);
      this->bytes_seeked += 4;
      message_size = remaining_bytes[3] << 24 |
                     remaining_bytes[2] << 16 |
                     remaining_bytes[1] << 8 |
                     remaining_bytes[0];
    }
    unsigned char byte;
    if (message_size > 0) {
      for (unsigned long int i=0; i<message_size; i++) {
        this->input->read(reinterpret_cast<char *>(&byte), 1);
        message.push_back(byte);
      }
      this->bytes_seeked += message_size;
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
  Replay replay = Replay(input_path);
  replay.decompress_to(output_path);
  return 0;
}
