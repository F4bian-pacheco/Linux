
#include <iostream>
#include <cstdio>


int main(){
    
    int input;
    std::cout << "cuantas notas ingresara: ";
    std::cin >> input;
    double notas[input];
    int contador = 0;

    for (int i = 0;i < input;i++) {
        std::cout << "ingrese nota "<<i+1 << ":";
        std::cin >> notas[i];
        if(notas[i]>4.0){
            contador++;
        }
    }
    
    std::cout << "Aprobados: "<<contador<<std::endl;
        


    return 0;
}





