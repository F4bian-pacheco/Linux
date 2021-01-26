#ifndef SUBTITULO_H
#define SUBTITULO_H
#include "persona.h"

#include <iostream>

using namespace std;

class Subtitulo{

private:
    string idioma;
    Persona autor;

public:
    Subtitulo();
    Subtitulo(string, Persona);
    ~Subtitulo();

    string get_idioma();
    void set_idioma(string);

    Persona get_autor();
    void set_autor(Persona);

};

#endif