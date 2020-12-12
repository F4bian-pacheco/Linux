#ifndef PAIS_H
#define PAIS_H



#include <iostream>
#include <string>


using namespace std;

class Pais
{
public:
	Pais();
	~Pais();

	Persona get_gerenteregional();
	void set_gerenteregional(Persona);

private:
	Persona gerenteregional;

};
#endif 
