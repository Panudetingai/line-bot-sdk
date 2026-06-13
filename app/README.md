# line-bot-sdk test app

Express server for testing [line-bot-sdk-rs](../README.md) in a real webhook + API flow.

## Setup

```bash
# 1. Build the native addon from repo root
cd ..
yarn build:debug

# 2. Install app dependencies
cd app
cp .env.example .env
# Edit .env — set LINE_CHANNEL_ACCESS_TOKEN and LINE_CHANNEL_SECRET

yarn install
```

## Run

```bash
yarn dev
```

Server starts at `http://localhost:3000`.

## Endpoints

| Method | Path                   | Description                                  |
| ------ | ---------------------- | -------------------------------------------- |
| GET    | `/`                    | API overview                                 |
| GET    | `/health`              | Health check                                 |
| GET    | `/api/bot`             | Get bot info                                 |
| GET    | `/api/profile/:userId` | Get user profile                             |
| GET    | `/api/quota`           | Message quota & consumption                  |
| POST   | `/webhook`             | LINE webhook (verify signature + auto reply) |

## Webhook testing locally

LINE requires HTTPS. Use [ngrok](https://ngrok.com/):

```bash
ngrok http 3000
```

Copy the HTTPS URL + `/webhook` into LINE Developers Console → Messaging API → Webhook URL.

## Bot behavior

- **Text message** → replies with `Echo: <your text>`
- **Follow** → welcome message
- **Postback** → replies with postback data
