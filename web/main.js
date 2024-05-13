"use strict";

import "htmx.org";
import "htmx.org/dist/ext/preload";
import "htmx.org/dist/ext/head-support";

if (
  localStorage.theme === "dark" ||
  (!("theme" in localStorage) &&
    window.matchMedia("(prefers-color-scheme: dark)").matches)
) {
  document.documentElement.dataset.theme = "dark";
} else {
  document.documentElement.dataset.theme = "light";
}

function toggle_dark_mode() {
  let t = document.documentElement.dataset.theme === "dark" ? "light" : "dark";
  localStorage.theme = t;
  if (t === "dark") {
    document.documentElement.dataset.theme = "dark";
  } else {
    document.documentElement.dataset.theme = "light";
  }
}

function remove_empty(obj) {
  return Object.fromEntries(Object.entries(obj).filter(([_, v]) => v != ""));
}
