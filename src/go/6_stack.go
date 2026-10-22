package main

import "fmt"

func GetRef() *int {
	var num int = 42
	// Рантайм языка Go использует подсчет ссылок, поэтому здесь произойдет
	// неявная аллокация на куче
	return &num
}

func Nested(ref_u32 *int) {
	fmt.Printf("Вложеный указатель: %d\n", *ref_u32)
}

func UseRef(ref_u32 *int) {
	fmt.Printf("Число по указателю: %d\n", *ref_u32)
	Nested(ref_u32)
}

func main() {
	var num int = 69
	UseRef(&num)

	var num_ref *int = GetRef()
	fmt.Printf("Число из указателя: %d\n", *num_ref)
}
