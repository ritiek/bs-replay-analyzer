#include <fstream>
#include <cstring>

#include "huffman.h"

// First 20 bytes: [37, 71, 1, 0, 33, 0 || 1, 0, 254, 80, 52, 133, 145, 239, 255, 225, 253, 205, 8, 254]

void print(std::vector <uint8_t> const &a) {
    for(int i=0; i<a.size(); i++) {
       // std::cout << a.at(i);
       std::cout << (int)a.at(i) << " ";
    }
    std::cout << std::endl;
}

int main() {
    std::ifstream input("/home/ritiek/.ballisticacore/replays/somereplay.brp", std::ios::binary);
    FILE * output = fopen("out.brp", "wb");
    // std::ifstream input("/home/ritiek/.ballisticacore/replays/copylast.brp", std::ios::binary);
    char header[7];
    input.read(header, 6);
    header[6] = '\0';
    int bytes_seeked = 6;

    // std::vector<uint8_t> buffer(
    //  (std::istreambuf_iterator<char>(input)),
    //  (std::istreambuf_iterator<char>()));

    // uint8_t file_id[4];
    // for (int i=0; i<4; i++) {
    //     file_id[i] = buffer[i];
    //     std::cout << std::dec << buffer[i] << std::endl;
    // }
    // int value;
    // std::memcpy(&value, file_id, sizeof(int));
    // std::cout << value << std::endl;

    // for (int i=0; i<4; i++) std::cout << buffer[i];

    uint32_t file_id = header[3] << 24 |
                       header[2] << 16 |
                       header[1] << 8  |
                       header[0];

    uint16_t protocol_version = header[5] << 8 |
                                header[4];

    // BS ignores the 6th byte (last byte of protocol version) for some reason
    // I could be wrong.

    // for (int i=0; i<6; i++) {
    //     std::cout << header[i];
    // }
    fwrite(&file_id, sizeof(file_id), 1, output);
    fwrite(&protocol_version, sizeof(protocol_version), 1, output);

    // std::cout << file_id << std::endl;
    // std::cout << protocol_version << std::endl;
    // std::cout << std::endl;

    // for (int i=0; i<buffer.size(); i++) {
    //     std::cout << buffer[i];
    // }

    std::vector<uint8_t> message;

    input.seekg(0, std::ios::end);
    int file_size = input.tellg();

    input.seekg(bytes_seeked, std::ios::beg);

    // std::cout << file_size << std::endl;

    // char a = 252;
    // std::cout << "hi!: " << static_cast<unsigned>(a) << std::endl;


    // std::cout << "F: " << file_size << std::endl;
    while (bytes_seeked < file_size) {
        unsigned char initial_message_size_byte;
        long unsigned int message_size;
        // unsigned char remaining_bytes[3];
        // remaining_bytes[3] = '\0';
        unsigned char remaining_bytes[5];
        remaining_bytes[4] = '\0';
        // std::cout << bytes_seeked << std::endl;
        input.read(reinterpret_cast<char *>(&initial_message_size_byte), 1);
        bytes_seeked += 1;

        // std::cout << initial_message_size_byte;

        // std::cout << static_cast<unsigned>(initial_message_size_byte) << std::endl;
        if (initial_message_size_byte < 254) {
            // std::cout << "< 254" << std::endl;
            message_size = initial_message_size_byte;
        } else if (initial_message_size_byte == 254) {
            // std::cout << "== 254" << std::endl;
            // input.read(reinterpret_cast<char *>(remaining_bytes), 1);
            // message_size = remaining_bytes[0] << 8 |
            //                initial_message_size_byte;
            // bytes_seeked += 1;
            input.read(reinterpret_cast<char *>(remaining_bytes), 2);
            bytes_seeked += 2;
            message_size = remaining_bytes[1] << 8 |
                           remaining_bytes[0];
            // std::cout << remaining_bytes[0] << remaining_bytes[1];
        } else {
            // std::cout << "== 255" << std::endl;
            // input.read(reinterpret_cast<char *>(remaining_bytes), 3);
            // message_size = remaining_bytes[2] << 24 |
            //                remaining_bytes[1] << 16 |
            //                remaining_bytes[0] << 8 |
            //                initial_message_size_byte;
            // bytes_seeked += 3;
            input.read(reinterpret_cast<char *>(remaining_bytes), 4);
            bytes_seeked += 4;
            message_size = remaining_bytes[3] << 24 |
                           remaining_bytes[2] << 16 |
                           remaining_bytes[1] << 8 |
                           remaining_bytes[0];
            // std::cout << remaining_bytes[0] << remaining_bytes[1] << remaining_bytes[2] << remaining_bytes[3];
        }
        // std::cout << message_size << std::endl;
        unsigned char byte;
        if (message_size > 0) {
            for (int i=0; i<message_size; i++) {
                input.read(reinterpret_cast<char *>(&byte), 1);
                // if (i == 0) {
                //     std::cout << (int)byte << std::endl;
                // }
                // std::cout << (int)byte << std::endl;
                // std::cout << (int)byte << " ";
                message.push_back(byte);
                // if (i > 0) {
                //     message.push_back(byte);
                // }
            }
            // std::cout << std::endl << std::endl;
            bytes_seeked += message_size;
            // std::cout << initial_message_size_byte;
            Huffman huffman;
            // std::cout << message.size() << ": ";
            // for (int i=0; i<message.size(); i++) {
            //     std::cout << (int)message[i] << " ";
            // }
            // std::cout << std::endl << std::endl;
            std::vector<uint8_t> result = huffman.decompress(message);

          auto len32 = static_cast<uint32_t>(result.size());
          // std::cout << len32 << ": ";
          {
            uint8_t len8;
            if (len32 < 254) {
              len8 = (uint8_t)len32;
            } else if (len32 < 65535) {
              len8 = 254;
            } else {
              len8 = 255;
            }
            // std::cout << len8;
            fwrite(&len8, 1, 1, output);

            // if (fwrite(&len8, 1, 1, replay_out_file_) != 1) {
            //   fclose(replay_out_file_);
            //   replay_out_file_ = nullptr;
            //   Log("error writing replay file: " + g_platform->GetErrnoString());
            //   return;
            // }
          }
          // write 16 bit val if need be..
          if (len32 >= 254) {
            if (len32 <= 65535) {
              uint16_t len16 = (uint16_t)len32;
              char bytes[sizeof len16];
              std::copy(static_cast<const char*>(static_cast<const void*>(&len16)),
                        static_cast<const char*>(static_cast<const void*>(&len16)) + sizeof len16,
                        bytes);
              fwrite(&len16, 2, 1, output);
              // std::cout << bytes[0] << bytes[1];

              // if (fwrite(&len16, 2, 1, replay_out_file_) != 1) {
              //   fclose(replay_out_file_);
              //   replay_out_file_ = nullptr;
              //   Log("error writing replay file: " + g_platform->GetErrnoString());
              //   return;
              // }
            } else {
              char bytes[sizeof len32];
              std::copy(static_cast<const char*>(static_cast<const void*>(&len32)),
                        static_cast<const char*>(static_cast<const void*>(&len32)) + sizeof len32,
                        bytes);
              fwrite(&len32, 4, 1, output);
              // std::cout << bytes[0] << bytes[1] << bytes[2] << bytes[3];

              // if (fwrite(&len32, 4, 1, replay_out_file_) != 1) {
              //   fclose(replay_out_file_);
              //   replay_out_file_ = nullptr;
              //   Log("error writing replay file: " + g_platform->GetErrnoString());
              //   return;
              // }
            }
          }

            fwrite(&(result[0]), result.size(), 1, output);
            // print(result);
            message.clear();
        }
        // std::cout << "B: " << bytes_seeked << std::endl;
    }

    input.close();
    fclose(output);

    // let mut buf = [0; 1];
    // // replay.read(&mut buf)?;
    // let mut initial_byte_of_size = u8::from_ne_buffer(buf);
    // // println!("{:?}", buf);
    // println!("{}", initial_byte_of_size);
    // let mut size: u32;
    // if initial_byte_of_size < 254 {
    //     size = initial_byte_of_size as u32;
    // } else if initial_byte_of_size == 254 {
    //     let mut buf = [0; 2];
    //     buf[0] = 254;
    //     replay.read(&mut buf)?;
    //     size = u16::from_ne_buffer(buf) as u32;
    // } else {
    //     let mut buf = [0; 4];
    //     buf[0] = 255;
    //     replay.read(&mut buf)?;
    //     size = u32::from_ne_buffer(buf);
    // }

    return 0;
}
