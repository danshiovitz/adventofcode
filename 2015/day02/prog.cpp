#include <fstream>
#include <iostream>
#include <sstream>
#include <algorithm>

int main(int argc, char* argv[]) {
  std::ifstream infile(argv[1]);
  std::string line;

  int paper = 0;
  int ribbon = 0;
  while (std::getline(infile, line)) {
    std::istringstream iss(line);

    int vals[3];
    char s1, s2;
    if (!(iss >> vals[0] >> s1 >> vals[1] >> s2 >> vals[2]) ||
        s1 != 'x' || s2 != 'x') {
      std::cout << "Bogus line:" << line << std::endl;
      return 1;
    }

    std::sort(vals, vals+3);

    paper += 3*vals[0]*vals[1] + 2*vals[0]*vals[2] + 2*vals[1]*vals[2];
    ribbon += 2*vals[0] + 2*vals[1] + vals[0]*vals[1]*vals[2];
  }

  std::cout << "Total feet of paper: " << paper << std::endl;
  std::cout << "Total feet of ribbon: " << ribbon << std::endl;

  return 0;
}
