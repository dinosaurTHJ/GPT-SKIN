import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App";
import "./styles.css";

if (navigator.userAgent.includes("Windows")) {
  document.documentElement.dataset.platform = "windows";
}

function isEditable(target: EventTarget | null) {
  return target instanceof HTMLElement && (target.closest("input, textarea, [contenteditable='true']") !== null);
}

// 桌面应用行为：禁用右键菜单与元素拖放（输入框保留原生编辑菜单）
document.addEventListener("contextmenu", (event) => {
  if (!isEditable(event.target)) event.preventDefault();
});
document.addEventListener("dragstart", (event) => {
  if (!isEditable(event.target)) event.preventDefault();
});

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
