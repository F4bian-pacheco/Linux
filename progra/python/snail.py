

matrix = [[1,2,3],
          [8,9,4,],
          [7,6,5]]


def recorrido_snail(matrix):
    res = []
    if len(matrix) == 0:
        return res
    
    fila_i,col_i  = 0,0
    fila_f,col_f = len(matrix) - 1, len(matrix[0]) - 1


    while fila_i <= fila_f and col_i <= col_f:
        for i in range(col_i,col_f+1):
            res.append(matrix[fila_i][i])
        fila_i += 1

        for i in range(fila_i,fila_f+1):
            res.append(matrix[i][col_f])
        col_f -= 1
        
        if fila_i <= fila_f:
            for i in range(col_f,col_i-1,-1):
                res.append(matrix[fila_f][i])
        fila_f -= 1

        if col_i <= col_f:
            for i in range(fila_f,fila_i-1,-1):
                res.append(matrix[i][col_i])
        col_i +=1

    return res

print(recorrido_snail(matrix))
