#include "serie.h"
#include "usuario.h"

void registrar_usuario();
void iniciar_sesion();
void ver_series();
void menu_admin();


Usuario us;

int main(int argc, char const *argv[])
{
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
            //ver_series();
            cout<<"viendo menu usuario\n";
        }else if(nom_us == "admin"){
            //menu_admin();
            cout<<"viendo menu admin\n";

        }else{
            cout<<"nombre no registrado, intente registrarse\n";
        }
        cout <<"\nsalir(0): ";
        cin>>op;
    }
}
