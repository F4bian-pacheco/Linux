#ifndef USUARIO_H
#define USUARIO_H

#include "persona.h"
#include "critica.h"
#include <iostream>
#include <string>


using namespace std;


class Usuario: public Persona{

public:
	Usuario();
	~Usuario();

	string get_correoEle();
	void set_correoEle(string);
	Critica get_critica();
	void set_critica(Critica);
private:
	string correoEle;
	Critica critica;

};

#endif 
