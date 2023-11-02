package handlers

import (
	"context"
	"fmt"
	"log"

	"github.com/Tanish2002/leetcode-bot/sender/services"
	"github.com/Tanish2002/leetcode-bot/sender/utils"
	"github.com/aws/aws-lambda-go/events"
)

type Handler struct {
	Service services.Service
}

func (h *Handler) HandlerFunc(ctx context.Context, sqsEvent events.SQSEvent) error {
	for _, message := range sqsEvent.Records {
		fmt.Printf("The message %s for event source %s = %s \n", message.MessageId, message.EventSource, message.Body)
		sqsMessage, err := utils.UnmarshalSQSMessage(message.Body)
		if err != nil {
			log.Println("Error while Unmarshalling SQS Message. Error: ", err)
			continue
		}
		fmt.Println(sqsMessage)
		if err := h.Service.SendEmbedToDiscord(sqsMessage); err != nil {
			log.Printf("Error while sending Embed to discord for user %s. Error: %v", sqsMessage.Username, err)
			continue
		}
	}

	return nil
}
