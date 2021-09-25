#include <stdio.h>
#include <unistd.h>


int main(){
    int pid;
    pid = fork();

    if(pid > 0){
        printf("Soy el proc padre, mi pid es %d y el de mi padre es %d\n", getpid(), getppid());
        while(1);
    } else{
        if (pid == 0){
            printf("Soy el proceso hijo. mi pid es %d y el de mi padre es %d\n", getpid(), getppid());
        } else {
            printf("ha ocurrido un error\n");
        }
    }
}
