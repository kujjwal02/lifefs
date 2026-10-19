import { commands } from "./bindings";

const versionLabel = document.querySelector<HTMLSpanElement>("#app-version");
if (versionLabel) {
  versionLabel.textContent = await commands.appVersion();
}
