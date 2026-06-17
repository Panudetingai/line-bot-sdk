/**
 * Outbound message objects for pushMessages / replyMessages / multicast / broadcast.
 * Serialized as JSON with `"type"` per the LINE Messaging API (not napi struct field names).
 */

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
