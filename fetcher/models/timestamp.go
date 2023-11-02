package models

import (
	"context"
	"fmt"

	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)

type Model struct {
	TIMESTAMP_TABLE_NAME string
	DYNAMODB_CLIENT      *dynamodb.Client
}

func (m *Model) AddOrUpdateTimestamp(user, timestamp string) error {
	_, err := m.DYNAMODB_CLIENT.PutItem(context.TODO(), &dynamodb.PutItemInput{
		TableName: aws.String(m.TIMESTAMP_TABLE_NAME),
		Item: map[string]types.AttributeValue{
			"User":      &types.AttributeValueMemberS{Value: user},
			"Timestamp": &types.AttributeValueMemberS{Value: timestamp},
		},
	})
	return err
}

func (m *Model) GetLatestTimestamp(user string) (string, error) {
	resp, err := m.DYNAMODB_CLIENT.GetItem(context.TODO(), &dynamodb.GetItemInput{
		TableName: aws.String(m.TIMESTAMP_TABLE_NAME),
		Key: map[string]types.AttributeValue{
			"User": &types.AttributeValueMemberS{Value: user},
		},
	})
	if err != nil {
		return "", err
	}

	if resp.Item != nil {
		latestTimestamp := resp.Item["Timestamp"].(*types.AttributeValueMemberS).Value
		return latestTimestamp, nil
	}

	return "", fmt.Errorf("Timestamp not found")
}
