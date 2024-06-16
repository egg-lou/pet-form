package server

import (
	"github.com/gofiber/fiber/v2"

	"pet-api/internal/database"
)

type FiberServer struct {
	*fiber.App

	db database.Service
}

func New() *FiberServer {
	server := &FiberServer{
		App: fiber.New(fiber.Config{
			ServerHeader: "pet-api",
			AppName:      "pet-api",
		}),

		db: database.New(),
	}

	return server
}
