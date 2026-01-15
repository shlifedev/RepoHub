<script lang="ts">
  import { onMount, onDestroy } from "svelte"
  import { events, commands, type RepositoryInfo } from "$lib/bindings"
  import "./page.css"

  let repositories = $state<RepositoryInfo[]>([])
  let searchQuery = $state("")
  let showModal = $state(false)
  let newRepoName = $state("")
  let newRepoUrl = $state("")
  let isCloning = $state(false)
  let cloneProgress = $state(0)
  let cloneMessage = $state("")
  let errorMessage = $state("")
  let nameError = $state("")
  let versionChangeModal = $state(false)
  let versionChangeTarget = $state<{ repoId: number; newVersion: string; newTag: string } | null>(null)
  let isRefreshing = $state<Set<number>>(new Set())
  let openMenuId = $state<number | null>(null)
  let deleteConfirmModal = $state(false)
  let deleteTarget = $state<number | null>(null)
  let isChangingVersion = $state(false)

  let unlistenProgress: (() => void) | null = null
  let unlistenComplete: (() => void) | null = null

  function openModal() {
    showModal = true
    newRepoName = ""
    newRepoUrl = ""
    errorMessage = ""
    nameError = ""
  }

  function closeModal() {
    showModal = false
    isCloning = false
    cloneProgress = 0
    cloneMessage = ""
    errorMessage = ""
  }

  function openVersionChangeModal(repoId: number, newVersion: string, newTag: string) {
    versionChangeTarget = { repoId, newVersion, newTag }
    versionChangeModal = true
  }

  function closeVersionChangeModal() {
    if (isChangingVersion) return
    versionChangeModal = false
    versionChangeTarget = null
  }

  async function handleVersionChange() {
    if (!versionChangeTarget || isChangingVersion) return

    isChangingVersion = true
    const { repoId, newTag } = versionChangeTarget
    const result = await commands.changeVersion(repoId, newTag)

    if (result.status === "ok") {
      repositories = repositories.map(repo =>
        repo.id === repoId ? result.data : repo
      )
    }

    isChangingVersion = false
    closeVersionChangeModal()
  }

  async function handleRefresh(repoId: number) {
    isRefreshing = new Set([...isRefreshing, repoId])

    const result = await commands.refreshRepository(repoId)

    if (result.status === "ok") {
      repositories = repositories.map(repo =>
        repo.id === repoId ? result.data : repo
      )
    }

    isRefreshing = new Set([...isRefreshing].filter(id => id !== repoId))
  }

  function toggleMenu(repoId: number) {
    openMenuId = openMenuId === repoId ? null : repoId
  }

  function closeMenu() {
    openMenuId = null
  }

  function openDeleteConfirm(repoId: number) {
    deleteTarget = repoId
    deleteConfirmModal = true
    closeMenu()
  }

  function closeDeleteConfirm() {
    deleteConfirmModal = false
    deleteTarget = null
  }

  async function handleDelete() {
    if (deleteTarget === null) return

    const result = await commands.deleteRepository(deleteTarget)

    if (result.status === "ok") {
      repositories = repositories.filter(repo => repo.id !== deleteTarget)
    }

    closeDeleteConfirm()
  }

  function validateName(name: string): boolean {
    if (!name) {
      nameError = ""
      return false
    }
    const valid = /^[a-zA-Z0-9_-]+$/.test(name)
    if (!valid) {
      nameError = "Only letters, numbers, underscores, and dashes allowed"
      return false
    }
    nameError = ""
    return true
  }

  async function handleAddRepository() {
    if (!validateName(newRepoName)) {
      return
    }
    if (!newRepoUrl.trim()) {
      errorMessage = "Repository URL is required"
      return
    }

    isCloning = true
    cloneProgress = 0
    cloneMessage = "Starting..."
    errorMessage = ""

    try {
      const result = await commands.cloneRepository(newRepoUrl, newRepoName)

      if (result.status === "ok") {
        repositories = [...repositories, result.data]
        showModal = false
        newRepoName = ""
        newRepoUrl = ""
      } else {
        errorMessage = result.error
      }
    } catch (e) {
      errorMessage = String(e)
    } finally {
      isCloning = false
      cloneProgress = 0
      cloneMessage = ""
    }
  }

  onMount(async () => {
    await commands.loadState()
    repositories = await commands.getRepositories()

    unlistenProgress = await events.cloneProgressEvent.listen((e) => {
      console.log("[Frontend] Progress event:", e.payload)
      cloneProgress = e.payload.progress
      cloneMessage = e.payload.message
    })

    unlistenComplete = await events.cloneCompleteEvent.listen((e) => {
      console.log("[Frontend] Complete event:", e.payload)
      if (!e.payload.success && e.payload.error_message) {
        errorMessage = e.payload.error_message
      }
    })
  })

  onDestroy(() => {
    unlistenProgress?.()
    unlistenComplete?.()
  })
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

  <!-- 테이블 헤더 -->
  <div class="table-header">
    <div class="header-cell name-cell">Name</div>
    <div class="header-cell version-cell">Game Version</div>
    <div class="header-cell sync-cell">Last Sync</div>
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
        <select
          value={repo.gameVersion}
          onchange={(e) => {
            const target = e.target as HTMLSelectElement
            const index = repo.gameVersions.indexOf(target.value)
            if (index !== -1 && repo.serverOptions[index]) {
              openVersionChangeModal(repo.id, target.value, repo.serverOptions[index])
            }
          }}
          class="version-select"
        >
          {#each repo.gameVersions as version}
            <option value={version}>{version}</option>
          {/each}
        </select>
      </div>
      <div class="cell sync-cell">
        <div class="sync-info">
          <button
            class="refresh-btn"
            onclick={() => handleRefresh(repo.id)}
            disabled={isRefreshing.has(repo.id)}
          >
            {isRefreshing.has(repo.id) ? "↻" : "Refresh"}
          </button>
          {#if repo.lastSyncTime}
            <span class="sync-time">{repo.lastSyncTime}</span>
          {/if}
        </div>
      </div>
      <div class="cell settings-cell">
        <button class="icon-btn" onclick={() => toggleMenu(repo.id)}>⋯</button>
        {#if openMenuId === repo.id}
          <div class="hamburger-menu">
            <button class="menu-item" onclick={() => openDeleteConfirm(repo.id)}>
              Delete Repository
            </button>
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>

<!-- 모달 -->
{#if showModal}
  <div class="modal-overlay" role="dialog" aria-modal="true" onclick={closeModal} onkeydown={(e) => e.key === 'Escape' && closeModal()}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
      <h2>Add Repository</h2>
      <form onsubmit={(e) => { e.preventDefault(); handleAddRepository(); }}>
        <div class="form-group">
          <label for="repo-name">Repository Name</label>
          <input
            id="repo-name"
            type="text"
            placeholder="e.g., MyProject (letters, numbers, _, - only)"
            bind:value={newRepoName}
            oninput={() => validateName(newRepoName)}
            disabled={isCloning}
            required
          />
          {#if nameError}
            <span class="field-error">{nameError}</span>
          {/if}
        </div>
        <div class="form-group">
          <label for="repo-url">Repository URL</label>
          <input
            id="repo-url"
            type="text"
            placeholder="e.g., https://github.com/user/repo.git"
            bind:value={newRepoUrl}
            disabled={isCloning}
            required
          />
        </div>

        {#if isCloning}
          <div class="clone-progress">
            <div class="progress-bar">
              <div class="progress-fill" style="width: {cloneProgress}%"></div>
            </div>
            <span class="progress-text">{cloneMessage} ({cloneProgress}%)</span>
          </div>
        {/if}

        {#if errorMessage}
          <div class="error-message">{errorMessage}</div>
        {/if}

        <div class="modal-actions">
          <button type="button" class="btn-secondary" onclick={closeModal}>
            Cancel
          </button>
          <button type="submit" class="btn-primary" disabled={isCloning || !!nameError}>
            {isCloning ? "Cloning..." : "Add"}
          </button>
        </div>
      </form>
    </div>
  </div>
{/if}

<!-- 삭제 확인 모달 -->
{#if deleteConfirmModal}
  <div class="modal-overlay" role="dialog" aria-modal="true" onclick={closeDeleteConfirm} onkeydown={(e) => e.key === 'Escape' && closeDeleteConfirm()}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
      <h2>Delete Repository</h2>
      <p class="warning-message">로컬 폴더를 포함하여 리포지토리가 완전히 삭제됩니다. 계속하시겠습니까?</p>
      <div class="modal-actions">
        <button class="btn-secondary" onclick={closeDeleteConfirm}>Cancel</button>
        <button class="btn-primary" onclick={handleDelete}>Delete</button>
      </div>
    </div>
  </div>
{/if}

<!-- 버전 변경 확인 모달 -->
{#if versionChangeModal}
  <div class="modal-overlay" role="dialog" aria-modal="true" onclick={closeVersionChangeModal} onkeydown={(e) => e.key === 'Escape' && closeVersionChangeModal()}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
      <h2>Change Version</h2>
      {#if isChangingVersion}
        <div class="loading-state">
          <span class="spinner">↻</span>
          <p>버전을 변경하는 중입니다...</p>
        </div>
      {:else}
        <p class="warning-message">
          모든 변경사항이 사라집니다. 버전을 <strong>{versionChangeTarget?.newVersion}</strong>(으)로 변경하시겠습니까?
        </p>
      {/if}
      <div class="modal-actions">
        <button class="btn-secondary" onclick={closeVersionChangeModal} disabled={isChangingVersion}>Cancel</button>
        <button class="btn-primary" onclick={handleVersionChange} disabled={isChangingVersion}>
          {isChangingVersion ? "Changing..." : "Change Version"}
        </button>
      </div>
    </div>
  </div>
{/if}
