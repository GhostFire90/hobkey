#include "stdlib.h"

char *itoa(int value, char *str, int base)
{
    unsigned i = 1;
    int sign = value < 0;
    str[0] = 0x0;
    if(value == 0){
        str[i] = '0';
        i++;
    }
    while(value){
        int digit = value % base;
        if (digit < 0) digit *= -1;
        value /= base;
        if(digit <= 9)
            str[i] = digit + '0';
        else{
            digit -= 10;
            str[i] = digit + 'A';
        }
        i++;
    }
    if(sign){
        str[i] = '-';
        i++;
    }
    for(int j = 0; j < i/2; j++){
        char old = str[j];
        str[j] = str[i-j-1];
        str[i-j-1] = old;
    }

    return str;
}