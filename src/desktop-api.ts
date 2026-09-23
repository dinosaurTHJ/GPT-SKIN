import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { openUrl } from "@tauri-apps/plugin-opener";
import type { ImageAsset, LiveThemeStatus } from "./types";

export const isDesktopRuntime = () => "__TAURI_INTERNALS__" in window;

export async function listImageAssets(): Promise<ImageAsset[]> {
  if (!isDesktopRuntime()) return [];
  return invoke<ImageAsset[]>("list_image_assets");
}

export async function getImageLibraryRoot(): Promise<string> {
  if (!isDesktopRuntime()) return "C:\\image";
  return invoke<string>("get_image_library_root");
}

export async function chooseImageLibraryRoot(): Promise<string | null> {
  if (!isDesktopRuntime()) return null;
  const selected = await open({
    title: "选择本地背景目录",
    multiple: false,
    directory: true,
  });
  if (!selected || Array.isArray(selected)) return null;
  return invoke<string>("set_image_library_root", { path: selected });
}

export async function openChatGPTWindow(): Promise<boolean> {
  if (!isDesktopRuntime()) return true;
  return invoke<boolean>("open_chatgpt_window");
}

export async function openOnlineStore(): Promise<void> {
  const url = "https://dreamskin.cc/gallery";
  if (!isDesktopRuntime()) {
    window.open(url, "_blank", "noopener,noreferrer");
    return;
  }
  await openUrl(url);
}

export async function applyImageTheme(imagePath: string, _locale: string, opacity = 0.8): Promise<LiveThemeStatus> {
  if (!isDesktopRuntime()) return { active: true, imagePath, themeId: "local-image-preview" };
  return invoke<LiveThemeStatus>("apply_live_image_theme", { imagePath, locale: "zh-CN", opacity });
}

export async function getImageOpacity(): Promise<number> {
  if (!isDesktopRuntime()) return 0.8;
  return invoke<number>("get_image_opacity");
}

export async function setImageOpacity(opacity: number): Promise<number> {
  if (!isDesktopRuntime()) return opacity;
  return invoke<number>("set_image_opacity", { opacity });
}

export async function restoreOfficialTheme(): Promise<boolean> {
  if (!isDesktopRuntime()) return true;
  return invoke<boolean>("restore_live_theme");
}

export async function getLiveThemeStatus(): Promise<LiveThemeStatus> {
  if (!isDesktopRuntime()) return { active: false };
  return invoke<LiveThemeStatus>("get_live_theme_status");
}
