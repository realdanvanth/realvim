#include "file_handle.h"
#include "keyboard/keyboard.h"
#include <ncurses.h>

extern Character curser;

void open_file(char path[]) {
        move(curser.rows - 2, curser.cols - 10);
        attron(COLOR_PAIR(1));
        attron(A_BOLD);
        printw("%s", path);
        attroff(A_BOLD);
        attroff(COLOR_PAIR(1));
        refresh();
        move(curser.x, curser.y);
        FILE *fptr = fopen(path, "r");
        char data[64];
        if (!fptr) {
                printw("error");
                refresh();
                return;
        }
        while (fgets(data, sizeof(data), fptr)) {
                for (int i = 0; data[i] != '\0'; i++) {
                        if (data[i] != '\n' && data[i] != ERR) {
                                move(curser.x, curser.y++);
                                check_cursor();
                                printw("%c", data[i]);
                        } else if (data[i] != ERR) {
                                move(++curser.x, 3);
                                curser.y = 3;
                                check_cursor();
                        }
                }
        }
        refresh();
        fclose(fptr);
}

void save_file(const char *path) {
        FILE *fptr = fopen(path, "w"); // Open the file for writing
        if (!fptr) {
                move(curser.rows - 2, 0);
                printw("Error: Could not save file!");
                refresh();
                return;
        }

        // Iterate through the contents of the screen and write to the file
        for (int i = 1; i < curser.rows - 3; i++) { // Skip top and bottom rows
                char line[curser.cols];
                move(i, 3); // Start after line numbers
                for (int j = 3; j < curser.cols; j++) {
                        line[j - 3] =
                            inch() & A_CHARTEXT; // Extract character only
                        move(i, j + 1);
                }
                line[curser.cols - 3] = '\0'; // Null-terminate the string
                fprintf(fptr, "%s\n", line);
        }

        fclose(fptr);

        // Notify the user that the file has been saved
        move(curser.rows - 2, 8);
        attron(COLOR_PAIR(1));
        // printw("File written successfully to %s!", path);
        attroff(COLOR_PAIR(1));
        refresh();

        napms(1000);
        move(curser.rows - 2, 8);
        attroff(COLOR_PAIR(1));
}
