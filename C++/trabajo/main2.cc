#include "serie.h"



int main(int argc, char const *argv[])
{
    int a;
    cout <<"cuantas series ingresara?: ";
    cin >> a;
    
    vector<Serie*> series;
    vector<Temporada*> temps;

    for (int i = 0; i < a; i++)
    {
        Serie* b = new Serie("hola2",i,"123",Genero::Accion,"123");
        
        series.push_back(b);
    }


            
    for (int j = 0; j < a; j++)
    {
        
        Temporada* b = new Temporada();
        b->set_nombre("temp "+to_string(j));
        cout <<b->get_nombre()<<endl;
    }
    
    // for (int j = 0; j < series.size(); j++)
    // {
    //     cout <<"--> "<<series[j]->get_titulo()<<" "<<series[j]->get_anio_inicio()<<endl;
    //     cout<<"--------------\n";
    // }
    
    for (auto *serie:series)
    {
        cout <<"--> "<<serie->get_titulo()<<" "<<serie->get_anio_inicio()<<endl;
        cout<<"--------------\n";
    }


}