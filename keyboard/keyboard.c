#include "keyboard.h"
#include "modes/modes.h"
#include <ncurses.h>

// use "extern" to link with other files
extern Character curser;
static Mode current_mode = NORMAL_MODE; // default its in normal mode

void check_cursor() {
        // i've made changes to file structure and function, currently i'm
        // working on the x,y rows and cols with pointer -> char conversion .
        // Character curser;
        if (curser.y == curser.cols - 1) {
                curser.y = 3;
                curser.x++;
                move(curser.x, curser.y);
        } else if (curser.y <= 3) {
                curser.y = 3;
                move(curser.x, curser.y);
        }
        if (curser.x == 0) {
                curser.x = 1;
                move(curser.x, curser.y);
        }
        if (curser.x >= curser.rows - 2) {
                curser.x = curser.rows - 3;
                move(curser.x, curser.y);
        }
}

void move_cursor(char c) {
        static int last_key = 0;
        switch (c) {
        case 'a':
                move(curser.x, --curser.y);
                break;
        case 'w':
                move(--curser.x, curser.y);
                break;
        case 'd':
                move(curser.x, ++curser.y);
                break;
        case 's':
                move(++curser.x, curser.y);
                break;
        case 'x':
                delch();
                break;
        case '\n':
                if (curser.x < curser.rows - 1) {
                        ++curser.x;
                        ++curser.y;
                        curser.y = 0;
                        move(curser.x, curser.y);
                }

                break;

        case 'm':
                if (current_mode == COMMAND_MODE ||
                    current_mode == INSERT_MODE) {
                        switch_mode();
                }
                break;
        case ':':
                if (current_mode == NORMAL_MODE) {
                        switch_mode();
                }
                break;
        case 'z':
                if (last_key == 'z') {
                        move(curser.x, 0);
                        deleteln();
                        last_key = 0;
                        break;
                }
                last_key = 'z';
                return;
        default:
                last_key = 0;
                break;
        }
        check_cursor();
        refresh();
}
