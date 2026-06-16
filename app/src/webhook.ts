import type { Request, Response } from 'express'
import type { LineClient } from '../../typescript/line-client'
import { parseWebhook, verifySignature } from '../../typescript/line-client'
import { env } from './env.js'

type ParsedEvent = {
  eventType?: string
  type?: string
  replyToken?: string
  message?: { messageType?: string; type?: string; text?: string }
  postback?: { data: string }
}

async function handleEvent(client: LineClient, event: ParsedEvent) {
  const type = event.eventType ?? event.type
  const messageType = event.message?.messageType ?? event.message?.type

  if (type === 'message' && messageType === 'text' && event.replyToken) {
    const text = event.message?.text ?? ''
    await client.replyText(event.replyToken, `Echo: ${text}`)
    return
  }

  if (type === 'follow' && event.replyToken) {
    await client.replyText(event.replyToken, 'Thanks for adding me! Send any text and I will echo it back.')
    return
  }

  if (type === 'postback' && event.replyToken) {
    const data = event.postback?.data ?? ''
    await client.replyText(event.replyToken, `Postback received: ${data}`)
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

    if (!verifySignature({ body: rawBody, signature, channelSecret: env.channelSecret })) {
      res.status(401).json({ error: 'Invalid signature' })
      return
    }

    const { events } = parseWebhook(rawBody)
    console.log('Event Webhook Received: ', events)

    for (const event of events) {
      if (event.type === 'message' && event.message.type === 'text') {
        console.log('Text Message Received: ', event.message.text)
      }
    }

    await Promise.all(
      events.map((event) =>
        handleEvent(client, event as ParsedEvent).catch((err) => {
          console.error('[webhook] event handler error:', err)
        }),
      ),
    )

    res.status(200).json({ ok: true })
  }
}
