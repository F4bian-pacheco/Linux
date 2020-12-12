#include "plataforma.h"

Plataforma::Plataforma(){}
Plataforma::~Plataforma(){}

string Plataforma::get_nombre(){return this->nombre;}
void Plataforma::set_nombre(string nombre){
    this->nombre = nombre;
}
Persona Plataforma::get_gerente(){return this->gerente;}
void Plataforma::set_gerente(Persona gerente){
    this->gerente = gerente;
}
string Plataforma::get_fecha_creacion(){return this->fecha_creacion;}
void Plataforma::set_fecha_creacion(string fecha_creacion){
    this->fecha_creacion = fecha_creacion;
}
string Plataforma::get_reseña_historica(){return this->reseña_historica;}
void Plataforma::set_reseña_historica(string reseña_historica){
    this->reseña_historica = reseña_historica;
}
vector<Pais> Plataforma::get_paises(){return this->paises;}
void Plataforma::insertar_pais(Pais pais){
    this->paises.push_back(pais);
}