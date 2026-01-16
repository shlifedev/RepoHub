<script lang="ts">
  import { onMount } from "svelte"
  import { open } from "@tauri-apps/plugin-dialog"
  import { commands } from "$lib/bindings"
  import { _, locale, isLoading } from "svelte-i18n"
  import "$lib/i18n"

  let rootPath = $state("")
  let currentLocale = $state("en")

  const supportedLocales = [
    { code: "en", name: "English" },
    { code: "ko", name: "한국어" },
    { code: "ja", name: "日本語" },
    { code: "zh-CN", name: "简体中文" },
    { code: "zh-TW", name: "繁體中文" }
  ]

  onMount(async () => {
    const path = await commands.getRootPath()
    if (path) {
      rootPath = path
    }
    currentLocale = localStorage.getItem("locale") ?? "en"
  })

  async function selectRootPath() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: $_("settings.rootPathDescription")
    })

    if (selected && typeof selected === "string") {
      const result = await commands.setRootPath(selected)
      rootPath = result
    }
  }

  function handleLocaleChange(e: Event) {
    const target = e.target as HTMLSelectElement
    const newLocale = target.value
    currentLocale = newLocale
    locale.set(newLocale)
    localStorage.setItem("locale", newLocale)
  }
</script>

{#if $isLoading}
  <div class="loading">Loading...</div>
{:else}
  <div class="settings-container">
    <header class="settings-header">
      <h1>{$_("settings.title")}</h1>
    </header>

    <div class="settings-content">
      <section class="settings-section">
        <h2>{$_("settings.repository")}</h2>
        <div class="setting-item">
          <div class="setting-info">
            <label for="root-path">{$_("settings.rootPath")}</label>
            <p class="setting-description">
              {$_("settings.rootPathDescription")}
            </p>
          </div>
          <div class="path-selector">
            <span class="current-path" title={rootPath}>
              {rootPath || $_("settings.notSet")}
            </span>
            <button class="btn-browse" onclick={selectRootPath}>
              {$_("actions.browse")}
            </button>
          </div>
        </div>
      </section>

      <section class="settings-section">
        <h2>{$_("settings.language")}</h2>
        <div class="setting-item">
          <div class="setting-info">
            <label for="language-select">{$_("settings.language")}</label>
            <p class="setting-description">
              {$_("settings.languageDescription")}
            </p>
          </div>
          <select
            id="language-select"
            value={currentLocale}
            onchange={handleLocaleChange}
          >
            {#each supportedLocales as lang}
              <option value={lang.code}>{lang.name}</option>
            {/each}
          </select>
        </div>
      </section>
    </div>
  </div>
{/if}

<style>
  .loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100%;
    color: #808080;
  }

  .settings-container {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .settings-header {
    padding: 30px;
    border-bottom: 1px solid #333;
  }

  .settings-header h1 {
    margin: 0;
    font-size: 28px;
    font-weight: 600;
  }

  .settings-content {
    flex: 1;
    overflow-y: auto;
    padding: 30px;
  }

  .settings-section {
    margin-bottom: 40px;
  }

  .settings-section h2 {
    font-size: 18px;
    font-weight: 600;
    margin-bottom: 20px;
    color: #fff;
  }

  .setting-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 0;
    border-bottom: 1px solid #282828;
    gap: 20px;
    flex-wrap: wrap;
  }

  .setting-item:last-child {
    border-bottom: none;
  }

  .setting-info {
    flex: 1;
  }

  .setting-info label {
    display: block;
    font-size: 15px;
    font-weight: 500;
    color: #fff;
    margin-bottom: 4px;
  }

  .setting-description {
    margin: 0;
    font-size: 13px;
    color: #808080;
  }

  select {
    padding: 8px 12px;
    background-color: #2d2d2d;
    border: 1px solid #404040;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 14px;
    cursor: pointer;
    min-width: 150px;
    max-width: 200px;
  }

  select:focus {
    outline: none;
    border-color: #4a9eff;
  }

  .path-selector {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .current-path {
    max-width: 300px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: #808080;
    font-size: 14px;
    flex: 1;
    min-width: 100px;
  }

  .btn-browse {
    padding: 8px 16px;
    background-color: #2d2d2d;
    border: 1px solid #404040;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .btn-browse:hover {
    background-color: #3d3d3d;
    border-color: #505050;
  }

  @media (max-width: 700px) {
    .settings-header {
      padding: 20px;
    }

    .settings-header h1 {
      font-size: 22px;
    }

    .settings-content {
      padding: 20px;
    }

    .setting-item {
      flex-direction: column;
      align-items: flex-start;
    }

    .path-selector {
      width: 100%;
    }

    .current-path {
      max-width: none;
    }

    select {
      width: 100%;
      max-width: none;
    }
  }
</style>
