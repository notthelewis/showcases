package checkout

import (
	"errors"
)

type ScannedItem struct {
	quantityScanned int
}

type Scanner struct {
	scannedItems map[rune]ScannedItem
}

func New() Scanner {
	return Scanner{
		scannedItems: make(map[rune]ScannedItem),
	}
}

var (
	ErrItemDoesntExist = errors.New("Item SKU does not exist in system")
	emptyScannedItem   = ScannedItem{}
)

func (s *Scanner) Scan(SKU rune) (ScannedItem, error) {
	if _, itemExists := allItems[SKU]; !itemExists {
		return emptyScannedItem, ErrItemDoesntExist
	}

	if scannedItem, scannedItemExists := s.scannedItems[SKU]; scannedItemExists {
		scannedItem.quantityScanned++
		s.scannedItems[SKU] = scannedItem
		return scannedItem, nil
	} else {
		toReturn := ScannedItem{
			quantityScanned: 1,
		}
		s.scannedItems[SKU] = toReturn
		return toReturn, nil
	}
}

func (s Scanner) GetTotalPrice() int {
	total := 0

	for key, scannedItem := range s.scannedItems {
		item := allItems[key]

		if item.SpecialPrice.Quantity > scannedItem.quantityScanned {
			total += scannedItem.quantityScanned * item.UnitPrice
			continue
		}

		if item.SpecialPrice.Quantity == scannedItem.quantityScanned {
			total += item.SpecialPrice.Price
			continue
		}

		if item.SpecialPrice.Quantity < scannedItem.quantityScanned {
			// Calculate the total price for items up to the special price quantity
			total += (scannedItem.quantityScanned / item.SpecialPrice.Quantity) * item.SpecialPrice.Price

			// Calculate the total price for the remaining items at the regular price
			remainingQuantity := scannedItem.quantityScanned % item.SpecialPrice.Quantity
			total += remainingQuantity * item.UnitPrice
		}
	}

	return total
}
