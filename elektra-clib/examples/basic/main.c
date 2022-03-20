#include <stdio.h>
#include <stdlib.h>

#include <kdb.h>

int main (void) {
    ElektraKey * key = elektraKeyNew ("user:/test/qwe/asd");
    printf("%s\n", elektraKeyName (key));

    ElektraKey * key2 = elektraKeyNew ("user:/test/qwe/asd/qwe");
    printf("%s\n", elektraKeyName (key2));

    printf("--------------\n");

    printf("%i\n", elektraKeyIsBelow (key, key2));
    printf("%i\n", elektraKeyIsBelow (key2, key));

    printf("--------------\n");

    printf("%i\n", elektraKeyAddName (key, "yyyyyyy"));
    printf("%s\n", elektraKeyName (key));

    printf("%i\n", elektraKeySetName (key, "system:/asd/qwe/asd"));
    printf("%s\n", elektraKeyName (key));

    // printf("%i\n", keySetBinary(key, "abc", 4));

    // printf("%p\n", keyValue (key));
    // printf("%s\n", (char *) keyValue (key));

    elektraKeyDel (key);
    elektraKeyDel (key2);

    // KeySet * ks = ksNew (1, KS_END);
    // ksAppendKey(ks, key);

    // Key * foundKey = ksLookupByName(ks, "test", 0);
    // printf("%s\n", keyName (foundKey));

    // ksDel(ks);
}