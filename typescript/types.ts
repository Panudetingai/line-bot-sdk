// Push Message Request
export interface PushMessageRequest {
  to: string
  messages: LineMessage[]
}

// ── Message Types ─────────────────────────────────────────────────────────────

export interface TextMessage {
  type: 'text'
  text: string
  quickReply?: QuickReply
  sender?: Sender
}

export interface ImageMessage {
  type: 'image'
  originalContentUrl: string
  previewImageUrl: string
  quickReply?: QuickReply
}

export interface VideoMessage {
  type: 'video'
  originalContentUrl: string
  previewImageUrl: string
  trackingId?: string
  quickReply?: QuickReply
}

export interface AudioMessage {
  type: 'audio'
  originalContentUrl: string
  duration: number
  quickReply?: QuickReply
}

export interface LocationMessage {
  type: 'location'
  title: string
  address: string
  latitude: number
  longitude: number
  quickReply?: QuickReply
}

export interface StickerMessage {
  type: 'sticker'
  packageId: string
  stickerId: string
  quickReply?: QuickReply
}

/** Flex message — pass the full LINE flex container object */
export interface FlexMessage {
  type: 'flex'
  altText: string
  contents: Record<string, unknown>
  quickReply?: QuickReply
}

/** Template message — pass buttons, confirm, carousel, or image_carousel */
export interface TemplateMessage {
  type: 'template'
  altText: string
  template: Record<string, unknown>
  quickReply?: QuickReply
}

export type LineMessage =
  | TextMessage
  | ImageMessage
  | VideoMessage
  | AudioMessage
  | LocationMessage
  | StickerMessage
  | FlexMessage
  | TemplateMessage

// ── Quick Reply ───────────────────────────────────────────────────────────────

export interface QuickReply {
  items: QuickReplyItem[]
}

export interface QuickReplyItem {
  type: 'action'
  action:
    | MessageAction
    | PostbackAction
    | URIAction
    | DatetimePickerAction
    | CameraAction
    | CameraRollAction
    | LocationAction
  imageUrl?: string
}

export interface MessageAction {
  type: 'message'
  label: string
  text: string
}

export interface PostbackAction {
  type: 'postback'
  label: string
  data: string
  text?: string
  displayText?: string
}

export interface URIAction {
  type: 'uri'
  label: string
  uri: string
}

export interface DatetimePickerAction {
  type: 'datetimepicker'
  label: string
  data: string
  mode: 'date' | 'time' | 'datetime'
  initial?: string
  max?: string
  min?: string
}

export interface CameraAction {
  type: 'camera'
  label: string
}

export interface CameraRollAction {
  type: 'cameraRoll'
  label: string
}

export interface LocationAction {
  type: 'location'
  label: string
}

export interface Sender {
  name?: string
  iconUrl?: string
}

// ── Profile / Group ───────────────────────────────────────────────────────────

export interface BotInfo {
  userId: string
  basicId: string
  displayName: string
  pictureUrl: string
  chatMode: string
  markAsReadMode: string
}

export interface Profile {
  displayName: string
  userId: string
  pictureUrl?: string
  statusMessage?: string
}

export interface GroupSummary {
  groupId: string
  groupName: string
  pictureUrl?: string
}

export interface GroupMemberProfile {
  displayName: string
  userId: string
  pictureUrl?: string
}

export interface MemberIds {
  memberIds: string[]
  next?: string
}

export interface FollowerIds {
  userIds: string[]
  next?: string
}

// ── Rich Menu ─────────────────────────────────────────────────────────────────

export interface RichMenuSize {
  width: 2500
  height: 843 | 1686
}

export interface RichMenuBounds {
  x: number
  y: number
  width: number
  height: number
}

export type RichMenuAction =
  | { type: 'postback'; data: string; label?: string; displayText?: string }
  | { type: 'message'; label?: string; text: string }
  | { type: 'uri'; label?: string; uri: string }
  | { type: 'richmenuswitch'; richMenuAliasId: string; data: string }

