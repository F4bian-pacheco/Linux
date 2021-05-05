import numpy as np


filas = int(input("filas: "))
col = int(input("columnas: "))

array = np.full((filas,col),0)
#array[1:3] = 1
#array[1:3][3:] = 0
array[:3,:3] = 1
print(array)
print(array[array==1])

