#include "serie.h"

Serie::Serie(){};
Serie::Serie(string titulo, int anio, string sinopsis, string genero, string idioma){
    this->titulo = titulo;
    this->anio_inicio = anio;
    this->sinopsis = sinopsis;
    this->genero = genero;
    this->idioma_original = idioma;

}
Serie::~Serie(){};

string Serie::get_titulo(){return this->titulo;}
void Serie::set_titulo(string titulo){
    this->titulo = titulo;
}
int Serie::get_anio_inicio(){return this->anio_inicio;}
void Serie::set_anio_inicio(int anio_inicio){
    this->anio_inicio = anio_inicio;
}
string Serie::get_sinopsis(){return this->sinopsis;}
void Serie::set_sinopsis(string sinopsis){
    this->sinopsis = sinopsis;
}
string Serie::get_genero(){return this->genero;}
void Serie::set_genero(string genero){
    this->genero = genero;
}
string Serie::get_idioma_original(){return idioma_original;}
void Serie::set_idioma_original(string idioma_orignal){
    this->idioma_original = idioma_original;
}
float Serie::get_puntuacion_media(){return puntuacion_media;}
void Serie::set_puntuacion_media(float puntuacion_media){
    this->puntuacion_media += puntuacion_media;
}

vector<Temporada> Serie::get_temporada(){return this->temporadas;}
void Serie::set_temporada(Temporada temp){
    this->temporadas.push_back(temp);
}