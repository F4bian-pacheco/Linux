#include <iostream>
#include <algorithm>
#include <vector>


int esPar(int a){
    if(a%2==0) return 1;
    return 0;
}

int main(){
    std::vector<int> array;
    int count,n;

    std::cout << "inserte un numero: "; 
    std::cin >> n;

    while(n-->0){
        array.push_back(n);
    }
    std::reverse(array.begin(),array.end());
    count = std::count_if(array.begin(),array.end(),esPar);
    std::cout << "numeros pares: "<< count << std::endl;
    std::cout << "ultimo elemento: " << array.back()<<std::endl;
    return 0;
}
