package main

import "fmt"

const Oventime = 40

func RemainingOvenTime(actual int) int {
    return Oventime - actual
}

func PreparationTime(numCapas int) int {
    return 2*numCapas
}

func ElapsedTime(numCapas,actualmins int) int {
    total := PreparationTime(numCapas) + actualmins
    return total
}

func main(){
    fmt.Print("####Ex_1####\n")
    fmt.Println(Oventime)

    fmt.Print("####Ex_2####\n")
    actual := 30
    fmt.Println(RemainingOvenTime(actual))

    fmt.Print("####Ex_3####\n") 
    var capas int
    capas = 3
    fmt.Println(PreparationTime(capas))

    fmt.Print("####Ex_4####\n") 
    fmt.Println(ElapsedTime(capas,20))

}
