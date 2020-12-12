#include "serie.h"
#include "usuario.h"

void registrar_usuario();
void iniciar_sesion();
void ver_series(vector<Serie>);
void menu_admin();
void Ingresar_series();
void modificar_series();
void info_serie(int);

Usuario us;
vector<Serie> series2;

int main(int argc, char const *argv[])
{
    

    Serie s1("holaa1",123,"123",Genero::Accion,"123");
    Serie s2("holaa2",124,"124",Genero::Accion,"123"); 
    Serie s3("holaa3",125,"125",Genero::Accion,"123"); 
    series2.push_back(s1);
    series2.push_back(s2);
    series2.push_back(s3);


    int op;
    do{
        system("clear");
        cout<<"----------Menu--------\n";
        cout<<"1) registrar usuario."<<endl;
        cout<<"2) iniciar sesion."<<endl;
        cout<<"0) Salir"<<endl;
        cout<<"\ningresar opcion: ";
        cin >>op;

    switch (op)
    {
    case 1:
        registrar_usuario();
        break;
    
    case 2:
        iniciar_sesion();
        break;
    }

    }while(op!=0);


    return 0;
}


void registrar_usuario(){
    int op;
    while (op!=0)
    {
        system("clear");
        cout<<"-----Registrar Usuario------"<<endl;
        string nom;
        cout<<"nombre: ";
        cin>>nom;
        us.set_nombreCompleto(nom);
        cout <<"\nsalir(0): ";
        cin>>op;
    }
}

void iniciar_sesion(){
    int op;
    while (op!=0)
    {
        system("clear");
        cout<<"-----Inicio Sesion------"<<endl;
        string nom_us;
        cout <<"ingrese su nombre:";
        cin>>nom_us;
        if(us.get_nombreCompleto() == nom_us){
            ver_series(series2);
            //cout<<"viendo menu usuario\n";
        }else if(nom_us == "admin"){
            menu_admin();
            //cout<<"viendo menu admin\n";

        }else{
            cout<<"nombre no registrado, intente registrarse\n";
        }
        cout <<"salir?(0): ";
        cin>>op;
    }
}




void menu_admin(){
  int md;
    do{
        system("clear");
        cout <<"----Menu Admin----\n";
        cout<<"1) Ver Series\n";
        cout<<"2) Ingresar Series\n";
        cout<<"3) Modificar Serie\n";
        cout <<"\nsalir(0): ";
        cout<<"\ningresar opcion: ";
        cin >>md;

    switch (md)
    {
    case 1:
        ver_series(series2);
        break;
    
    case 2:
        //Ingresar_series();
        break;
    case 3:

        //modificar_series();
        break;
    }

    }while(md!=0);
}



void ver_series(vector<Serie> Lista){
    int md;
    while(md!=0){
    system("clear");
    for (int i = 0; i < Lista.size(); ++i)
    {
        cout << i+1<<".-"<< Lista[i].get_titulo()<<endl;
    }
    cout <<"\nsalir(0): ";
    cout<<"\ningresar opcion: ";
    cin>>md;
    if (0<md && md<=Lista.size())
        //cout<<"\nui";
        info_serie(md-1);
    }


}

void info_serie(int pos){
    int op;
    while(op!=0){
        system("clear");
        cout<<"-----"<<series2[pos].get_titulo()<<"-----"<<endl;
        cout<<series2[pos].get_sinopsis()<<endl;
        //cout<<
        cout <<"\nsalir(0): ";
        cout<<"\ningresar opcion: ";
        cin >>op;
    }
}
