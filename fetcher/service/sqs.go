package service

import (
	"context"
	"encoding/json"
	"log"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/service/sqs"
)

type Service struct {
	SQS_URL    string
	SQS_CLIENT *sqs.Client
}

type SQSMessage struct {
	Username    string
	Submissions *RecentAcSubmissionResp
}

func (s *Service) SendToSQS(data SQSMessage) error {
	messageBody, err := json.Marshal(data)
	if err != nil {
		log.Println("Error while marshalling sqsMessage: ", err)
		return err
	}

	_, err = s.SQS_CLIENT.SendMessage(context.Background(), &sqs.SendMessageInput{
		QueueUrl:    aws.String(s.SQS_URL),
		MessageBody: aws.String(string(messageBody)),
	})
	if err != nil {
		log.Println("Error sending message to SQS:", err)
		return err
	}

	log.Println("Message sent successfully!")
	return nil
}
