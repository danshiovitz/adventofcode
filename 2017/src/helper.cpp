#include <cstdint>
#include <iostream>
#include <vector>

int day15(const std::vector<std::string>& args) {
  if (args.size() != 3) {
    std::cout << "Usage: <use filter> <A start> <B start>" << std::endl;
    exit(1);
  }

  bool use_filter = args[0] == "1";
  uint64_t a_cur = stoi(args[1]);
  uint64_t b_cur = stoi(args[2]);

  const uint32_t a_factor = 16807;
  const uint32_t b_factor = 48271;

  const uint32_t a_filter = 4;
  const uint32_t b_filter = 8;

  uint32_t mask = 0x0000ffff;
  uint32_t mod = 2147483647;

  uint32_t matches = 0;

  if (use_filter) {
    uint32_t times = 5000000;
    for (int i = 0; i < times; i++) {
      do {
        a_cur = (a_cur * a_factor) % mod;
      } while (a_cur % a_filter != 0);

      do {
        b_cur = (b_cur * b_factor) % mod;
      } while (b_cur % b_filter != 0);

      if ((a_cur & mask) == (b_cur & mask)) {
        matches++;
      }
    }
  } else {
    uint32_t times = 40000000;
    for (int i = 0; i < times; i++) {
      a_cur = (a_cur * a_factor) % mod;
      b_cur = (b_cur * b_factor) % mod;
      if ((a_cur & mask) == (b_cur & mask)) {
        matches++;
      }
    }
  }

  std::cout << "Matches: " << matches << std::endl;
  return 0;
}

int main(int argc, char** argv) {
  if (argc < 2) {
    std::cout << "Usage: helper <day> <day args...>" << std::endl;
    exit(1);
  }

  std::string type = argv[1];
  std::vector<std::string> args;
  for (int i = 2; i < argc; i++) {
    args.push_back(argv[i]);
  }

  if (type == "day15") {
      day15(args);
  } else {
    std::cout << "Unknown day: " << type << std::endl;
    exit(1);
  }
}
