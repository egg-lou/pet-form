package utils

import (
	"fmt"
	"pet-api/internal/models"
	"reflect"
	"strings"
)

func ValidateOwner(owner *models.Owner) error {
	v := reflect.ValueOf(*owner)
	missingFields := []string{}

	for i := 0; i < v.NumField(); i++ {
		field := v.Field(i)
		if field.Kind() == reflect.String && field.String() == "" {
			missingFields = append(missingFields, strings.ToLower(v.Type().Field(i).Name))
		}
	}

	if len(missingFields) > 0 {
		return fmt.Errorf("the following fields must be filled: %s", strings.Join(missingFields, ", "))
	}

	return nil
}
