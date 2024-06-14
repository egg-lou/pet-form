package owners

import (
	"github.com/gofiber/fiber/v2"
	"github.com/google/uuid"
	"pet-api/internal/database"
	"pet-api/internal/queries"
)

var db database.Service
var oq = queries.NewOwnerQueries(db)

func OwnerRoutes(app *fiber.App) {
	ownerRouter := app.Group("/owners")

	ownerRouter.Get("/", getAllOwners)
	ownerRouter.Get("/:id", getOwnerById)
	ownerRouter.Post("/", createOwner)
	ownerRouter.Put("/:id", updateOwner)
	ownerRouter.Delete("/:id", deleteOwner)
}

func getAllOwners(c *fiber.Ctx) error {
	owners, err := oq.GetAll()
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to get owners",
		})
	}
	return c.JSON(owners)
}

func getOwnerById(c *fiber.Ctx) error {
	id, err := uuid.Parse(c.Params("id"))
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Invalid ID",
		})
	}

	owner, err := oq.GetById(id)
	if err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to get owner",
		})
	}

	return c.JSON(owner)
}

func createOwner(c *fiber.Ctx) error {
	var owner queries.Owner
	if err := c.BodyParser(&owner); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Cannot parse JSON",
		})
	}

	if err := oq.Create(owner); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to create owner",
		})
	}

	return c.JSON(owner)
}

func updateOwner(c *fiber.Ctx) error {
	var owner queries.Owner
	if err := c.BodyParser(&owner); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Cannot parse JSON",
		})
	}
	if err := oq.Update(owner); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to update owner",
		})
	}
	return c.JSON(owner)
}

func deleteOwner(c *fiber.Ctx) error {
	id, err := uuid.Parse(c.Params("id"))
	if err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "Invalid ID",
		})
	}
	if err := oq.Delete(id); err != nil {
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "Failed to delete owner",
		})
	}
	return c.SendStatus(fiber.StatusNoContent)
}
