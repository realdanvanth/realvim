/* Text.c */
#include "filehandler/file_handle.h"
#include "keyboard/keyboard.h"
#include "modes/modes.h"
#include <ncurses.h>
#include <unistd.h>

Character curser;

int main(int argc, char *argv[]) {

        if (argc < 1) {
                fprintf(stderr, "Not enough arguments\n");
        }
        // Initialize the ncurses.h
        initscr();

        getmaxyx(stdscr, curser.rows, curser.cols);
        curs_set(1);
        start_color();
        init_pair(1, COLOR_BLACK, COLOR_GREEN);
        init_pair(2, COLOR_GREEN, COLOR_BLACK);
        cbreak();
        noecho();
        clear();
        refresh();

        move(curser.rows - curser.rows, (curser.cols / 2) - 7);
        attron(COLOR_PAIR(1));
        printw("File ~ %s ", argv[3]);
        attron(COLOR_PAIR(2));
        curser.x = 1;
        curser.y = 3;
        for (int i = 1; i < curser.rows - 1; i++) {
                move(i, 0);
                printw(" %d", i);
                refresh();
        }
        attroff(COLOR_PAIR(2));
        move(curser.rows - 2, 0);
        attron(COLOR_PAIR(1));
        attron(A_BOLD);
        printw("NORMAL");
        move(curser.rows - 2, 6);
        for (int i = 0; i < curser.cols - 6; i++)
                addch(' ');
        attroff(A_BOLD);
        attroff(COLOR_PAIR(1));
        refresh();

        open_file(argv[3]);

        curser.x = 1;
        curser.y = 3;
        move(1, 3);
        refresh();
        char c = getch();

        while (1) {
                if (c == 'i')
                        insert_mode();
                else if (c == ':') {
                        move(curser.rows - 1, 0);
                        printw(":");
                        refresh();
                        if (command_mode(argv[3]) == 1) {
                                endwin();
                                return 0;
                        }

                } else {
                        move_cursor(c);
                }
                refresh();
                timeout(3000);
                c = getch();
        }
        refresh();
        endwin();

        return 0;
}
