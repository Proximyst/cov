package sample

import (
	"sync"
	"testing"
	"time"
)

func TestUncovered(t *testing.T) {
	t.Skip("this test is skipped")
	EntirelyUncovered(time.Now)
}

func TestCommonlyCovered(t *testing.T) {
	CommonlyCovered(time.Now)
}

func TestGoroutineCovered(t *testing.T) {
	wg := &sync.WaitGroup{}
	wg.Add(100)
	for range 100 {
		go func() {
			defer wg.Done()
			GoroutineCovered(time.Now)
		}()
	}
	wg.Wait()
}

func TestUnitTested(t *testing.T) {
	if !testing.Short() {
		t.Skip("skipping unit test")
	}
	UnitTestedFunction(time.Now, "World")
}

func TestIntegrationTested(t *testing.T) {
	if testing.Short() {
		t.Skip("skipping integration test")
	}
	IntegrationTestedFunction(time.Now, "Cap'n Jack")
}
