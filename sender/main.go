package main

import (
	"github.com/Tanish2002/leetcode-bot/sender/conf"
	"github.com/aws/aws-lambda-go/lambda"
)

func main() {
	newConfig := conf.NewConfig()
	lambda.Start(newConfig.Handler.HandlerFunc)
}
