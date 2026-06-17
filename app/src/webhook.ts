import type { Request, Response } from 'express'
import type { LineClient, WebhookEvent } from '../../index'
import { parseWebhookBody, verifyWebhookSignature } from '../../index'
import { env } from './env.js'

async function handleEvent(client: LineClient, event: WebhookEvent) {
  if (event.eventType === 'message' && event.message?.messageType === 'text' && event.replyToken) {
    const text = event.message?.text
    await client.replyMessage(event.replyToken, `Echo: ${text}`)
    return
  }

  if (event.eventType === 'follow' && event.replyToken) {
    await client.replyMessage(event.replyToken, 'Thanks for adding me! Send any text and I will echo it back.')
    return
  }

  if (event.eventType === 'postback' && event.replyToken) {
    const data = event.postback?.data ?? ''
    await client.replyMessage(event.replyToken, `Postback received: ${data}`)
  }
}

export function createWebhookHandler(client: LineClient) {
  return async (req: Request, res: Response) => {
    const signature = req.headers['x-line-signature']
    if (typeof signature !== 'string') {
      res.status(401).json({ error: 'Missing X-Line-Signature header' })
      return
    }

    const rawBody = req.body instanceof Buffer ? req.body.toString('utf8') : String(req.body)

    if (!env.channelSecret) {
      res.status(500).json({ error: 'LINE_CHANNEL_SECRET is not configured' })
      return
    }

    if (!verifyWebhookSignature({ body: rawBody, signature, channelSecret: env.channelSecret })) {
      res.status(401).json({ error: 'Invalid signature' })
      return
    }

    const { events } = parseWebhookBody(rawBody)
    console.log('Event Webhook Received: ', events)

    for (const event of events) {
      if (event.eventType === 'message' && event.message?.messageType === 'text') {
        console.log('Text Message Received: ', event.message?.text)
      }
    }

    await Promise.all(
      events.map((event: WebhookEvent) =>
        handleEvent(client, event).catch((err) => {
          console.error('[webhook] event handler error:', err)
        }),
      ),
    )

    res.status(200).json({ ok: true })
  }
}
