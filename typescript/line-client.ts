const {
  LineClient: _LineClient,
  verifyWebhookSignature,
  parseWebhookBody,
} = require('../index.js') as typeof import('../index')
import type {
  BotInfo,
  FollowerIds,
  FollowersInsight,
  GroupMemberProfile,
  GroupSummary,
  LineMessage,
  MemberIds,
  MessageDeliveryOverview,
  MessageQuota,
  MessageQuotaConsumption,
  Profile,
  RichMenu,
  RichMenuResponse,
  WebhookBody,
} from './types'

export class LineClient {
  private _client: InstanceType<typeof _LineClient>

  constructor({ accessToken }: { accessToken: string }) {
    this._client = new _LineClient({
      accessToken: accessToken,
    })
  }

  // ── Messaging ─────────────────────────────────────────────────────────────────

  /** Send a single text message to one user. */
  async pushText(to: string, text: string): Promise<void> {
    await this._client.pushMessage(to, text)
  }

  /** Send any LINE message type(s) to one user. */
  async pushMessages(to: string, messages: LineMessage[]): Promise<void> {
    await this._client.pushMessages(to, JSON.stringify(messages))
  }

  /** Reply to a webhook event with a single text message. */
  async replyText(replyToken: string, text: string): Promise<void> {
    await this._client.replyMessage(replyToken, text)
  }

  /** Reply to a webhook event with any message type(s). */
  async replyMessages(replyToken: string, messages: LineMessage[]): Promise<void> {
    await this._client.replyMessages(replyToken, JSON.stringify(messages))
  }

  /** Send messages to multiple users (up to 500 users). */
  async multicast(userIds: string[], messages: LineMessage[]): Promise<void> {
    await this._client.multicast(JSON.stringify(userIds), JSON.stringify(messages))
  }

  /** Send messages to all followers. */
  async broadcast(messages: LineMessage[]): Promise<void> {
    await this._client.broadcast(JSON.stringify(messages))
  }

  /** Get message quota for the current month. */
  async getMessageQuota(): Promise<MessageQuota> {
    return this._client.getMessageQuota() as unknown as MessageQuota
  }

  /** Get message quota consumption for the current month. */
  async getMessageQuotaConsumption(): Promise<MessageQuotaConsumption> {
    return this._client.getMessageQuotaConsumption() as unknown as MessageQuotaConsumption
  }

  /** Download message content (image, video, audio, file). Returns Buffer. */
  async getMessageContent(messageId: string): Promise<Buffer> {
    return this._client.getMessageContent(messageId) as unknown as Buffer
  }

  // ── Bot ───────────────────────────────────────────────────────────────────────

  /** Get channel bot info. */
  async getBotInfo(): Promise<BotInfo> {
    return this._client.verify() as unknown as BotInfo
  }

  // ── Profile ───────────────────────────────────────────────────────────────────

  /** Get a user's profile. */
  async getProfile(userId: string): Promise<Profile> {
    return this._client.getProfile(userId) as unknown as Profile
  }

  /** Get follower user IDs (paginated). */
  async getFollowerIds(next?: string): Promise<FollowerIds> {
    return this._client.getFollowerIds(next) as unknown as FollowerIds
  }

  // ── Group ─────────────────────────────────────────────────────────────────────

  /** Get a group's summary. */
  async getGroupSummary(groupId: string): Promise<GroupSummary> {
    return this._client.getGroupSummary(groupId) as unknown as GroupSummary
  }

  /** Get a group member's profile. */
  async getGroupMemberProfile(groupId: string, userId: string): Promise<GroupMemberProfile> {
    return this._client.getGroupMemberProfile(groupId, userId) as unknown as GroupMemberProfile
  }

  /** Get member IDs in a group (paginated). */
  async getGroupMemberIds(groupId: string, next?: string): Promise<MemberIds> {
    return this._client.getGroupMemberIds(groupId, next) as unknown as MemberIds
  }

  /** Leave a group. */
  async leaveGroup(groupId: string): Promise<void> {
    await this._client.leaveGroup(groupId)
  }

  /** Get a room member's profile. */
  async getRoomMemberProfile(roomId: string, userId: string): Promise<GroupMemberProfile> {
    return this._client.getRoomMemberProfile(roomId, userId) as unknown as GroupMemberProfile
  }

