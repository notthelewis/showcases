package main

import (
	"bufio"
	"strconv"
)

func runLoopInitial(scanner *bufio.Scanner) map[string]StationResult {
    scannerTokBuf := make([]byte, MAX_LINE_LENGTH)

    // This split function emits each field of the CSV file as a separate token.
    // i.e. the station name is emitted, then the value is emitted, then the next station name etc
    scanner.Split(func(data []byte, atEOF bool) (advance int, token []byte, err error) {
        for byte_pos := 0; byte_pos < len(data); byte_pos++ {
            switch data[byte_pos] {
                case CSV_DELIM:
                    return byte_pos+1, scannerTokBuf[:byte_pos], nil
                case LINE_BREAK:
                    return byte_pos+1, scannerTokBuf[:byte_pos], nil
                default:
                    scannerTokBuf[byte_pos] = data[byte_pos]
           }
        }
        
        if !atEOF {
            return 0, nil, nil
        }

        return 0, nil, bufio.ErrFinalToken
    })

    isReadingName := true
    currentName := ""

    measurements := make(map[string][]float32)
    uniqueStationNumber := 0

    for scanner.Scan() {
        if isReadingName {
            currentName = scanner.Text()
        } else {
            val, err := strconv.ParseFloat(scanner.Text(), 32)
            // data should always be valid float, so we blow up if not
            if err != nil {
                panic(err)
            }

            val32 := float32(val)

            if m, ok := measurements[currentName]; !ok {
                measurements[currentName] = append(make([]float32, 0, 5), val32)
                uniqueStationNumber++
            } else {
                measurements[currentName] = append(m, val32)
            }
        }

        isReadingName = !isReadingName
    }

    endResult := make(map[string]StationResult, uniqueStationNumber)

    for station, measurement := range measurements {
        var min, max, total, mean float32

        min = measurement[0]
        for _, entry := range measurement {
            total += entry
            if entry < min {
                min = entry
            } 
            if entry > max {
                max = entry
            } 
        }

        mean = total / float32(len(measurement))

        endResult[station] = StationResult{
            min,
            max,
            mean,
        }
    }

    return endResult
}
