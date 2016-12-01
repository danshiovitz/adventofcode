#include <stdio.h>
#include <string.h>

int main(int argc, char* argv[])
{
    char const* const fileName = argv[1]; /* should check that argc > 1 */
    FILE* file = fopen(fileName, "r"); /* should check the result */
    char line[20000];

    while (fgets(line, sizeof(line), file)) {
        /* note that fgets don't strip the terminating \n, checking its
           presence would allow to handle lines longer that sizeof(line) */
        int i = 0;
        int floor = 0;
        int basement = 0;
        for (i = 0; i < strlen(line); i++) {
            if (line[i] == '(') { floor++; }
            if (line[i] == ')') { floor--; }
            if (floor == -1 && basement == 0) {
                basement = i + 1;
            }
        } 
        printf("Final floor: %d\n", floor);
        printf("Basement: %d\n", basement);
    }
    /* may check feof here to make a difference between eof and io failure -- network
       timeout for instance */

    fclose(file);

    return 0;
}
