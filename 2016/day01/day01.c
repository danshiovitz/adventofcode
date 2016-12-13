#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_HISTORY 10000
int x_history[MAX_HISTORY];
int y_history[MAX_HISTORY];        

void add_history(int x, int y, int history_size) {
  for (int i = 0; i < history_size; i++) {
      if (x_history[i] == x && y_history[i] == y) {
          printf("Repeat x: %d, y: %d, total: %d\n",
               x, y, (abs(x) + abs(y)));
      }
  }
  
  if (history_size >= MAX_HISTORY) {
      printf("Oops, too much history, the universe is full\n");
  } else {
      x_history[history_size] = x;
      y_history[history_size] = y;
  }
}

int main(int argc, char* argv[])
{
    char const* const fileName = argv[1]; /* should check that argc > 1 */
    FILE* file = fopen(fileName, "r"); /* should check the result */
    char line[20000];
    
    while (fgets(line, sizeof(line), file)) {
        /* note that fgets don't strip the terminating \n, checking its
           presence would allow to handle lines longer that sizeof(line) */
        int i = 0;
        int len = strlen(line);
        char facing = 0;
        int x = 0;
        int y = 0;
        int history_size = 0;

        x_history[history_size] = 0;
        y_history[history_size] = 0;
        history_size++;
        
        for (i = 0; i < len; i++) {
            if (line[i] == 'L') {
                facing = (facing + 3) % 4;
            } else if (line[i] == 'R') {
                facing = (facing + 1) % 4;
            } else if (line[i] >= '0' && line[i] <= '9') {
                int amount = line[i] - '0';
                i++;
                while (i < len && line[i] >= '0' && line[i] <= '9') {
                    amount *= 10;
                    amount += line[i] - '0';                    
                    i++;
                }
                i--;

                for (int j = 0; j < amount; j++) {                
                    if (facing == 0) {
                        y++;
                    } else if (facing == 1) {
                        x++;
                    } else if (facing == 2) {
                        y--;
                    } else if (facing == 3) {
                        x--;
                    } else {
                        printf("Bad facing value: %d\n", facing);
                    }
                    add_history(x, y, history_size);
                    history_size++;
                }
            }
        }
        
        printf("Final x: %d, y: %d, total: %d\n", x, y, (abs(x) + abs(y)));
    }
    /* may check feof here to make a difference between eof and io failure -- network
       timeout for instance */

    fclose(file);

    return 0;
}
