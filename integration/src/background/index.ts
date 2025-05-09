import { AppMessage, AppMessageChannel, AppMessageData } from "../ambient";

const HOST = "127.0.0.1";
const PORT = 0xDEAD

let state: State = {
  lastClickedHref: new String().toString(),
  lastClickedTime: null,
}


chrome.runtime.onMessage.addListener((message: AppMessageData<string>) => {
  if (message.channel != AppMessageChannel.FromContentScript) return;

  if (message.type == AppMessage.URL) {
    state.lastClickedHref = message.data as string;
    state.lastClickedTime = new Date();
  }
})

chrome.downloads.onCreated.addListener((item) => {
  fetch(`http://${HOST}:${PORT}/`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      url: item.url,
      final_url: item.finalUrl
    } as Payload)
  }).then(async () => {
    chrome.downloads.setShelfEnabled(false);
    await chrome.downloads.cancel(item.id)
    await chrome.downloads.erase({ id: item.id })

    state = {
      lastClickedHref: "",
      lastClickedTime: null
    }
  }).catch(console.log)
})
