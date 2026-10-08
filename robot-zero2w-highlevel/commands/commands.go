package commands

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"robot/uart"
	"strconv"
	"strings"
	"time"

	"go.bug.st/serial"
)

func runScript(filename string, port serial.Port) error {
	file, err := os.Open("scripts/" + filename)
	if err != nil {
		return fmt.Errorf("open %s: %w", filename, err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	lineNum := 0
	for scanner.Scan() {
		lineNum++
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}

		fields := strings.Fields(line)
		if fields[0] == "DELAY" {
			if len(fields) != 2 {
				return fmt.Errorf("line %d: DELAY needs one value", lineNum)
			}
			ms, err := strconv.Atoi(fields[1])
			if err != nil {
				return fmt.Errorf("line %d: bad delay %q", lineNum, fields[1])
			}
			time.Sleep(time.Duration(ms) * time.Millisecond)
			continue
		} else if fields[0] == "RUN" {
			if len(fields) < 2 {
				log.Printf("usage: COMMAND <file>")
				continue
			}
			times := 1
			if len(fields) > 2 {
				times, _ = strconv.Atoi(fields[2])
			}
			for range times {
				runScript(fields[1], port)
			}
		}

		uart.SendCommand(port, line+"\n")
	}
	return scanner.Err()
}

func StartCommandListener(port serial.Port, responseCh chan string) {
	go func() {
		scanner := bufio.NewScanner(os.Stdin)
		for scanner.Scan() {
			command := strings.TrimSpace(scanner.Text())
			if command == "" {
				continue
			}

			args := strings.Fields(command)
			if args[0] == "COMMAND" {
				if len(args) < 2 {
					log.Printf("usage: COMMAND <file>")
					continue
				}
				times := 1
				if len(args) > 2 {
					times, _ = strconv.Atoi(args[2])
				}

				for range times {
					if err := runScript(args[1], port); err != nil {
						log.Printf("script failed: %v", err)
					}
				}
				continue
			}

			uart.SendCommand(port, command+"\n")

			select {
			case result := <-responseCh:
				log.Printf("Command Result: %s", result)
			case <-time.After(5 * time.Second):
				log.Printf("Command Result Timed Out")
			}
		}
		if err := scanner.Err(); err != nil {
			log.Printf("scanner error: %v", err)
		}
	}()
}
