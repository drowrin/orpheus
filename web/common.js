if (
  localStorage.theme === "dark" ||
  (!("theme" in localStorage) &&
    window.matchMedia("(prefers-color-scheme: dark)").matches)
) {
  document.documentElement.classList.add("dark");
} else {
  document.documentElement.classList.remove("dark");
}

function toggle_dark_mode() {
  let t = document.documentElement.classList.contains("dark")
    ? "light"
    : "dark";
  localStorage.theme = t;
  if (t === "dark") {
    document.documentElement.classList.add("dark");
  } else {
    document.documentElement.classList.remove("dark");
  }
}

function remove_empty(obj) {
  return Object.fromEntries(Object.entries(obj).filter(([_, v]) => v != ""));
}
