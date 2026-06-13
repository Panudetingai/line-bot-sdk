import { createHmac } from 'node:crypto'
import test from 'ava'
import { LineClient, verifyWebhookSignature, parseWebhookBody } from '../index'

// ── Constructor ───────────────────────────────────────────────────────────────

test('reject empty access token', (t) => {
  t.throws(() => new LineClient(''), { message: /access_token is required/ })
})

test('reject whitespace-only access token', (t) => {
  t.throws(() => new LineClient('   '), { message: /access_token is required/ })
})

test('accept valid access token', (t) => {
  t.notThrows(() => new LineClient('dummy-token'))
})

// ── Webhook signature verification ────────────────────────────────────────────

test('verifyWebhookSignature returns true for valid signature', (t) => {
  const body = '{"destination":"Uxxxxxxxx","events":[]}'
  const secret = 'my-channel-secret'
  const sig = createHmac('sha256', secret).update(body).digest('base64')

  t.true(verifyWebhookSignature(body, sig, secret))
})

test('verifyWebhookSignature returns false for wrong signature', (t) => {
  t.false(verifyWebhookSignature('body', 'invalidsignature==', 'secret'))
})

test('verifyWebhookSignature returns false for wrong secret', (t) => {
  const body = 'test'
  const sig = createHmac('sha256', 'correct-secret').update(body).digest('base64')

  t.false(verifyWebhookSignature(body, sig, 'wrong-secret'))
})

// ── parseWebhookBody ──────────────────────────────────────────────────────────

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
  t.is(result.events[0].source.userId, 'Uabc123')
  t.is(result.events[0].message?.text, 'Hello')
})

test('parseWebhookBody throws on invalid JSON', (t) => {
  t.throws(() => parseWebhookBody('not-json'), { message: /Invalid webhook body/ })
})

test('parseWebhookBody handles empty events array', (t) => {
  const body = JSON.stringify({ destination: 'Uyyy', events: [] })
  const result = parseWebhookBody(body)
  t.is(result.events.length, 0)
})

// ── Integration tests (require LINE_CHANNEL_ACCESS_TOKEN) ────────────────────

test('verify bot info with real token', async (t) => {
  const token = process.env.LINE_CHANNEL_ACCESS_TOKEN
  if (!token) {
    t.pass('skipped: set LINE_CHANNEL_ACCESS_TOKEN to run')
    return
  }

  const client = new LineClient(token)
  const info = await client.verify()
  const parsed = JSON.parse(info)

  t.truthy(parsed.userId || parsed.basicId)
})
