<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { events, commands } from "$lib/bindings"
  interface RepositoryInfo {
      id: number;
    name: string;
    path: string;
    branch: string;
    gameVersion: string;
    gameVersions: string[];
    server: string;
    serverOptions: string[];
    hasWarning: boolean;
  }

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
    {
      id: 3,
      name: "civiliseum",
      path: "C:\\Users\\shlif\\@src\\civiliseum\\civiliseum",
      branch: "feature/ui-improvements",
      gameVersion: "1.0.0",
      gameVersions: ["1.0.0", "1.1.0", "2.0.0"],
      server: "DEV",
      serverOptions: ["DEV", "QA", "LIVE"],
      hasWarning: false,
    },
    {
      id: 4,
      name: "Unity-Game-Framework",
      path: "C:\\Users\\shlif\\unity-game-template\\Unity-Game-Framework",
      branch: "main",
      gameVersion: "1.1.0",
      gameVersions: ["1.0.0", "1.1.0", "1.2.0", "1.3.0"],
      server: "QA",
      serverOptions: ["DEV", "QA", "LIVE"],
      hasWarning: false,
    },
    {
      id: 5,
      name: "application",
      path: "C:\\Users\\shlif\\src\\application",
      branch: "hotfix/critical-bug",
      gameVersion: "1.0.2",
      gameVersions: ["1.0.0", "1.0.1", "1.0.2"],
      server: "LIVE",
      serverOptions: ["LIVE"],
      hasWarning: true,
    },
  ]);


  let searchQuery = $state("");
  let showModal = $state(false);
  let newRepoName = $state("");
  let newRepoUrl = $state("");

  function openModal() {
    showModal = true;
    newRepoName = "";
    newRepoUrl = "";
  }

  function closeModal() {
    showModal = false;
  }

  async function submitRepository() {
    if (!newRepoName || !newRepoUrl) return;

    try {
      const newRepo: RepositoryInfo = {
        id: repositories.length + 1,
        name: newRepoName,
        path: newRepoUrl,
        branch: "main",
        gameVersion: "1.0.0",
        gameVersions: ["1.0.0", "1.0.1", "1.0.2"],
        server: "DEV",
        serverOptions: ["DEV", "QA", "LIVE"],
        hasWarning: false,
      };

      repositories = [...repositories, newRepo];

      // Rust로 저장
      await invoke("save_repositories", { repos: repositories });

      closeModal();
    } catch (error) {
      console.error("Failed to add repository:", error);
    }
  }

  // 앱 시작시 저장된 레포지토리 로드
  onMount(async () => {
    try {
      const savedRepos = await invoke<RepositoryInfo[]>("load_repositories");
      if (savedRepos.length > 0) {
        repositories = savedRepos;
        console.log("Loaded repositories:", savedRepos);
      }
    } catch (error) {
      console.error("Failed to load repositories:", error);
    }
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

  <button onclick={() => commands.helloWorld("World!")} >
    Do Something
  </button>

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
      <form onsubmit={(e) => { e.preventDefault(); submitRepository(); }}>
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

<style>
  /* 헤더 */
  .header {
    background-color: #1a1a1a;
    border-bottom: 1px solid #333;
  }

  .header-main {
    padding: 30px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .header-main h1 {
    margin: 0;
    font-size: 28px;
    font-weight: 600;
  }

  .header-controls {
    display: flex;
    gap: 10px;
    align-items: center;
  }

  .search-box {
    position: relative;
    display: flex;
    align-items: center;
  }

  .search-icon {
    position: absolute;
    left: 12px;
    opacity: 0.5;
  }

  .search-box input {
    padding: 8px 12px 8px 35px;
    background-color: #2d2d2d;
    border: 1px solid #404040;
    border-radius: 6px;
    color: #e0e0e0;
    width: 250px;
    font-size: 14px;
  }

  .search-box input:focus {
    outline: none;
    border-color: #4a9eff;
  }

  /* 버튼 */
  .btn-primary {
    padding: 8px 16px;
    background-color: #1e88e5;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    font-weight: 500;
    transition: background-color 0.2s;
  }

  .btn-primary:hover {
    background-color: #1976d2;
  }

  .btn-secondary {
    padding: 8px 16px;
    background-color: transparent;
    color: #e0e0e0;
    border: 1px solid #505050;
    border-radius: 6px;
    cursor: pointer;
    font-size: 14px;
    transition: all 0.2s;
  }

  .btn-secondary:hover {
    background-color: #2d2d2d;
    border-color: #606060;
  }

  .dropdown {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  /* 테이블 */
  .table-header {
    display: grid;
    grid-template-columns: 1fr 180px 180px 50px;
    gap: 15px;
    padding: 12px 30px;
    background-color: #202020;
    border-top: 1px solid #333;
    border-bottom: 1px solid #333;
    color: #909090;
    font-size: 13px;
    font-weight: 500;
  }

  .header-cell {
    display: flex;
    align-items: center;
  }

  /* 레포지토리 리스트 */
  .repository-list {
    flex: 1;
    overflow-y: auto;
  }

  .repo-item {
    display: grid;
    grid-template-columns: 1fr 180px 180px 50px;
    gap: 15px;
    padding: 15px 30px;
    border-bottom: 1px solid #282828;
    transition: background-color 0.2s;
    align-items: center;
  }

  .repo-item:hover {
    background-color: #222;
  }

  .cell {
    display: flex;
    align-items: center;
  }

  .settings-cell {
    justify-content: center;
  }

  .icon-btn {
    background: none;
    border: none;
    cursor: pointer;
    font-size: 16px;
    opacity: 0.6;
    transition: opacity 0.2s;
    padding: 4px;
  }

  .icon-btn:hover {
    opacity: 1;
  }

  .repo-info {
    display: flex;
    gap: 10px;
    align-items: flex-start;
  }

  .warning-icon {
    font-size: 18px;
    margin-top: 2px;
  }

  .repo-name {
    font-weight: 500;
    font-size: 15px;
    margin-bottom: 4px;
    color: #fff;
  }

  .repo-path {
    font-size: 12px;
    color: #808080;
  }

  /* 드롭다운 셀렉트 */
  .version-select,
  .server-select {
    padding: 6px 12px;
    background-color: #2d2d2d;
    border: 1px solid #404040;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 14px;
    cursor: pointer;
    width: 100%;
    transition: border-color 0.2s;
  }

  .version-select:hover,
  .server-select:hover {
    border-color: #505050;
  }

  .version-select:focus,
  .server-select:focus {
    outline: none;
    border-color: #4a9eff;
  }

  .server-select {
    font-weight: 500;
  }

  .version-cell,
  .server-cell {
    justify-content: flex-start;
  }

  /* 모달 */
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.7);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
  }

  .modal-content {
    background-color: #252525;
    border-radius: 12px;
    padding: 30px;
    width: 500px;
    max-width: 90%;
    border: 1px solid #404040;
    box-shadow: 0 10px 40px rgba(0, 0, 0, 0.5);
  }

  .modal-content h2 {
    margin: 0 0 24px 0;
    font-size: 24px;
    font-weight: 600;
    color: #fff;
  }

  .form-group {
    margin-bottom: 20px;
  }

  .form-group label {
    display: block;
    margin-bottom: 8px;
    font-size: 14px;
    font-weight: 500;
    color: #e0e0e0;
  }

  .form-group input {
    width: 100%;
    padding: 10px 12px;
    background-color: #1a1a1a;
    border: 1px solid #404040;
    border-radius: 6px;
    color: #e0e0e0;
    font-size: 14px;
    box-sizing: border-box;
  }

  .form-group input:focus {
    outline: none;
    border-color: #4a9eff;
  }

  .modal-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    margin-top: 24px;
  }
</style>
