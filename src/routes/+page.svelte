<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import {events, commands, type RepositoryInfo} from "$lib/bindings"
  import "./page.css";

  // 샘플 레포지토리 데이터
  let repositories = $state<RepositoryInfo[]>([
    {
      id: 1,
      name: "MyaoLand",
      path: "C:\\Users\\shlif\\MyaoLand",
      branch: "main",
      gameVersion: "1.0.2",
      gameVersions: ["1.0.0", "1.0.1", "1.0.2", "1.1.0", "1.2.0"],
      server: "LIVE",
      serverOptions: ["DEV", "QA", "LIVE"],
      hasWarning: false,
    },
    {
      id: 2,
      name: "LocaleKit",
      path: "...sers\\shlif\\@src\\percent-localization-v2\\SampleProjects\\LocaleKit",
      branch: "develop",
      gameVersion: "1.0.1",
      gameVersions: ["1.0.0", "1.0.1", "1.0.2"],
      server: "QA",
      serverOptions: ["DEV", "QA"],
      hasWarning: false,
    },
  ]);


  let searchQuery = $state("");
  let showModal = $state(false);
  let newRepoName = $state("");
  let newRepoUrl = $state("");
  let counter = $state(0);

  function openModal() {
    showModal = true;
    newRepoName = "";
    newRepoUrl = "";
  }

  function closeModal() {
    showModal = false;
  }


  onMount(async () => {
    console.log("Mounted!");
  });
</script>


<!-- 헤더 -->
<header class="header">
  <div class="header-main">
    <h1>Projects</h1>
    <div class="header-controls">
      <div class="search-box">
        <span class="search-icon">🔍</span>
        <input type="text" placeholder="Search" bind:value={searchQuery} />
      </div>
      <button class="btn-secondary dropdown"> Add ▼ </button>
      <button class="btn-primary" onclick={openModal}>
        Add Repository
      </button>
    </div>
  </div>

  <button onclick={async ()=>{
     counter = await commands.increaseCounter();
  }}> Click Me! {counter} </button>

  <!-- 테이블 헤더 -->
  <div class="table-header">
    <div class="header-cell name-cell">Name</div>
    <div class="header-cell version-cell">Game Version</div>
    <div class="header-cell server-cell">Server</div>
    <div class="header-cell settings-cell">⚙️</div>
  </div>
</header>

<!-- 레포지토리 리스트 -->
<div class="repository-list">
  {#each repositories as repo}
    <div class="repo-item">
      <div class="cell name-cell">
        <div class="repo-info">
          {#if repo.hasWarning}
            <span class="warning-icon">⚠️</span>
          {/if}
          <div>
            <div class="repo-name">{repo.name}</div>
            <div class="repo-path">{repo.path}</div>
          </div>
        </div>
      </div>
      <div class="cell version-cell">
        <select bind:value={repo.gameVersion} class="version-select">
          {#each repo.gameVersions as version}
            <option value={version}>{version}</option>
          {/each}
        </select>
      </div>
      <div class="cell server-cell">
        <select bind:value={repo.server} class="server-select">
          {#each repo.serverOptions as server}
            <option value={server}>{server}</option>
          {/each}
        </select>
      </div>
      <div class="cell settings-cell">
        <button class="icon-btn">⋯</button>
      </div>
    </div>
  {/each}
</div>

<!-- 모달 -->
{#if showModal}
  <div class="modal-overlay" onclick={closeModal}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
      <h2>Add Repository</h2>
      <form onsubmit={(e) => { e.preventDefault(); closeModal();}}>
        <div class="form-group">
          <label for="repo-name">Repository Name</label>
          <input
            id="repo-name"
            type="text"
            placeholder="e.g., MyProject"
            bind:value={newRepoName}
            required
          />
        </div>
        <div class="form-group">
          <label for="repo-url">Repository URL</label>
          <input
            id="repo-url"
            type="text"
            placeholder="e.g., C:\path\to\repo"
            bind:value={newRepoUrl}
            required
          />
        </div>
        <div class="modal-actions">
          <button type="button" class="btn-secondary" onclick={closeModal}>
            Cancel
          </button>
          <button type="submit" class="btn-primary">Add</button>
        </div>
      </form>
    </div>
  </div>
{/if}
