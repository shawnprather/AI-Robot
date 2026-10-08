package main

import (
	"fmt"
	"robot/camera"
	"robot/commands"
	"robot/sensors"
	"robot/uart"
)

func main() {
	fmt.Println("Starting Up")

	port := uart.Open()
	sensorCh, responseCh := uart.StartRouter(port)
	camera.StartCapture()
	commands.StartCommandListener(port, responseCh)
	sensors.StartSensorListener(sensorCh)

	select {}
}
