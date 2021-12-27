#include <elektra.h>
#include <stdio.h>

int main (void) {
    Key * key = keyNew ("tesqqt", KEY_END);
    printf("%s\n", keyString (key));
    keyDel (key);
}