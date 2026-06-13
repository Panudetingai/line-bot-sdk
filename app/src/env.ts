function required(name: string, ...fallbacks: string[]): string {
  for (const key of [name, ...fallbacks]) {
    const value = process.env[key]?.trim()
    if (value) return value
  }
  throw new Error(`Missing required environment variable: ${name}`)
}

export const env = {
  port: Number(process.env.PORT ?? 3000),
  accessToken: required('LINE_CHANNEL_ACCESS_TOKEN', 'LINE_ACCESS_TOKEN'),
  channelSecret: process.env.LINE_CHANNEL_SECRET?.trim() ?? '',
}
