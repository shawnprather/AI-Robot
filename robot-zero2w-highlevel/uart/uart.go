package uart

import (
	"log"
	"strings"
	"time"

	"go.bug.st/serial"
)

func Open() serial.Port {
	mode := &serial.Mode{BaudRate: 115_200}
	port, err := serial.Open("/dev/serial0", mode)
	if err != nil {
		log.Fatal(err)
	}
	return port
}

func StartRouter(port serial.Port) (sensorCh chan string, responseCh chan string) {
	responseCh = make(chan string)
	sensorCh = make(chan string, 100)

	go func() {
		for {
			msg := readBuffer(port)
			if strings.HasPrefix(msg, "OK") || strings.HasPrefix(msg, "ERR") {
				responseCh <- msg
			} else {
				sensorCh <- msg
			}
		}
	}()

	return
}

func readBuffer(port serial.Port) string {
	var line strings.Builder
	buf := make([]byte, 1)

	for {
		n, err := port.Read(buf)
		if err != nil || n == 0 {
			log.Printf("read err: %v", err)
			time.Sleep(10 * time.Millisecond)
			continue
		}

		char := string(buf)
		line.WriteString(char)

		if char == "\n" {
			return line.String()
		}
	}
}

func SendCommand(port serial.Port, command string) {
	n, err := port.Write([]byte(command))
	if err != nil || n == 0 {
		log.Printf("write err: %v", err)
	}
}
