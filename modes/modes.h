#ifndef modes
#define modes()
typedef enum {
    NORMAL_MODE,
    COMMAND_MODE,
    INSERT_MODE
} Mode;

int command_mode(char *argv);
void insert_mode();
void display_mode();
void switch_mode();

#endif
