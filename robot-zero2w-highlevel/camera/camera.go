package camera

import (
	"fmt"
	"log"
	"os/exec"
	"time"
)

func takePhoto(filename string) {
	cmd := exec.Command("rpicam-still", "-o", filename, "--nopreview", "-t", "1")
	err := cmd.Run()
	if err != nil {
		log.Printf("camera error: %v", err)
	}
}

func StartCapture() {
	// Camera
	go func() {
		ticker := time.NewTicker(time.Second * 10)
		timestamp := time.Now().Format("2006-01-02_15-04-05")
		filename := fmt.Sprintf("/home/robot/photos/photo_%s.jpg", timestamp)
		for {
			<-ticker.C
			takePhoto(filename)
		}
	}()
}
