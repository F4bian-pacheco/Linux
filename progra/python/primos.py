


def es_primo(n:int) -> bool:
    for i in range(2,n):
        if n%i == 0:
            return False
    return True


num1 = int(input("num1: "))
num2 = int(input("num2: "))
lista = []

while num1 > num2:
    print("el num1 debe ser menor a num2")
    num1 = int(input("num1: "))
    num2 = int(input("num2: "))
for i in range(num1,num2):
    if es_primo(i):
        lista.append(i)
print(lista)


        
