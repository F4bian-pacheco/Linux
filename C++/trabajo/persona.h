#ifndef PERSONA_H
#define PERSONA_H


#include <iostream>
#include <string>


using namespace std;



class Persona{

public:
	Persona(string, string, string);
	Persona();
	~Persona();

	string get_nombreCompleto();
	string get_fechaNacimiento();
	string get_nacionalidad();
	void set_nombreCompleto(string);
	void set_fechaNacimiento(string);
	void set_nacionalidad(string);




private:
	string nombreCompleto;
	string fechaNacimiento;
	string nacionalidad;


};

#endif


