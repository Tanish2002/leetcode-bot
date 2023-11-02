package conf

import (
	"log"
	"os"

	"github.com/Tanish2002/leetcode-bot/sender/handlers"
	"github.com/Tanish2002/leetcode-bot/sender/services"
	"github.com/diamondburned/arikawa/v3/api/webhook"
)

type Configuration struct {
	Handler handlers.Handler
}

func getWebhookURL() string {
	url := os.Getenv("DISCORD_WEBHOOK_URL")
	if url == "" {
		log.Panic("DISCORD_WEBHOOK_URL env var missing")
	}
	return url
}

func newWebhookClient() *webhook.Client {
	client, err := webhook.NewFromURL(getWebhookURL())
	if err != nil {
		log.Panicf("Error while creating webhook client. Error: %v", err)
	}
	return client
}

func NewConfig() Configuration {
	return Configuration{
		Handler: handlers.Handler{
			Service: services.Service{
				Client: newWebhookClient(),
			},
		},
	}
}
