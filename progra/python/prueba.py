


def saludar(nombre="a todos"):
    return f"hola mundo y hola {nombre}"

def llamada_global():
    print(globals()["saludar"]("fabian"))
llamada_global()
#print(locals()["saludar"]("fabian"))
