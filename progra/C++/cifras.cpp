#include <iostream>
#include <cstdio> // incluir funciones de c

int main(){
    int num;
    std::cout << "ingrese un numero: ";
    std::cin >>num;    
    int cifras = 1;
    while(num >= 10){
        num /= 10;
        cifras++;
    }
    std::cout <<"el numero "<<num<<" tiene "<<cifras<<" cifras"<<std::endl;
}
