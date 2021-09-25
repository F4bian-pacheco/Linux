#include <stdio.h>
#include <unistd.h>

int main() {
	int i;

	for(i = 0; i < 3; i++)
		if(fork() == 0)
			break;

	printf("Soy un proceso\n");
	while(1);
}
