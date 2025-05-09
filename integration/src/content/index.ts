import { AppMessage, AppMessageChannel, AppMessageData } from "../ambient"


document.addEventListener("click", async function(event) {
  let element = event.target as HTMLElement;

  if (element.tagName.toLowerCase() == "a") {
    let href = element.getAttribute("href") ?? element.closest("a")?.getAttribute("href");

    if (href) {
      let url;

      try {
        url = new URL(href).href;
      } catch {
        url = new URL(href, window.location.href).href;
      }

      await chrome.runtime.sendMessage({
        channel: AppMessageChannel.FromContentScript,
        type: AppMessage.URL,
        data: url
      } as AppMessageData<string>)
    }
  }

}, true);