  /** Get member IDs in a room (paginated). */
  async getRoomMemberIds(roomId: string, next?: string): Promise<MemberIds> {
    return this._client.getRoomMemberIds(roomId, next) as unknown as MemberIds
  }

  /** Leave a room. */
  async leaveRoom(roomId: string): Promise<void> {
    await this._client.leaveRoom(roomId)
  }

  // ── Rich Menu ─────────────────────────────────────────────────────────────────

  /** Create a rich menu. Returns the richMenuId. */
  async createRichMenu(richMenu: RichMenu): Promise<string> {
    return this._client.createRichMenu(richMenu as never)
  }

  /** Get a rich menu by ID. */
  async getRichMenu(richMenuId: string): Promise<RichMenuResponse> {
    return this._client.getRichMenu(richMenuId) as unknown as RichMenuResponse
  }

  /** Delete a rich menu. */
  async deleteRichMenu(richMenuId: string): Promise<void> {
    await this._client.deleteRichMenu(richMenuId)
  }

  /** Get all rich menus. */
  async getRichMenuList(): Promise<RichMenuResponse[]> {
    const res = (await this._client.getRichMenuList()) as unknown as { richmenus: RichMenuResponse[] }
    return res.richmenus
  }

  /** Upload a rich menu image. contentType: 'image/jpeg' or 'image/png'. */
  async uploadRichMenuImage(
    richMenuId: string,
    imageData: Buffer,
    contentType: 'image/jpeg' | 'image/png' = 'image/jpeg',
  ): Promise<void> {
    await this._client.uploadRichMenuImage(richMenuId, imageData as never, contentType)
  }

  /** Download a rich menu image. Returns raw Buffer. */
  async downloadRichMenuImage(richMenuId: string): Promise<Buffer> {
    return this._client.downloadRichMenuImage(richMenuId) as unknown as Buffer
  }

  /** Set the default rich menu for all users. */
  async setDefaultRichMenu(richMenuId: string): Promise<void> {
    await this._client.setDefaultRichMenu(richMenuId)
  }

  /** Remove the default rich menu. */
  async cancelDefaultRichMenu(): Promise<void> {
    await this._client.cancelDefaultRichMenu()
  }

  /** Get the rich menu linked to a user. */
  async getUserRichMenu(userId: string): Promise<string> {
    const res = (await this._client.getUserRichMenu(userId)) as unknown as { richMenuId: string }
    return res.richMenuId
  }

  /** Link a rich menu to a user. */
  async linkRichMenuToUser(userId: string, richMenuId: string): Promise<void> {
    await this._client.linkRichMenuToUser(userId, richMenuId)
  }

  /** Unlink rich menu from a user. */
  async unlinkRichMenuFromUser(userId: string): Promise<void> {
    await this._client.unlinkRichMenuFromUser(userId)
  }

  /** Bulk link a rich menu to multiple users. */
  async bulkLinkRichMenu(richMenuId: string, userIds: string[]): Promise<void> {
    await this._client.bulkLinkRichMenu(richMenuId, JSON.stringify(userIds))
  }

  /** Bulk unlink rich menu from multiple users. */
  async bulkUnlinkRichMenu(userIds: string[]): Promise<void> {
    await this._client.bulkUnlinkRichMenu(JSON.stringify(userIds))
  }

  // ── Insight ───────────────────────────────────────────────────────────────────

  /** Get message delivery overview. date format: 'YYYYMMDD'. */
  async getMessageDeliveryOverview(date: string): Promise<MessageDeliveryOverview> {
    return this._client.getMessageDeliveryOverview(date) as unknown as MessageDeliveryOverview
  }

  /** Get followers insight. date format: 'YYYYMMDD'. */
  async getFollowersInsight(date: string): Promise<FollowersInsight> {
    return this._client.getFollowersInsight(date) as unknown as FollowersInsight
  }
}

// ── Webhook helpers (standalone functions) ───────────────────────────────────

/**
 * Verify the X-Line-Signature header.
 * Use in your Next.js webhook route handler.
 */
export function verifySignature(rawBody: string, signature: string, channelSecret: string): boolean {
  return verifyWebhookSignature({
    body: rawBody,
    signature: signature,
    channelSecret: channelSecret,
  })
}

/**
 * Parse the webhook body into typed events.
 * Use in your Next.js webhook route handler.
 */
export function parseWebhook(rawBody: string): WebhookBody {
  return parseWebhookBody(rawBody) as unknown as WebhookBody
}

export * from './types'
