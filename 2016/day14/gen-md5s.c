#include <stdio.h>
#include <stdlib.h>
#include "md5.h"

int main(int argc, char* argv[]) {
  int num_to_gen, iterations;
  char buffer[256];
  unsigned char result[16];
  char* hex = "0123456789abcdef";
  MD5_CTX md5_ctx;
  
  if (argc < 4) {
    printf("Expected <num to generate> <num iterations> <startword>\n");
    exit(1);
  }

  num_to_gen = atoi(argv[1]);
  iterations = atoi(argv[2]);

  for (int i = 0; i < num_to_gen; i++) {
    int len = sprintf(buffer, "%s%d", argv[3], i);
    //printf("Initial: %s\n", buffer);

    for (int j = 0; j < iterations; j++) {
      MD5_Init(&md5_ctx);
      MD5_Update(&md5_ctx, buffer, len);
      MD5_Final(result, &md5_ctx);
      for (int k = 0; k < 16; k++) {
        buffer[k*2] = hex[(result[k]>>4) & 0xF];
        buffer[k*2+1] = hex[result[k] & 0xF];
      }
      if (j == 0) {
        buffer[32] = '\0';
        len = 32;
      }
      //printf("After step %d: %s\n", j, buffer);
    }

    printf("%s\n", buffer);
  }
}
