#ifndef CAPITULO_H
#define CAPITULO_H
#include <iostream>
#include <vector>
#include "actor.h"
#include "director.h"
#include "guionista.h"
#include "productor.h"
#include "subtitulo.h"

using namespace std;

class Capitulo
{

public:
    Capitulo();
    Capitulo(string,int,string,string);
    ~Capitulo();

    string get_titulo();
    void set_titulo(string);
    int get_duracion();
    void set_duracion(int);
    string get_sinopsis();
    void set_sinopsis(string);
    string get_fecha_estreno();
    void set_fecha_estreno(string);
    vector<Actor> get_actores();
    void set_actor(Actor);
    Director get_director();
    void set_director(Director);
    Guionista get_guionista();
    void set_guionista(Guionista);
    vector<Productor> get_productores();
    void set_productores(Productor);
    Subtitulo get_subtitulo();
    void set_subtitulo(Subtitulo);

private:
    string titulo;
    int duracion;
    string sinopsis;
    string fecha_estreno;
    vector<Actor> actores;
    Director director;
    Guionista guionista;
    vector<Productor> productores;
    Subtitulo subtitulo;
};
#endif