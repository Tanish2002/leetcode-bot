provider "aws" {
  region = "ap-south-1" 
}

# DynamoDB Table
resource "aws_dynamodb_table" "timestamp_table" {
  name         = "timestamp_table"
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "User"

  attribute {
    name = "User"
    type = "S"
  }

}

# SQS Queue
resource "aws_sqs_queue" "leetcode_bot_queue" {
  name = "lambda-queue"
}

# IAM Role for Lambda Functions
resource "aws_iam_role" "lambda_role" {
  name = "lambda_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = "sts:AssumeRole",
        Effect = "Allow",
        Principal = {
          Service = "lambda.amazonaws.com"
        },
      },
    ],
  })
}

# IAM Policy to allow Lambda to access DynamoDB, SQS and CloudWatch Logs
resource "aws_iam_policy" "lambda_policy" {
  name   = "lambda_policy"
  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Action = [
          "dynamodb:UpdateItem",
          "dynamodb:GetItem",
          "dynamodb:PutItem"
        ],
        Effect   = "Allow",
        Resource = aws_dynamodb_table.timestamp_table.arn
      },
      {
        Action = [
          "sqs:SendMessage",
          "sqs:ReceiveMessage",
          "sqs:DeleteMessage",
          "sqs:GetQueueAttributes"
        ],
        Effect   = "Allow",
        Resource = aws_sqs_queue.leetcode_bot_queue.arn
      },
      {
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents"
        ],
        Effect   = "Allow",
        Resource = "arn:aws:logs:*:*:*"
      },
    ],
  })
}

# Attach the policy to the role
resource "aws_iam_role_policy_attachment" "lambda_attach" {
  role       = aws_iam_role.lambda_role.name
  policy_arn = aws_iam_policy.lambda_policy.arn
}


// build the fetcher binary and archive it 
resource "null_resource" "fetcher_binary" {
  provisioner "local-exec" {
    command = "cd ../fetcher && cargo lambda build --release --output-format zip"
  }
}

# Lambda Function for Fetching Data
resource "aws_lambda_function" "fetcher_lambda" {
  depends_on = [null_resource.fetcher_binary]

  filename         = "${path.module}/../fetcher/target/lambda/fetcher/bootstrap.zip"
  function_name    = "leetcode_bot_fetcher"
  role             = aws_iam_role.lambda_role.arn
  handler          = "bootstrap"
  runtime          = "provided.al2"

  environment {
    variables = {
      DYNAMODB_TABLE_NAME = aws_dynamodb_table.timestamp_table.name
      SQS_QUEUE_URL       = aws_sqs_queue.leetcode_bot_queue.url
      USERS = var.users
    }
  }
}

// build the sender binary and archive it 
resource "null_resource" "sender_binary" {
  provisioner "local-exec" {
    command = "cd ../sender && cargo lambda build --release --output-format zip"
  }
}

# Lambda Function for Sending Data to Discord
resource "aws_lambda_function" "sender_lambda" {
  depends_on = [null_resource.sender_binary]

  filename         = "${path.module}/../sender/target/lambda/sender/bootstrap.zip"
  function_name    = "leetcode_bot_sender"
  role             = aws_iam_role.lambda_role.arn
  handler          = "bootstrap" # the file name without extension of the compiled Go binary inside the zip
  runtime          = "provided.al2"

  environment {
    variables = {
      DISCORD_WEBHOOK_URL = var.discord_webhook_url
    }
  }
}

# EventBridge Rule to Trigger Fetcher Lambda on a Schedule
resource "aws_cloudwatch_event_rule" "every_hour" {
  name                = "every-hour"
  description         = "Trigger every hour"
  schedule_expression = "rate(1 hour)"
}

# Permission for EventBridge to invoke Lambda
resource "aws_lambda_permission" "allow_cloudwatch_to_call_fetcher" {
  statement_id  = "AllowExecutionFromCloudWatch"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.fetcher_lambda.function_name
  principal     = "events.amazonaws.com"
  source_arn    = aws_cloudwatch_event_rule.every_hour.arn
}

# Target to associate the rule with the fetcher Lambda
resource "aws_cloudwatch_event_target" "invoke_fetcher" {
  rule      = aws_cloudwatch_event_rule.every_hour.name
  target_id = "fetcherLambdaTarget"
  arn       = aws_lambda_function.fetcher_lambda.arn
}

# SQS Event Source Mapping for Sender Lambda
resource "aws_lambda_event_source_mapping" "sqs_sender_mapping" {
  event_source_arn = aws_sqs_queue.leetcode_bot_queue.arn
  function_name    = aws_lambda_function.sender_lambda.arn
}


# Outputs to verify resources creation
output "fetcher_lambda_arn" {
  value = aws_lambda_function.fetcher_lambda.arn
}

output "sender_lambda_arn" {
  value = aws_lambda_function.sender_lambda.arn
}

output "dynamodb_table_name" {
  value = aws_dynamodb_table.timestamp_table.name
}

output "sqs_queue_url" {
  value = aws_sqs_queue.leetcode_bot_queue.url
}
