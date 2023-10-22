#include <stddef.h>


#define VGA_WIDTH 160
#define VGA_HEIGHT 25


unsigned char* scr_buff = (unsigned char*)0xb8000;
size_t line_pos[VGA_HEIGHT];

size_t x, y;



void terminal_char_at(size_t x, size_t y, char character, char color){
    scr_buff[(x*2)+(y*VGA_WIDTH)] = character;
    scr_buff[(x*2)+(y*VGA_WIDTH)+1] = color;
}

void terminal_clear(){
    for(size_t x = 0; x < VGA_WIDTH; x++){
        for(size_t y = 0; y < VGA_HEIGHT; y++){
            terminal_char_at(x, y, ' ', 0);
        }
    }
}
void terminal_init(){
    terminal_clear();
    x = y = 0;

}

unsigned strlen(const unsigned char* string){
    const unsigned char* oldPos = string;
    while(*string != 0x0){
        string++;
    }
    return string-oldPos;
}

void print(const unsigned char* string){
    unsigned len = strlen(string);
    for(int i = 0; i < len; i++){
        if(string[i] == '\n'){
            line_pos[y] = x;
            y++;
            x = 0;       
        }
        else{
            terminal_char_at(x, y, string[i], 15);
            
            x++;
        }
    }
}

void kernel_main(void){
    terminal_clear();
    print("I am become insane, thats the quote right?");

}