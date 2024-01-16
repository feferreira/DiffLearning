#include<stdio.h>
#include<string.h>

int main(int argc, char *argv[])
{
    // simple string
    char str1[] = {'N','i','n','o', '\0'};
    char str2[] = "Nino"; //implicity \0
    char *str3 = "Nino";
    printf("str1 = %s, str2 = %s, str3 = %s\n", str1, str2, str3); 
    
    // string elements
    char str4[5] = {'N'};
    str4[1] = str1[1];
    str4[2] = str2[2];
    str4[3] = str3[3];
    str4[4] = '\0';
    printf("str4 = %s\n", str4);

    //iterate string like an array
    for(int i = 0; i < 4; ++i){
	printf("str[%d] - %c\n", i, *(str1 + i));
    }

    //iterate using strlen
    for(int i = 0; i < strlen(str1); ++i){
	printf("str2[%d] - %c\n", i, str2[i]);
    }
    
}
