#ifndef CRITICA_H
#define CRITICA_H

#include <iostream>
#include <string>


using namespace std;



class Critica
{
public:
	Critica();
	~Critica();

	string get_fechaCritica();
	string get_comentario();
	void set_fechaCritica(string);
	void set_comentario(string);
private:
	string fechaCritica;
	string comentario;
	
};
#endif 