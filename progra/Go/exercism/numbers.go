package main

import "fmt"

func CalculateWorkingCarsPerHour(n_cars_h int, ratio float64) float64{
    //var ratio_dec float64 = ratio / 100
    ratio_dec := ratio / 100
    return float64(n_cars_h) * ratio_dec
}

func CalculateWorkingCarsPerMinute(n_cars_h int, ratio float64) int {
    cars_per_min := n_cars_h / 60
    ratio_dec := ratio / 100
    return int(float64(cars_per_min) * ratio_dec) 
}

func CalculateCost(num int) uint {
    f_digit := uint(num) / 10
    s_digit := uint(num) % 10

    return f_digit*95000 + s_digit*10000
}

func main(){
    rate := CalculateWorkingCarsPerHour(1547,90)
    fmt.Println("Numero de autos por hora: ",rate)
    rate_min := CalculateWorkingCarsPerMinute(1105, 90)
    fmt.Println("Numero de autos por minuto: ",rate_min)
    cost := CalculateCost(21)
    fmt.Println("Costo de produccion: ",cost)
}
