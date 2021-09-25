#include <stdio.h>
#include <unistd.h>


int main(){
    fork();
    fork();
    fork();

    printf("Soy un proceso\n");
    while(1);
}
