

OBJS = texto.o serie.o temporada.o capitulo.o subtitulo.o actor.o director.o guionista.o productor.o persona.o usuario.o critica.o


all:programa

programa: $(OBJS)
	g++ -o programa $(OBJS)

serie.o: serie.h serie.cc
temporada: temporada.h temporada.cc
capitulo.o: capitulo.h capitulo.cc
subtitulo.o: subtitulo.h subtitulo.cc
actor.o: actor.h actor.cc
director.o: director.h director.cc
guionista.o: guionista.h guionista.cc
productor.o: productor.h productor.cc
persona.o: persona.h persona.cc
usuario.o: usuario.h usuario.cc
critica.o: critica.h critica.cc

clean:
	rm *.o programa