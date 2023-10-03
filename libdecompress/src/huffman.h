// The MIT License (MIT)
// Copyright (c) 2023 Eric Froemling
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

#include <vector>
#include <cstdint>
#include <cassert>
#include <iostream>
#include <exception>

// #include "ballistica/core/object.h"

class Huffman {
public:
  Huffman();
  ~Huffman();

  void train(const char* buffer, int len);

  void build();

  // NOTE: this assumes the topmost bit of the first byte is unused
  // (see details in implementation).
  auto compress(const std::vector<uint8_t>& src) -> std::vector<uint8_t>;
  auto decompress(const std::vector<uint8_t>& src) -> std::vector<uint8_t>;
  auto get_built() const -> bool {
    return built;
  }

private:
  bool built;
  uint32_t test_bytes = 0;
  uint32_t test_bits_compressed = 0;
  int total_count = 0;
  int total_length = 0;

  class Node {
  public:
    Node() = default;

    // Left child index in node array (-1 for none).
    int16_t left_child = -1;

    // Right child index in node array (-1 for none).
    int16_t right_child = -1;

    // Parent index in node array (0 for none - add 255 to this to get actual
    // index).
    uint8_t parent = 0;
    uint8_t bits = 0;
    uint16_t val = 0;
    int frequency = 0;
  };

  Node nodes_[511];
};
