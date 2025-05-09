/// <reference types="vite/client" />

declare const __APP_VERSION__: string

interface State {
  lastClickedHref: string;
  lastClickedTime: Date | null;
}

interface Payload {
  url: string;
  final_url: string;
}
