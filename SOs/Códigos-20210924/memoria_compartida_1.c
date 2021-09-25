#include <stdio.h>
#include <sys/shm.h>
#include <unistd.h>

int main() {
	key_t Clave;
	int Id_Memoria;
	int *Memoria = NULL;
	int i,j;

	/*
	Conseguimos una clave para la memoria compartida. Todos los
	procesos que quieran compartir la memoria, deben obtener la misma
	clave. Esta se puede conseguir por medio de la función ftok().

	La función ftok() viene del acrónimo File TO Key, que genera un
	valor único para ser usado por funciones IPC.
	
	Esta función toma la ruta a un fichero existente en el sistema
	y un segundo parámetro que se trata de un valor cualquiera, pero
	que tiene que ser el mismo en todos los procesos que se desea
	sincronizar.
	*/

		Clave = ftok ("/bin/ls", 33);
		if (Clave == -1) {
			printf("%s\n", "No consigo clave para memoria compartida");
			return 0;
		}

	/*
	Creamos la memoria con la clave recién conseguida. Para ello llamamos
	a la función shmget() pasándole la clave, el tamaño de memoria que
	queremos reservar (100 enteros en nuestro caso).
	
	Los flags son  los permisos de lectura/escritura/ejecucion 
	para propietario, grupo y otros (es el 777 en octal) y el 
	flag IPC_CREAT para indicar que cree la memoria.
	
	La función nos devuelve un identificador para la memoria recién
	creada.
	*/	 
	
		Id_Memoria = shmget (Clave, sizeof(int)*100, 0777 | IPC_CREAT);
		if (Id_Memoria == -1) {
			printf("%s\n", "No consigo Id para memoria compartida");
			return 0;
		}

	/*
	Una vez creada la memoria, hacemos que uno de nuestros punteros
	apunte a la zona de memoria recién creada. Para ello llamamos a
	shmat(), pasándole el identificador obtenido anteriormente.
	*/

		Memoria = (int *)shmat (Id_Memoria, (char *)0, 0);
		if (Memoria == NULL) {
			printf("%s\n", "No consigo memoria compartida");
			return 0;
		}

	/*
	Ya podemos utilizar la memoria.
	Escribimos en la memoria los números de 1 a 10 esperando
	un segundo entre ellos. Estos datos serán leidos por otro
	proceso (memoria_compartida_2.c).
	*/

		for (i = 0; i < 10; i++) {
			for (j = 0; j < 100; j++) {
				Memoria[j] = i;
			}
			printf("Escrito %d\n", i);
			sleep (1);
		}

	/*
	Terminada de usar la memoria compartida, la liberamos.
	*/

		shmdt ((char *)Memoria);
		shmctl (Id_Memoria, IPC_RMID, (struct shmid_ds *)NULL);
}
