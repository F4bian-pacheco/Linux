#include <stdio.h>
#include <sys/shm.h>
#include <unistd.h>

int main() {
	key_t Clave;
	int Id_Memoria;
	int *Memoria = NULL;
	int i,j;

	/*
	Igual que en memoria_compartida_1.c, obtenemos una clave para la memoria compartida
	*/

		Clave = ftok ("/bin/ls", 33);
		if (Clave == -1) {
			printf("%s\n", "No consigo clave para memoria compartida");
			return 0;
		}

	/*
	Igual que en memoria_compartida_1.c, obtenemos el id de la memoria. Al no poner
	el flag IPC_CREAT, estamos indicando que dicha memoria ya está creada.
	*/

		Id_Memoria = shmget (Clave, sizeof(int)*100, 0777);
		if (Id_Memoria == -1) {
			printf("%s\n", "No consigo Id para memoria compartida");
			return 0;
		}

	/*
	Igual que en memoria_compartida_1.c, obtenemos un puntero a la memoria compartida
	*/

		Memoria = (int *)shmat (Id_Memoria, (char *)0, 0);
		if (Memoria == NULL) {
			printf("%s\n", "No consigo memoria compartida");
			return 0;
		}

	/*
	Vamos leyendo el valor de la memoria con esperas de un segundo
	y mostramos en pantalla dicho valor. Debería ir cambiando según
	memoria_compartida_1.c lo va modificando.
	*/

		for (i=0; i<10; i++) {
			printf("Leido %d\n", Memoria[0]);
			sleep (1);
		}

	/*
	Desasociamos nuestro puntero de la memoria compartida. Suponemos
	que memoria_compartida_1.c (el proceso que la ha creado), la liberará.
	*/

		if (Id_Memoria != -1) {
			shmdt ((char *)Memoria);
		}
}
