/* keyboard.h */
#ifndef keyboard
#define keyboard()


typedef struct {
        int x;
        int y;
        int rows;
        int cols;
} Character;

void check_cursor();
void move_cursor(char);

#endif
