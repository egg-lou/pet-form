package server

import (
	"github.com/gofiber/fiber/v2"
	"pet-api/internal/handlers"
)

func (s *FiberServer) RegisterFiberRoutes() {
	s.App.Get("/", s.IndexHandler)

	s.App.Get("/health", s.healthHandler)

	ownerHandler := handlers.NewOwnerHandler(s.db)

	ownerHandler.SetupRoutes(s.App.Group("/api/"))

}

func (s *FiberServer) IndexHandler(c *fiber.Ctx) error {
	resp := fiber.Map{
		"message": "Pet Pawrm is running...",
	}

	return c.JSON(resp)
}

func (s *FiberServer) healthHandler(c *fiber.Ctx) error {
	return c.JSON(s.db.Health())
}
