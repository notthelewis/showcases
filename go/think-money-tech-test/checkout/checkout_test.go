package checkout

import (
	"errors"
	"testing"
)

func TestScan(t *testing.T) {
	testCases := []struct {
		sku          rune
		scannedItem  ScannedItem
		scannedError error
	}{
		{
			sku: 'A',
			scannedItem: ScannedItem{
				quantityScanned: 1,
			},
			scannedError: nil,
		},
		{
			sku: 'A',
			scannedItem: ScannedItem{
				quantityScanned: 2,
			},
			scannedError: nil,
		},
		{
			sku:          '|',
			scannedError: ErrItemDoesntExist,
		},
	}

	scanner := New()

	for _, tc := range testCases {
		item, err := scanner.Scan(tc.sku)
		if item.quantityScanned != tc.scannedItem.quantityScanned {
			t.Fatalf(
				"quantity scanned does not match expected. Wanted: %v, got: %v",
				tc.scannedItem.quantityScanned,
				item.quantityScanned,
			)
		}

		if !errors.Is(tc.scannedError, err) {
			t.Fatalf(
				"received mismatch of expected error value. Wanted: %v, got: %v",
				tc.scannedError,
				err,
			)
		}
	}
}

func TestGetTotalPrice(t *testing.T) {
	testCases := []struct {
		skuScanList   []rune
		expectedTotal int
	}{
		{
			skuScanList:   []rune{'A', 'A'},
			expectedTotal: allItems['A'].UnitPrice * 2,
		},
		{
			skuScanList:   []rune{'A', 'A', 'A'},
			expectedTotal: allItems['A'].SpecialPrice.Price,
		},
		{
			skuScanList:   []rune{'A', 'A', 'A', 'A'},
			expectedTotal: allItems['A'].SpecialPrice.Price + allItems['A'].UnitPrice,
		},
		{
			skuScanList:   []rune{'A', 'A', 'A', 'A', 'A', 'A'},
			expectedTotal: allItems['A'].SpecialPrice.Price * 2,
		},
		{
			skuScanList:   []rune{'A', 'B', 'A', 'A'},
			expectedTotal: allItems['A'].SpecialPrice.Price + allItems['B'].UnitPrice,
		},
		{
			skuScanList:   []rune{'A', 'B', 'A', 'B', 'A'},
			expectedTotal: allItems['A'].SpecialPrice.Price + allItems['B'].SpecialPrice.Price,
		},
	}

	for _, tc := range testCases {
		scanner := New()

		for _, sku := range tc.skuScanList {
			_, err := scanner.Scan(sku)
			if err != nil {
				t.Fatal(err)
			}
		}

		total := scanner.GetTotalPrice()
		if total != tc.expectedTotal {
			t.Fatalf(
				"received invalid total for SKUs: %c. Expected: %v, received: %v",
				tc.skuScanList,
				tc.expectedTotal,
				total,
			)
		}
	}
}
