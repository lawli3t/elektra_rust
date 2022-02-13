#include <stdio.h>
#include <stdlib.h>

#include <elektra.h>

int main (void) {
    Key * key = keyNew ("user:/test/qwe/asd", KEY_END);
    printf("%s\n", keyName (key));

    Key * key2 = keyNew ("user:/test/qwe/asd/qwe", KEY_END);
    printf("%s\n", keyName (key2));

    printf("--------------\n");

    printf("%i\n", keyIsBelow (key, key2));
    printf("%i\n", keyIsBelow (key2, key));

    printf("--------------\n");

    printf("%i\n", keyAddName (key, "yyyyyyy"));
    printf("%s\n", keyName (key));

    printf("%i\n", keySetName (key, "system:/asd/qwe/asd"));
    printf("%s\n", keyName (key));

    keyDel (key);
    keyDel (key2);

    // KeySet * ks = ksNew (1, KS_END);
    // ksAppendKey(ks, key);

    // Key * foundKey = ksLookupByName(ks, "test", 0);
    // printf("%s\n", keyName (foundKey));

    // ksDel(ks);
}