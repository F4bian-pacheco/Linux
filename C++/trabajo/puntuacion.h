#ifndef PUNTUACION_H
#define PUNTUACION_H


#include <iostream>
#include <string>


using namespace std;

class Puntuacion
{
public:
	Puntuacion();
	~Puntuacion();
	int get_puntaje();
	void set_puntaje(int);

private:
	int puntaje;
};

#endif