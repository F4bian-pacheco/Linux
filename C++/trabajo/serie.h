#ifndef SERIE_H
#define SERIE_H
#include <iostream>
#include "temporada.h"
#include <vector>
using namespace std;

enum class Genero
{
    Accion,
    Aventura,
    Animacion,
    Comedia,
    Documental,
    Drama,
    Horror,
    Musical,
    Romance,
    CienciaFiccion
};



class Serie
{

public:
    Serie();
    Serie(string, int, string, string, string);
    ~Serie();
    string get_titulo();
    void set_titulo(string);
    int get_anio_inicio();
    void set_anio_inicio(int);
    string get_sinopsis();
    void set_sinopsis(string);
    string get_genero();
    void set_genero(string);
    string get_idioma_original();
    void set_idioma_original(string);
    float get_puntuacion_media();
    void set_puntuacion_media(float);
    vector<Temporada> get_temporada();
    void set_temporada(Temporada);

private:
    string titulo;
    int anio_inicio;
    string sinopsis;
    string genero;
    string idioma_original;
    float puntuacion_media = 0;
    vector<Temporada> temporadas;
};

#endif
