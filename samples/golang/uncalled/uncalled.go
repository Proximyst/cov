package uncalled

import "fmt"

func PackageNeverCalled() {
	fmt.Println("this entire package is never called.")
}
