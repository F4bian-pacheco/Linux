#include "usuario.h"


Usuario::Usuario(){};
Usuario::~Usuario(){};

string Usuario::get_correoEle(){return this->correoEle;}
void Usuario::set_correoEle(string correoEle){

	this->correoEle= correoEle;
}

Critica Usuario::get_critica(){return this->critica;}

void Usuario::set_critica(Critica critica){
	this->critica = critica;
}