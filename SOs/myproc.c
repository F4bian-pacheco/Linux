#include <stdio.h>
#include <unistd.h>



int main(){
    for(int i = 0;i<2;i++){
        printf("Soy un proceso dentro del bucle\n");
        fork();
    }
    fork();
    
    printf("soy un proceso afuera del bucle\n");
    while(1);
}
