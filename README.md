# line-bot-sdk

High-performance [LINE Messaging API](https://developers.line.biz/en/reference/messaging-api/) SDK for **Node.js / TypeScript**, powered by **Rust** native bindings via [napi-rs](https://napi.rs).

> **Disclaimer:** This is a community SDK and is **not** affiliated with or endorsed by LINE Corporation. For the official SDK, see [@line/bot-sdk](https://www.npmjs.com/package/@line/bot-sdk).

## Features

- **Native performance** — core HTTP client and crypto run in Rust
- **Full TypeScript support** — auto-generated types + ergonomic wrapper
- **Messaging** — push, reply, multicast, broadcast, quota, content download
- **Profile & followers** — user profile, follower IDs
- **Group & room** — member profiles, member IDs, leave group/room
- **Rich menu** — create, upload image, link/unlink, bulk operations
- **Insight** — message delivery overview, followers statistics
- **Webhook** — signature verification and typed event parsing

## Requirements

- Node.js `>= 18.17.0` (or 20+, 21+)
- [Rust](https://rustup.rs/) (for building from source)

## Installation

```bash
npm install @panudetingai/line-bot-sdk
# or
yarn add @panudetingai/line-bot-sdk
```

## Quick Start

### 1. Create a client

Get your **Channel Access Token** from [LINE Developers Console](https://developers.line.biz/console/).

```typescript
import { LineClient } from '@panudetingai/line-bot-sdk'

const client = new LineClient(process.env.LINE_CHANNEL_ACCESS_TOKEN!)

const bot = await client.getBotInfo()
console.log(bot.displayName)
```

### 2. Send messages

```typescript
await client.pushText('Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx', 'Hello from line-bot-sdk!')

await client.pushMessages('Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx', [
  { type: 'text', text: 'Hello!' },
  { type: 'sticker', packageId: '446', stickerId: '1988' },
])

await client.replyText(replyToken, 'Got your message!')
```

### 3. Get user profile

```typescript
const profile = await client.getProfile('Uxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx')
console.log(profile.displayName, profile.pictureUrl)
```

### 4. Webhook handler (Next.js / Express)

```typescript
import { verifySignature, parseWebhook, LineClient } from '@panudetingai/line-bot-sdk'

const CHANNEL_SECRET = process.env.LINE_CHANNEL_SECRET!
const client = new LineClient(process.env.LINE_CHANNEL_ACCESS_TOKEN!)

export async function POST(req: Request) {
  const rawBody = await req.text()
  const signature = req.headers.get('x-line-signature') ?? ''

  if (!verifySignature(rawBody, signature, CHANNEL_SECRET)) {
    return new Response('Invalid signature', { status: 401 })
  }

  const { events } = parseWebhook(rawBody)

  for (const event of events) {
    if (event.type === 'message' && event.message?.type === 'text') {
      await client.replyText(event.replyToken!, `You said: ${event.message.text}`)
    }
  }

  return new Response('OK')
}
```

## API Overview

### `LineClient`

| Category | Methods |
|----------|---------|
| **Bot** | `getBotInfo()` |
| **Messaging** | `pushText`, `pushMessages`, `replyText`, `replyMessages`, `multicast`, `broadcast` |
| **Quota** | `getMessageQuota()`, `getMessageQuotaConsumption()` |
| **Content** | `getMessageContent(messageId)` |
| **Profile** | `getProfile(userId)`, `getFollowerIds(next?)` |
| **Group** | `getGroupSummary`, `getGroupMemberProfile`, `getGroupMemberIds`, `leaveGroup` |
| **Room** | `getRoomMemberProfile`, `getRoomMemberIds`, `leaveRoom` |
| **Rich Menu** | `createRichMenu`, `uploadRichMenuImage`, `setDefaultRichMenu`, `linkRichMenuToUser`, ... |
| **Insight** | `getMessageDeliveryOverview(date)`, `getFollowersInsight(date)` |

### Webhook helpers

| Function | Description |
|----------|-------------|
| `verifySignature(body, signature, channelSecret)` | Validate `X-Line-Signature` header |
| `parseWebhook(body)` | Parse webhook JSON into typed `WebhookBody` |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `LINE_CHANNEL_ACCESS_TOKEN` | Channel access token for API calls |
| `LINE_CHANNEL_SECRET` | Channel secret for webhook signature verification |

Never commit tokens to git. Use `.env` locally and secrets in CI/production.

## Development

```bash
yarn install
yarn build:debug
yarn test

# Integration test with a real token
LINE_CHANNEL_ACCESS_TOKEN=xxx yarn test
```

### Project structure

```
src/
├── client.rs          # HTTP client & auth
├── api/endpoints.rs   # LINE API URL paths
├── services/          # API methods (messaging, profile, group, ...)
├── types/             # Request/response types
└── webhook.rs         # Signature verify & event parsing

typescript/
├── line-client.ts     # Ergonomic TypeScript wrapper
└── types.ts           # Shared TypeScript types
```

## Low-level API

Call the native binding directly if you prefer:

```typescript
import { LineClient } from './index'

const client = new LineClient(token)
await client.pushMessage('Uxxx', 'hello')
await client.getProfile('Uxxx')
```

The TypeScript wrapper in `typescript/line-client.ts` provides a cleaner API (objects instead of JSON strings).

## Platform Support

Prebuilt binaries for:

- macOS (x64, arm64)
- Linux (x64 gnu)
- Windows (x64)

Built with [napi-rs](https://napi.rs) — no `node-gyp` or C++ toolchain required for end users.

## Related Projects

- [@line/bot-sdk](https://github.com/line/line-bot-sdk-nodejs) — Official LINE Bot SDK for Node.js
- [line-bot-sdk-rust](https://crates.io/crates/line-bot-sdk-rust) — LINE SDK for pure Rust applications

## License

MIT © [Panudetingai](https://github.com/Panudetingai)
