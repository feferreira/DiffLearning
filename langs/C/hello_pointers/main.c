#include <stdio.h>

int main(int argc, char *argv[])
{
    // simple array
    int ages[] = {12, 15, 11};
    int agesCount = sizeof(ages) / sizeof(int);
    printf("ages[%d] = %d, %d, %d\n", agesCount, ages[0], ages[1], ages[2]);
    
    // simple string
    char string1[5] = {'n'};
    printf("string1 = %s\n", string1);

    // string elements
    string1[1] = 'i';
    string1[2] = 'n';
    string1[3] = 'o';
    string1[4] = '\0';
    printf("string1 complete = %s\n", string1);

}
