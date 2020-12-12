#include "subtitulo.h"

Subtitulo::Subtitulo(){};
Subtitulo::Subtitulo(string idioma, Persona autor){
    this->idioma = idioma;
    this->autor = autor;
}
Subtitulo::~Subtitulo(){};

string Subtitulo::get_idioma(){return this->idioma;}

void Subtitulo::set_idioma(string idioma){
    this->idioma = idioma;
}

Persona Subtitulo::get_autor(){return this->autor;}

void Subtitulo::set_autor(Persona autor){
    this->autor = autor;
}



