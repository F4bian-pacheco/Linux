#include "critica.h"

Critica::Critica(){};
Critica::~Critica(){};

string Critica::get_fechaCritica(){return this->fechaCritica;}
string Critica::get_comentario(){return this->comentario;}
void Critica::set_fechaCritica(string fechaC){
    this->fechaCritica = fechaC;
}
void Critica::set_comentario(string comen){
    this->comentario = comen;
}