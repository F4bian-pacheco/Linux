#include <iostream>




int main(){
    int ganados, empatados, perdidos, resultado;
    int puntos[] = {3,1,0};
    std::cout <<"partidos ganados: ";
    std::cin >> ganados;
    std::cout << "partidos empatados: ";
    std::cin >> empatados;
    std::cout << "partidos perdidos: ";
    std::cin >> perdidos;

    resultado = ganados*puntos[0]+empatados*puntos[1]+perdidos*puntos[2];
    std::cout << "Puntaje total: "<<resultado<<" puntos"<<std::endl;


}
