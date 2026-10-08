package sensors

import (
	"log"
)

func StartSensorListener(sensorCh chan string) {
	go func() {
		for {
			sensorData := <-sensorCh
			log.Printf("Sensor Data: %s", sensorData)
		}
	}()
}
