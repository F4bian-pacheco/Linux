#include <iostream>
#include "serie.h"

using namespace std;

void ingresar_serie();
void ver_series(vector<Serie>);

vector<Serie> series2;
int main(int argc, char const *argv[])
{
    ingresar_serie();
    return 0;
}


void ingresar_serie(){
    int op;
    while (op!=0)
    {   
        if (op == 8)
        {
            ver_series(series2);
            cout<<"\ncontinuar?(1,0):";
            cin>>op;
        }
        
        system("clear");
        cout<<"-----Ingresar Serie------\n";
        Serie s;
        string titulo;
        int anio_inicio;
        string sinopsis;
        string genero;
        int num_temps;
        cout<<"Titulo: ";
        getline(cin,titulo);

        cout<<"\nAño inicio: ";
        cin>>anio_inicio;
        //cin.ignore();
        //cin>>anio_inicio>>ws;
        //cin>>sinopsis>>ws;
        cout<<"Genero: ";
        cin>>genero;
        cout<<"Numero de temporadas: ";
        cin>>num_temps;
        cin.ignore();
        cout<<"\nSinopsis:\n";
        getline(cin,sinopsis);
        s.set_titulo(titulo);
        s.set_anio_inicio(anio_inicio);
        s.set_sinopsis(sinopsis);
        s.set_genero(genero);
        for (int i = 0; i < num_temps; i++)
        {
            system("clear");
            Temporada t;
            string fecha_estreno;
            string fecha_produccion;
            int num_caps;
            cout<<"-----Temporada "<<i+1<<"----"<<endl;
            cout<<"Fecha produccion: ";
            cin>>fecha_produccion;
            cout<<"Fecha estreno:";
            cin>>fecha_estreno;
            cout<<"Numero de capitulos: ";
            cin>>num_caps;
            t.set_nombre("Temporada "+to_string(i+1));
            t.set_fecha_produccion(fecha_produccion);
            t.set_fecha_estreno(fecha_estreno);

            for (int j = 0; j < num_caps; j++)
            {
                system("clear");

                Capitulo c;
                Subtitulo p;
                Persona h;
                p.set_idioma("Ingles");
                Persona autor("Raymond Chandler","23-07-1998","Estadounidense");
                p.set_autor(h);
                int duracion;
                string fecha_estrenoC;
                string sinopsisC;
                cout<<"------Capitulo "<<j+1<<"------"<<endl;
                cout<<"\nduracion(minutos): ";
                cin>>duracion;
                cout<<"\nfecha estreno: ";
                cin>>fecha_estrenoC;
                cout<<"\nSinopsis:\n";
                getline(cin,sinopsisC);
                c.set_titulo("capitulo"+to_string(j+1));
                c.set_duracion(duracion);
                c.set_fecha_estreno(fecha_estrenoC);
                c.set_sinopsis(sinopsisC);

                t.set_capitulos(c);

            }
            s.set_temporada(t);
        }
        
        series2.push_back(s);

        cout<<"\n0)Salir";
        cout<<"\n8)Ver";
        cout<<"\n9)Seguir";
        cout<<"\nOpcion: ";
        cin>>op;
    }
    
}



void ver_series(vector<Serie> s){
    system("clear");
    int j = 1;
    for(auto i:s){
        cout<<"----Serie "<<j<<"----"<<endl;
        cout<<"titulo: "<<i.get_titulo()<<endl;
        cout<<"año inicio: "<<i.get_anio_inicio()<<endl;
        cout<<"genero: "<<i.get_genero()<<endl;
        cout<<"sinopsis: \n"<<i.get_sinopsis()<<endl;
        for (auto k:i.get_temporada())
        {
            cout<<"     ----"<<k.get_nombre()<<"----"<<endl;
            cout<<"     fecha produccion: "<<k.get_fecha_produccion()<<endl;
            cout<<"     fecha estreno: "<<k.get_fecha_estreno()<<endl;
            
        }
        
        cout<<"------------\n";
        j++;
    }

}