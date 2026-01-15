<script lang="ts">
  import { onMount } from "svelte"
  import { open } from "@tauri-apps/plugin-dialog"
  import { commands } from "$lib/bindings"

  let rootPath = $state("")

  onMount(async () => {
    const path = await commands.getRootPath()
    if (path) {
      rootPath = path
    }
  })

  async function selectRootPath() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "Select Repository Root Directory"
    })

    if (selected && typeof selected === "string") {
      const result = await commands.setRootPath(selected)
      rootPath = result
    }
  }
</script>

<div class="settings-container">
  <header class="settings-header">
    <h1>Settings</h1>
  </header>

  <div class="settings-content">
    <section class="settings-section">
      <h2>Repository</h2>
      <div class="setting-item">
        <div class="setting-info">
          <label for="root-path">Root Path</label>
          <p class="setting-description">
            Directory where repositories will be cloned
          </p>
        </div>
        <div class="path-selector">
          <span class="current-path" title={rootPath}>
            {rootPath || "Not set"}
          </span>
          <button class="btn-browse" onclick={selectRootPath}>
            Browse
          </button>
        </div>
      </div>
    </section>
  </div>
</div>

<style>
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
    min-width: 200px;
  }

  select:focus {
    outline: none;
    border-color: #4a9eff;
  }

  /* Toggle Switch */
  .toggle {
    position: relative;
    display: inline-block;
    width: 48px;
    height: 26px;
  }

  .toggle input {
    opacity: 0;
    width: 0;
    height: 0;
  }

  .slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: #404040;
    transition: 0.3s;
    border-radius: 26px;
  }

  .slider:before {
    position: absolute;
    content: "";
    height: 18px;
    width: 18px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    transition: 0.3s;
    border-radius: 50%;
  }

  input:checked + .slider {
    background-color: #1e88e5;
  }

  input:checked + .slider:before {
    transform: translateX(22px);
  }

  .about-info {
    padding: 20px;
    background-color: #202020;
    border-radius: 8px;
    border: 1px solid #333;
  }

  .about-info p {
    margin: 8px 0;
    font-size: 14px;
    color: #b0b0b0;
  }

  .about-info strong {
    color: #fff;
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
</style>
