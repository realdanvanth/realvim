#include <ncurses.h>
#include <string.h>
#include <unistd.h>
int x;
int y;
int rows;
int cols;
void check_cursor() {
     if (y == cols - 1) {
          y = 3;
          x++;
          move(x, y);
     } else if (y <= 3) {
          y = 3;
          move(x, y);
     }
     if (x == 0) {
          x = 1;
          move(x, y);
     }
     if (x >= rows - 2) {
          x = rows - 3;
          move(x, y);
     }
}
void open_file(char path[]) {
     move(rows - 2, cols - 10);
     attron(COLOR_PAIR(1));
     attron(A_BOLD);
     printw("%s", path);
     attroff(A_BOLD);
     attroff(COLOR_PAIR(1));
     refresh();
     move(x, y);
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
                    move(x, y++);
                    check_cursor();
                    printw("%c", data[i]);
               } else if (data[i] != ERR) {
                    move(++x, 3);
                    y = 3;
                    check_cursor();
               }
          }
     }
     refresh();
     fclose(fptr);
}
void display(char c) {
     move(x, y++);
     check_cursor();
     printw("%c", c);
     refresh();
}
int command_mode() {
     move(rows - 2, 0);
     attron(COLOR_PAIR(1));
     attron(A_BOLD);
     printw("COMMAND");
     attroff(A_BOLD);
     attroff(COLOR_PAIR(1));
     move(x, y);
     refresh();
     int cx = rows - 1;
     int cy = 1;
     move(cx, cy);
     char input[10] = {0};
     echo();
     getstr(input);
     for (int j = 0; j < 10; j++) {
          if (input[j] == 'q') {
               return 1;
          }
     }
     noecho();
     move(rows - 2, 0);
     attron(COLOR_PAIR(1));
     attron(A_BOLD);
     printw("VISUAL ");
     attroff(A_BOLD);
     attroff(COLOR_PAIR(1));
     move(x, y);
     refresh();
     move(rows - 1, 0);
     clrtoeol();
     move(x, y);
     refresh();
     return 0;
}
void insert_mode() {
     move(rows - 2, 0);
     attron(COLOR_PAIR(1));
     attron(A_BOLD);
     printw("INSERT");
     attroff(A_BOLD);
     attroff(COLOR_PAIR(1));
     move(x, y);
     refresh();
     timeout(5000);
     char c = getch();
     while (c != 27) {
          if (c == 127) {
               move(x, --y);
               check_cursor();
               delch();
               refresh();
          } else {
               move(x, y++);
               check_cursor();
               refresh();
               insch(c);
          }
          timeout(5000);
          c = getch();
     }

     move(rows - 2, 0);
     attron(COLOR_PAIR(1));
     attron(A_BOLD);
     printw("VISUAL");
     attroff(A_BOLD);
     attroff(COLOR_PAIR(1));
     move(x, y);
     refresh();
     noecho();
}
void move_cursor(char c) {
   switch (c) {
    case 'a':
        move(x, --y); break;
    case 'w': 
        move(--x, y); break;
    case 'd': 
        move(x, ++y); break;
    case 's':
        move(++x, y); break;
     }
     check_cursor();
     }
     refresh();
}
int main(int argc, char *argv[]) {
     // rows = atoi(argv[1]);
     // cols = atoi(argv[2]);
     initscr();
     getmaxyx(stdscr, rows, cols);
     curs_set(1);
     start_color();
     init_pair(1, COLOR_BLACK, COLOR_GREEN);
     init_pair(2, COLOR_GREEN, COLOR_BLACK);
     cbreak();
     noecho();
     clear();
     refresh();
     move(rows - rows, (cols / 2) - 7);
     attron(COLOR_PAIR(1));
     printw("C TEXT EDITOR %s ", argv[3]);
     attron(COLOR_PAIR(2));
     x = 1;
     y = 3;
     for (int i = 1; i < rows - 1; i++) {
          move(i, 0);
          printw(" %d", i);
          refresh();
     }
     attroff(COLOR_PAIR(2));
     move(rows - 2, 0);
     attron(COLOR_PAIR(1));
     attron(A_BOLD);
     printw("VISUAL");
     move(rows - 2, 6);
     for (int i = 0; i < cols - 6; i++)
          addch(' ');
     attroff(A_BOLD);
     attroff(COLOR_PAIR(1));
     refresh();
     open_file(argv[3]);
     x = 1;
     y = 3;
     move(1, 3);
     refresh();
     char c = getch();
     while (1) {
          if (c == 'i')
               insert_mode();
          else if (c == ':') {
               move(rows - 1, 0);
               printw(":");
               refresh();
               if (command_mode() == 1) {
                    endwin();
                    return 0;
               }
          }

          else
               move_cursor(c);
          refresh();
          timeout(5000);
          c = getch();
     }
     refresh();
     endwin();
     return 0;
}
