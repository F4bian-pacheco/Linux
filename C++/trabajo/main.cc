#include "serie.h"



int main(int argc, char const *argv[])
{
    int a;
    cout <<"cuantas series ingresara?: ";
    cin >> a;
    // vector<Serie> series2;

    // Serie s1("holaa",123,"123",Genero::Accion,"123");
    // Serie s2("holaa",123,"123",Genero::Accion,"123"); 
    // Serie s3("holaa",123,"123",Genero::Accion,"123"); 

    // series2.push_back(s1);
    // series2.push_back(s2);
    // series2.push_back(s3);

    vector<Serie> series;

    for (int i = 0; i < a; i++)
    {   
        string nom;
        cout <<"nombre serie "<<i+1<<": ";
        cin >> nom;
        Serie a(nom,i,"123",Genero::Accion,"123");
        series.push_back(a);
        for (int j = 0; j < 2; j++)
        {
            string nomt;
            cout <<"temp: ";
            cin >> nomt;
            Temporada b;
            b.set_nombre(nomt);
            

            for (int k = 0; k < 2; k++)
            {
                string nomC;
                cout <<"cap "<<k+1<<endl;
                cin>>nomC;
                Capitulo c;
                c.set_titulo(nomC+to_string(k+1));
                b.set_capitulos(c);
            }
            series[i].set_temporada(b);
        }
    }

    // Serie* series = new Serie[a];

    // for (int i = 0; i < a; i++)
    // {
    //     series[i] = Serie("holaa",i+1,"123",Genero::Accion,"123");
    // }
    
    
    for (int i = 0; i < a; i++)
    {
        cout <<"serie "<<i+1<<": "<< series[i].get_titulo()<<"\n"<<series[i].get_anio_inicio()<<endl;

        vector <Temporada> tempp = series[i].get_temporada();
        for (int k=0;k<tempp.size();k++)
        {
            cout <<"  -temp "<<k+1<<": "<<tempp[k].get_nombre()<<endl;
            vector<Capitulo> capp = tempp[k].get_capitulos();
            for (int p = 0; p < capp.size(); p++)
            {
               cout <<"      -cap "<<p+1<<": "<<capp[p].get_titulo()<<endl; 
            }
            

        }
        
        cout <<"--------------\n";
    }
    


}