import test from 'ava'
import { createHmac } from 'node:crypto'
import { LineClient, parseWebhookBody, verifyWebhookSignature } from '../index'

test('reject empty access token', (t) => {
  t.throws(() => new LineClient({ accessToken: '' }), { message: /access_token is required/ })
})

test('verifyWebhookSignature returns true for valid signature', (t) => {
  const body = '{"destination":"Uxxxxxxxx","events":[]}'
  const channelSecret = 'my-channel-secret'
  const signature = createHmac('sha256', channelSecret).update(body).digest('base64')

  t.true(verifyWebhookSignature({ body, signature, channelSecret }))
})

test('parseWebhookBody parses valid JSON', (t) => {
  const body = JSON.stringify({
    destination: 'Uxxxxxxxx',
    events: [
      {
        type: 'message',
        timestamp: 1625000000000,
        source: { type: 'user', userId: 'Uabc123' },
        replyToken: 'reply-token-xxx',
        message: { id: 'msg1', type: 'text', text: 'Hello' },
      },
    ],
  })

  const result = parseWebhookBody(body)
  t.is(result.destination, 'Uxxxxxxxx')
  t.is(result.events.length, 1)
  t.is(result.events[0].eventType, 'message')
  t.is(result.events[0].source.sourceType, 'user')
  t.is(result.events[0].source.userId, 'Uabc123')
  t.is(result.events[0].message?.messageType, 'text')
  t.is(result.events[0].message?.text, 'Hello')
})

test('parseWebhookBody throws on invalid JSON', (t) => {
  t.throws(() => parseWebhookBody('not-json'), { message: /Invalid webhook body/ })
})
