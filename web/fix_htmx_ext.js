import Prism from "prismjs";

window.htmx = require("htmx.org");
window.htmx.on("htmx:afterSwap", (event) => {
  if (event.detail.boosted) {
    Prism.highlightAll();
  }
});
