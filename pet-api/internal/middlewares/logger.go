package middlewares

import (
	"github.com/gofiber/fiber/v2"
	"log"
	"runtime"
	"strings"
)

func FunctionLogger() fiber.Handler {
	return func(c *fiber.Ctx) error {
		pc, _, _, _ := runtime.Caller(5)
		functionPath := runtime.FuncForPC(pc).Name()
		functionName := functionPath[strings.LastIndex(functionPath, ".")+1:]
		log.Println("Function called:", functionName)
		return c.Next()
	}
}
