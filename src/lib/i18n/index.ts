import { browser } from "$app/environment"
import { init, register, locale } from "svelte-i18n"

register("en", () => import("./locales/en.json"))
register("ko", () => import("./locales/ko.json"))
register("ja", () => import("./locales/ja.json"))
register("zh-CN", () => import("./locales/zh-CN.json"))
register("zh-TW", () => import("./locales/zh-TW.json"))

const defaultLocale = "en"

init({
  fallbackLocale: defaultLocale,
  initialLocale: browser ? window.localStorage.getItem("locale") ?? defaultLocale : defaultLocale,
})

export { locale }
