#include <iostream>
#include <cstdio>
#include <cmath>

double semiarea(double, double, double);
double area(double, double, double,double);



int main(){
    double a,b,c,s, res;
    printf("inserte los lados del triangulo: ");
    std::cin >> a >> b >> c;
    s = semiarea(a,b,c);
    res = area(a,b,c,s);

    printf("\nEl area del triangulo es: %f\n", res);

    return 0;
}

double semiarea(double a, double b, double c){
    double s;

    s = (a+b+c)/2;
    return s;
}

double area(double a, double b, double c,double s){
    double res = (s * (s-a) * (s-b) * (s-c));
    res = sqrt(res);
    return res;
}
