package sample

import (
	"fmt"
	"time"
)

func EntirelyUncovered(clock func() time.Time) {
	now := clock()

	fmt.Println("this is uncovered code. called at", now)
}

func CommonlyCovered(clock func() time.Time) {
	now := clock()

	fmt.Println("this is covered code. called at", now)
}

func UnitTestedFunction(clock func() time.Time, name string) {
	greeting := fmt.Sprintf("Hello, %s!", name)
	now := clock()

	fmt.Println(greeting, now)
}

func IntegrationTestedFunction(clock func() time.Time, name string) {
	greeting := fmt.Sprintf("Aye, %s!", name)
	now := clock()

	fmt.Println(greeting, now)
}

func GoroutineCovered(clock func() time.Time) {
	now := clock()

	fmt.Println("this is covered code. called at", now)
}
