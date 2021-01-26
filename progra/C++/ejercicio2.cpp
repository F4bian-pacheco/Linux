#include <iostream>
#include <cstdio>

double promedio(double, double, double);

int main(){
    double a,b,c;
    int resultado;
    
    printf("ingrese las notas:\n");
    scanf("%lf %lf %lf",&a,&b,&c);
    printf("El promedio del alumno es: %.2lf\n",promedio(a,b,c));
}

double promedio(double a, double b, double c){
    return (a+b+c)/3;
}