export interface RichMenuArea {
  bounds: RichMenuBounds
  action: RichMenuAction
}

export interface RichMenu {
  size: RichMenuSize
  selected: boolean
  name: string
  chatBarText: string
  areas: RichMenuArea[]
}

export interface RichMenuResponse extends RichMenu {
  richMenuId: string
}

// ── Webhook Events ────────────────────────────────────────────────────────────

export interface WebhookBody {
  destination: string
  events: WebhookEvent[]
}

export type WebhookEvent =
  | MessageEvent
  | FollowEvent
  | UnfollowEvent
  | JoinEvent
  | LeaveEvent
  | PostbackEvent
  | BeaconEvent
  | AccountLinkEvent

interface BaseEvent {
  timestamp: number
  source: EventSource
  webhookEventId?: string
  deliveryContext?: { isRedelivery: boolean }
  replyToken?: string
}

export interface EventSource {
  type: 'user' | 'group' | 'room'
  userId?: string
  groupId?: string
  roomId?: string
}

export interface MessageEvent extends BaseEvent {
  type: 'message'
  replyToken: string
  message: IncomingMessage
}

export type IncomingMessage =
  | IncomingTextMessage
  | IncomingImageMessage
  | IncomingVideoMessage
  | IncomingAudioMessage
  | IncomingFileMessage
  | IncomingLocationMessage
  | IncomingStickerMessage

interface BaseIncomingMessage {
  id: string
}

export interface IncomingTextMessage extends BaseIncomingMessage {
  type: 'text'
  text: string
}

export interface IncomingImageMessage extends BaseIncomingMessage {
  type: 'image'
  contentProvider: ContentProvider
}

export interface IncomingVideoMessage extends BaseIncomingMessage {
  type: 'video'
  duration: number
  contentProvider: ContentProvider
}

export interface IncomingAudioMessage extends BaseIncomingMessage {
  type: 'audio'
  duration: number
  contentProvider: ContentProvider
}

export interface IncomingFileMessage extends BaseIncomingMessage {
  type: 'file'
  fileName: string
  fileSize: number
}

export interface IncomingLocationMessage extends BaseIncomingMessage {
  type: 'location'
  title?: string
  address?: string
  latitude: number
  longitude: number
}

export interface IncomingStickerMessage extends BaseIncomingMessage {
  type: 'sticker'
  packageId: string
  stickerId: string
  stickerResourceType: string
}

export interface ContentProvider {
  type: 'line' | 'external'
  originalContentUrl?: string
  previewImageUrl?: string
}

export interface FollowEvent extends BaseEvent {
  type: 'follow'
  replyToken: string
}

export interface UnfollowEvent extends BaseEvent {
  type: 'unfollow'
}

export interface JoinEvent extends BaseEvent {
  type: 'join'
  replyToken: string
}

export interface LeaveEvent extends BaseEvent {
  type: 'leave'
}

export interface PostbackEvent extends BaseEvent {
  type: 'postback'
  replyToken: string
  postback: {
    data: string
    params?: { date?: string; time?: string; datetime?: string }
  }
}

export interface BeaconEvent extends BaseEvent {
  type: 'beacon'
  replyToken: string
  beacon: { hwid: string; type: string; dm?: string }
}

export interface AccountLinkEvent extends BaseEvent {
  type: 'accountLink'
  replyToken: string
  link: { result: 'ok' | 'failed'; nonce: string }
}

// ── Insight ───────────────────────────────────────────────────────────────────

export interface MessageDeliveryOverview {
  status: string
  broadcast?: number
  targeting?: number
}

export interface FollowersInsight {
  status: string
  followers?: number
  targetedReaches?: number
  blocks?: number
}

export interface MessageQuota {
  type: 'none' | 'limited'
  value?: number
}

export interface MessageQuotaConsumption {
  totalUsage: number
}
