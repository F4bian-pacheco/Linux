#ifndef PLATAFORMA_H
#define PLATAFORMA_H

#include <iostream>
#include <vector>
#include "persona.h"
#include "pais.h"


class Plataforma{

public:
    Plataforma();
    ~Plataforma();

    string get_nombre();
    void set_nombre(string);
    Persona get_gerente();
    void set_gerente(Persona);
    string get_fecha_creacion();
    void set_fecha_creacion(string);
    string get_reseña_historica();
    void set_reseña_historica(string);
    vector<Pais> get_paises();
    void insertar_pais(Pais);

private:
    string nombre;  
    Persona gerente;
    string fecha_creacion;
    string reseña_historica;
    vector<Pais> paises;
};


#endif