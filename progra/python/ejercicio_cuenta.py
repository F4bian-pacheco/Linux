
class Cuenta:
    
    balance = 0
    def __init__(self,num_cuenta,nombre_titular,saldo_inicial,tipo_cuenta):
        self.num_cuenta = num_cuenta
        self.nombre_titular = nombre_titular
        self.saldo = saldo_inicial
        self.tipo_cuenta = tipo_cuenta

    def depositar(self, monto):
        self.saldo += monto
        self.balance += monto

    def retirar(self, monto):
        if monto > self.saldo:
            raise Exception("saldo insuficiente, retire un monto menor")
        self.saldo -= monto
        self.balance += monto
    def __str__(self):
        return f"cuenta de {self.tipo_cuenta} numero {self.num_cuenta} con titular {self.nombre_titular}, balance total: {self.balance}"


cuenta1 = Cuenta("12345","pepito",10,"ahorro")
cuenta1.depositar(20)
cuenta1.retirar(5)
print(cuenta1)
