#include <stdio.h>
#include <stdlib.h>

#include <elektra.h>

int main (void) {
    Key * key = keyNew ("testqq", KEY_END);
    printf("%s\n", keyName (key));

    keySetName (key, "qweqwe");
    printf("%s\n", keyName (key));

    keyDel (key);

    // KeySet * ks = ksNew (1, KS_END);
    // ksAppendKey(ks, key);

    // Key * foundKey = ksLookupByName(ks, "test", 0);
    // printf("%s\n", keyName (foundKey));

    // ksDel(ks);
}