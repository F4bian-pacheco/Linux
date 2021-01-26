#include <stdio.h>

int main(){
    
    char* a = "hola";
    char res[11];
    char* abcd = "abcdefghijklmnopqrstuvwxyz";
    for(int i = 0;a[i]!='\0';i++){
        for(int j = 0;abcd[j]!='\0';j++){
            if(a[i] == abcd[j]){ 
                printf("%d ",j+1);
            }
        }
    }
    return 0;
} 
