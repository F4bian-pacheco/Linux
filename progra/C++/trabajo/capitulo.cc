#include "capitulo.h"

Capitulo::Capitulo(){};
Capitulo::Capitulo(string titulo,int duracion,string sinopsis,string fecha_estreno){
    this->titulo = titulo;
    this->duracion = duracion;
    this->sinopsis = sinopsis;
    this->fecha_estreno = fecha_estreno;
    
};
Capitulo::~Capitulo(){};

string Capitulo::get_titulo(){return this->titulo;}
void Capitulo::set_titulo(string titulo){
    this->titulo = titulo;
}
int Capitulo::get_duracion(){return this->duracion;}
void Capitulo::set_duracion(int){
    this->duracion = duracion;
}
string Capitulo::get_sinopsis(){return this->sinopsis;}
void Capitulo::set_sinopsis(string){
    this->sinopsis = sinopsis;
}
string Capitulo::get_fecha_estreno(){return this->fecha_estreno;}
void Capitulo::set_fecha_estreno(string fecha_estreno){
    this->fecha_estreno = fecha_estreno;
}
vector<Actor> Capitulo::get_actores(){return this->actores;}
void Capitulo::set_actor(Actor act){
    this->actores.push_back(act);
}
Director Capitulo::get_director(){return this->director;}
void Capitulo::set_director(Director director){
    this->director = director;
}
Guionista Capitulo::get_guionista(){return this->guionista;}
void Capitulo::set_guionista(Guionista guionista){
    this->guionista = guionista;
}
vector<Productor> Capitulo::get_productores(){return this->productores;}
void Capitulo::set_productores(Productor prod){
    this->productores.push_back(prod);
}