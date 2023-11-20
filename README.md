# Leetcode-Bot 🚀

![Rust](https://img.shields.io/badge/rust-1.56%2B-orange.svg) ![AWS Lambda](https://img.shields.io/badge/AWS-Lambda-orange.svg) ![AWS SQS](https://img.shields.io/badge/AWS-SQS-blue.svg) ![AWS DynamoDB](https://img.shields.io/badge/AWS-DynamoDB-green.svg)  
Welcome to Leetcode-Bot! This project is a handy tool for fetching and sharing your latest Leetcode submissions on a Discord channel. Whether you're a coding enthusiast or just love to showcase your problem-solving skills, Leetcode-Bot has got you covered!

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Setup](#setup)
- [Project Structure](#project-structure)
- [Environment Variables](#environment-variables)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Overview

Leetcode-Bot consists of two Lambda functions: **Fetcher** and **Sender**. The Fetcher regularly checks Leetcode APIs for new submissions by the specified users, sending them to an SQS queue. It also stores the timestamp of the last sent submissions in a DynamoDB table. The Sender consumes submissions from the SQS queue and sends them to a Discord channel using a webhook.

## Getting Started

Setting up Leetcode-Bot is a breeze! Just follow these simple steps:

### Prerequisites

Make sure you have the following installed:

- Rust
- Cargo-lambda
- Terraform and AWS CLI v2 (If you are going to use it.)
- Nix (If you want to use [flake.nix](flake.nix) for devshell.)

### Setup

1. Clone this repository: `git clone https://github.com/Tanish2002/leetcode-bot.git`
2. Navigate to the project folder: `cd leetcode-bot`
3. Set the required environment variables (see [Environment Variables](#environment-variables)).
4. Inside the `infrastructure` folder, run `terraform init && terraform apply` to set up the AWS infrastructure.

## Project Structure

```
├── fetcher
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── configuration.rs
│       ├── main.rs
│       ├── models.rs
│       └── service
│           ├── leetcode
│           │   ├── mod.rs
│           │   ├── submissions.rs
│           │   └── user.rs
│           ├── mod.rs
│           └── sqs.rs
├── flake.lock
├── flake.nix
├── infrastructure
│   ├── main.tf
│   ├── terraform.tfstate
│   ├── terraform.tfstate.backup
│   └── variables.tf
└── sender
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── configuration.rs
        ├── handler.rs
        ├── main.rs
        └── service.rs
```

## Environment Variables

### Manual Deployment

#### For Fetcher

- `USERS`: Leetcode usernames separated by commas.
- `DYNAMODB_TABLE_NAME`: DynamoDB table name.
- `SQS_QUEUE_URL`: URL of the SQS queue.

#### For Sender

- `DISCORD_WEBHOOK_URL`: Discord channel webhook URL.

### If Using Terraform

- `TF_VAR_users`: Leetcode usernames separated by commas.
- `TF_VAR_discord_webhook_url`: Discord channel webhook URL.

## Contributing

We welcome contributions! Feel free to submit issues or pull requests.
