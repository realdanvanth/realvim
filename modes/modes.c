#include "modes.h"
#include "filehandler/file_handle.h"
#include "keyboard/keyboard.h"
#include <ncurses.h>
#include <string.h>
#include <unistd.h>

extern Character curser;

static Mode current_mode = NORMAL_MODE; // default its in normal mode
void display_mode() {
        move(curser.rows - 1, 0); // Move to the bottom row
        clrtoeol();               // Clear the line to avoid leftover text

        if (current_mode == NORMAL_MODE) {
                attron(COLOR_PAIR(1)); // Highlight for Normal Mode
                printw("Mode: Normal");
                attroff(COLOR_PAIR(1));
        } else if (current_mode == COMMAND_MODE) {
                attron(COLOR_PAIR(2)); // Highlight for Command Mode
                printw("Mode: Command");
                attroff(COLOR_PAIR(2));
        }

        refresh(); // Refresh the screen to show changes
}
void switch_mode() {
        // Toggle between NORMAL_MODE and COMMAND_MODE
        if (current_mode == NORMAL_MODE) {
                current_mode = COMMAND_MODE;
        } else {
                current_mode = NORMAL_MODE;
        }
        display_mode(); // Update the mode section
}

int command_mode(char *argv) {
        move(curser.rows - 2, 0);
        attron(COLOR_PAIR(1));
        attron(A_BOLD);
        printw("COMMAND");
        attroff(A_BOLD);
        attroff(COLOR_PAIR(1));
        move(curser.x, curser.y);
        refresh();
        int cx = curser.rows - 1;
        int cy = 1;
        move(cx, cy);
        char input[10] = {0};
        echo();
        getstr(input);

        // remove loop, which is not necessary
        if (strcmp(input, "q") == 0) { // Quit
                return 1;
        } else if (strcmp(input, "w") == 0) {
                save_file(argv); // Save to the given file path
                printw("File Written successfully %s", argv);

        } else if (strcmp(input, "wq") == 0) { // Save and quit
                save_file(argv);
                return 1;
        }

        noecho();
        move(curser.rows - 2, 0);
        attron(COLOR_PAIR(1));
        attron(A_BOLD);
        printw("COMMAND");
        attroff(A_BOLD);
        attroff(COLOR_PAIR(1));
        move(curser.x, curser.y);
        refresh();
        move(curser.rows - 1, 0);
        clrtoeol();
        move(curser.x, curser.y);
        refresh();
        return 0;
}

void insert_mode() {
        move(curser.rows - 2, 0);
        attron(COLOR_PAIR(1));
        attron(A_BOLD);
        printw("INSERT");
        attroff(A_BOLD);
        attroff(COLOR_PAIR(1));
        move(curser.x, curser.y);
        refresh();
        timeout(5000);
        char c = getch();
        while (c != 27) {
                if (c == 127) {
                        move(curser.x, --curser.y);
                        check_cursor();
                        delch();
                        refresh();
                } else if (c == '\n') {
                        // returns to the new row
                        move_cursor(c);
                } else {
                        move(curser.x, curser.y++);
                        check_cursor();
                        refresh();
                        insch(c);
                }
                timeout(5000);
                c = getch();
        }

        move(curser.rows - 2, 0);
        attron(COLOR_PAIR(1));
        attron(A_BOLD);
        printw("NORMAL");
        attroff(A_BOLD);
        attroff(COLOR_PAIR(1));
        move(curser.x, curser.y);
        refresh();
        noecho();
}
