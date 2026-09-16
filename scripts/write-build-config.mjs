import { mkdirSync, writeFileSync } from "node:fs";

const required = [
  "RETHEME_API_SECRET_ID",
  "RETHEME_API_SECRET_KEY",
  "RETHEME_PACKAGE_KEY",
  "RETHEME_LICENSE_PUBLIC_KEY",
  "RETHEME_COMPATIBILITY_PUBLIC_KEY",
  "RETHEME_THEME_PUBLIC_KEY",
];

const missing = required.filter((name) => !process.env[name]);
if (missing.length > 0) {
  throw new Error(`Missing build secrets: ${missing.join(", ")}`);
}

const string = (value) => JSON.stringify(value);
mkdirSync("src-tauri/config", { recursive: true });
writeFileSync("src-tauri/config/api.toml", [
  `base_url = ${string(process.env.RETHEME_API_BASE_URL || "https://theme.dux.cn/api/desktop/v1")}`,
  `secret_id = ${string(process.env.RETHEME_API_SECRET_ID)}`,
  `secret_key = ${string(process.env.RETHEME_API_SECRET_KEY)}`,
  "",
].join("\n"), { mode: 0o600 });
writeFileSync("src-tauri/config/security.toml", [
  `package_key = ${string(process.env.RETHEME_PACKAGE_KEY)}`,
  `license_public_key = ${string(process.env.RETHEME_LICENSE_PUBLIC_KEY)}`,
  `compatibility_public_key = ${string(process.env.RETHEME_COMPATIBILITY_PUBLIC_KEY)}`,
  `theme_public_key = ${string(process.env.RETHEME_THEME_PUBLIC_KEY)}`,
  "",
].join("\n"), { mode: 0o600 });
