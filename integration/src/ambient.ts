export const enum AppMessage {
  URL = 0
}

export const enum AppMessageChannel {
  FromServiceWorker = 0,   // Message sent from background/service worker
  FromContentScript = 1    // Message sent from content script (executing in page context)
}

export type AppMessageData<T> = {
  channel: AppMessageChannel,
  type: AppMessage;
  data: T;
}
