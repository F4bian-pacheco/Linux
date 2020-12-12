#ifndef TEMPORADA_H
#define TEMPORADA_H
#include <iostream>
#include <vector>
#include "capitulo.h"
using namespace std;

class Temporada
{
private:
    string nombre;
    string fecha_produccion;
    string fecha_estreno;
    vector<Capitulo> capitulos;


public:
    Temporada();
    Temporada(string, string, string);
    ~Temporada();
    string get_nombre();
    void set_nombre(string);
    string get_fecha_produccion();
    void set_fecha_produccion(string);
    string get_fecha_estreno();
    void set_fecha_estreno(string);
    vector<Capitulo> get_capitulos();
    void set_capitulos(Capitulo);

};

#endif