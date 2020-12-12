#include "temporada.h"

Temporada::Temporada(){};
Temporada::Temporada(string nombre, string fecha_p, string fecha_e){
    this->nombre = nombre;
    this->fecha_produccion = fecha_p;
    this->fecha_estreno = fecha_e;
}
Temporada::~Temporada(){};


string Temporada::get_nombre(){return this->nombre;}
void Temporada::set_nombre(string nombre){
    this->nombre = nombre;
}
string Temporada::get_fecha_produccion(){return this->fecha_produccion;}
void Temporada::set_fecha_produccion(string fecha_produccion){
    this->fecha_produccion = fecha_produccion;
}
string Temporada::get_fecha_estreno(){return this->fecha_estreno;}
void Temporada::set_fecha_estreno(string fecha_estreno){
    this->fecha_estreno = fecha_estreno;
}

vector<Capitulo> Temporada::get_capitulos(){return this->capitulos;}

void Temporada::set_capitulos(Capitulo cap){
    this->capitulos.push_back(cap);
}