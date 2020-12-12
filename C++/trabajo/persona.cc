#include "persona.h"

Persona::Persona(string nombre, string fecha, string nacionalidad){
	this->nombreCompleto = nombre;
	this->fechaNacimiento = fecha;
	this->nacionalidad = nacionalidad;
}
Persona::Persona(){};

Persona::~Persona(){};

string Persona::get_nombreCompleto(){return this->nombreCompleto;}
string Persona::get_fechaNacimiento(){return this->fechaNacimiento; }
string Persona::get_nacionalidad(){return this->nacionalidad;}
void Persona::set_nombreCompleto(string nombreCompleto ){

	this->nombreCompleto = nombreCompleto;
}
void Persona::set_fechaNacimiento(string fechaNacimiento){

	this->fechaNacimiento=fechaNacimiento;
}
void Persona::set_nacionalidad(string nacionalidad){

	this->nacionalidad=nacionalidad;

}
