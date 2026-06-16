import express from 'express'
import morgan from 'morgan'
import { LineClient } from '../../typescript/line-client'
import { env } from './env.js'
import { createWebhookHandler } from './webhook.js'

const client = new LineClient({ accessToken: env.accessToken })
const app = express()

app.use(morgan('dev'))
app.get('/', (_req, res) => {
  res.json({
    name: 'line-bot-sdk test server',
    endpoints: {
      health: 'GET /health',
      bot: 'GET /api/bot',
      profile: 'GET /api/profile/:userId',
      quota: 'GET /api/quota',
      webhook: 'POST /webhook',
    },
  })
})

app.get('/health', (_req, res) => {
  res.json({ status: 'ok' })
})

app.get('/api/bot', async (_req, res) => {
  try {
    const bot = await client.getBotInfo()
    res.json(bot)
  } catch (err) {
    console.error('[api/bot]', err)
    res.status(500).json({ error: String(err) })
  }
})

app.get('/api/profile/:userId', async (req, res) => {
  try {
    const profile = await client.getProfile(req.params.userId)
    res.json(profile)
  } catch (err) {
    console.error('[api/profile]', err)
    res.status(500).json({ error: String(err) })
  }
})

app.get('/api/quota', async (_req, res) => {
  try {
    const [quota, consumption] = await Promise.all([client.getMessageQuota(), client.getMessageQuotaConsumption()])
    res.json({ quota, consumption })
  } catch (err) {
    console.error('[api/quota]', err)
    res.status(500).json({ error: String(err) })
  }
})

app.get('/webhook', (req, res) => {
  res.json({ status: 'ok' })
})

app.post('/webhook', express.raw({ type: '*/*' }), createWebhookHandler(client))

app.listen(env.port, () => {
  console.log(`Test server running at http://localhost:${env.port}`)
  console.log(`Webhook URL: http://localhost:${env.port}/webhook`)
  console.log('Set this URL in LINE Developers Console (use ngrok for local testing)')
})
