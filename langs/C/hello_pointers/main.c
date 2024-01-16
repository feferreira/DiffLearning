#include <stdio.h>

int main(int argc, char *argv[])
{
    //simple pointer
    int var = 10; // copy 10 to var;
    int * pVar = &var; //copy the address of var to pvar;
    printf("1-simple pointer:\n");
    printf("var = %d, address = %x\n\n", var, pVar);

    //dereferencing
    int dVar = *pVar; //dereferencing, get value pointed by pVar and copy to dVar
    printf("2-Dereferencing:\n");
    printf("Dereferencing pVar = %d\n\n", dVar);
    dVar++;
    printf("Changing dVar(%d) does not affect var(%d)\n\n", dVar, var);

    printf("3-copy pointer:\n");
    int *pVar2 = pVar; //copy pVar to pVar2, both pointers stores var address
    printf("pVar2(%x) and pVar(%x) points to the same address\n", pVar2, pVar);
    

    /*
    // simple array
    int ages[] = {12, 15, 11};
    int agesCount = sizeof(ages) / sizeof(int);
    printf("ages[%d] = %d, %d, %d\n", agesCount, ages[0], ages[1], ages[2]);
    
    // simple string
    char string1[5] = {'n'}; //string1 is a pointer to the first element
    
    printf("string1 = %s\n", string1);

    // string elements
    string1[1] = 'i';
    string1[2] = 'n';
    string1[3] = 'o';
    string1[4] = '\0';
    char addrString1 = *string1;
    printf("string1:%d complete = %s\n", addrString1, string1);
    */

}
