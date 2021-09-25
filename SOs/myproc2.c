#include <stdio.h>
#include <unistd.h>


int main(){

    for(int i = 0;i<3;i++){
        if(fork()==0) break;   
    }
    while(1);
}
