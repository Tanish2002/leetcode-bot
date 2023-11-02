package conf

import (
	"context"
	"log"
	"os"
	"strings"

	"github.com/Tanish2002/leetcode-bot/fetcher/models"
	"github.com/Tanish2002/leetcode-bot/fetcher/service"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/sqs"
)

type Configuration struct {
	Model   models.Model
	Service service.Service
	USERS   []string
}

func getSQSURL() string {
	url := os.Getenv("SQS_QUEUE_URL")
	if url == "" {
		log.Panic("SQS_QUEUE_URL env var missing")
	}
	return url
}

func getTableName() string {
	tablename := os.Getenv("DYNAMODB_TABLE_NAME")
	if tablename == "" {
		log.Panic("DYNAMODB_TABLE_NAME env var missing")
	}
	return tablename
}

func getUsers() []string {
	users := os.Getenv("USERS")
	if users == "" {
		log.Panic("USERS env var missing")
	}
	return strings.Split(users, ",")
}

func NewConfig() Configuration {
	cfg, err := config.LoadDefaultConfig(context.Background())
	if err != nil {
		log.Panic("Error loading AWS config:", err)
	}
	dynnamoClient := dynamodb.NewFromConfig(cfg)
	sqsClient := sqs.NewFromConfig(cfg)

	return Configuration{
		Model: models.Model{
			TIMESTAMP_TABLE_NAME: getTableName(),
			DYNAMODB_CLIENT:      dynnamoClient,
		},
		Service: service.Service{
			SQS_URL:    getSQSURL(),
			SQS_CLIENT: sqsClient,
		},
		USERS: getUsers(),
	}
}
