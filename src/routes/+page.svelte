<script lang="ts">
  import { onMount, onDestroy } from "svelte"
  import { events, commands, type RepositoryInfo } from "$lib/bindings"
  import { revealItemInDir } from "@tauri-apps/plugin-opener"
  import { _, isLoading } from "svelte-i18n"
  import "$lib/i18n"
  import "./page.css"

  let repositories = $state<RepositoryInfo[]>([])
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

  async function handleOpenFolder(path: string) {
    closeMenu()
    await revealItemInDir(path)
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
      nameError = $_("errors.nameInvalid")
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
      errorMessage = $_("errors.urlRequired")
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

{#if $isLoading}
  <div class="loading">Loading...</div>
{:else}
  <!-- 헤더 -->
  <header class="header">
    <div class="header-main">
      <h1>{$_("header.projects")}</h1>
      <div class="header-controls">
        <button class="btn-primary" onclick={openModal}>
          {$_("header.addRepository")}
        </button>
      </div>
    </div>

    <!-- 테이블 헤더 -->
    <div class="table-header">
      <div class="header-cell name-cell">{$_("table.name")}</div>
      <div class="header-cell version-cell">{$_("table.gameVersion")}</div>
      <div class="header-cell sync-cell">{$_("table.lastSync")}</div>
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
              {isRefreshing.has(repo.id) ? "↻" : $_("actions.refresh")}
            </button>
            {#if repo.lastSyncTime}
              <span class="sync-time">{repo.lastSyncTime}</span>
            {/if}
          </div>
        </div>
        <div class="cell settings-cell">
          <button class="icon-btn" onclick={() => toggleMenu(repo.id)}>☰</button>
          {#if openMenuId === repo.id}
            <div class="hamburger-menu">
              <button class="menu-item" onclick={() => handleOpenFolder(repo.path)}>
                {$_("actions.openFolder")}
              </button>
              <div class="menu-divider"></div>
              <button class="menu-item danger" onclick={() => openDeleteConfirm(repo.id)}>
                {$_("actions.deleteRepository")}
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
        <h2>{$_("modal.addRepository")}</h2>
        <form onsubmit={(e) => { e.preventDefault(); handleAddRepository(); }}>
          <div class="form-group">
            <label for="repo-name">{$_("modal.repositoryName")}</label>
            <input
              id="repo-name"
              type="text"
              placeholder={$_("modal.namePlaceholder")}
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
            <label for="repo-url">{$_("modal.repositoryUrl")}</label>
            <input
              id="repo-url"
              type="text"
              placeholder={$_("modal.urlPlaceholder")}
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
              {$_("actions.cancel")}
            </button>
            <button type="submit" class="btn-primary" disabled={isCloning || !!nameError}>
              {isCloning ? $_("actions.cloning") : $_("actions.add")}
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
        <h2>{$_("deleteModal.title")}</h2>
        <p class="warning-message">{$_("deleteModal.warning")}</p>
        <div class="modal-actions">
          <button class="btn-secondary" onclick={closeDeleteConfirm}>{$_("actions.cancel")}</button>
          <button class="btn-primary" onclick={handleDelete}>{$_("actions.delete")}</button>
        </div>
      </div>
    </div>
  {/if}

  <!-- 버전 변경 확인 모달 -->
  {#if versionChangeModal}
    <div class="modal-overlay" role="dialog" aria-modal="true" onclick={closeVersionChangeModal} onkeydown={(e) => e.key === 'Escape' && closeVersionChangeModal()}>
      <div class="modal-content" onclick={(e) => e.stopPropagation()} onkeydown={(e) => e.stopPropagation()}>
        <h2>{$_("versionModal.title")}</h2>
        {#if isChangingVersion}
          <div class="loading-state">
            <span class="spinner">↻</span>
            <p>{$_("versionModal.loading")}</p>
          </div>
        {:else}
          <p class="warning-message">
            {$_("versionModal.warning", { values: { version: versionChangeTarget?.newVersion ?? "" } })}
          </p>
        {/if}
        <div class="modal-actions">
          <button class="btn-secondary" onclick={closeVersionChangeModal} disabled={isChangingVersion}>{$_("actions.cancel")}</button>
          <button class="btn-primary" onclick={handleVersionChange} disabled={isChangingVersion}>
            {isChangingVersion ? $_("actions.changing") : $_("actions.changeVersion")}
          </button>
        </div>
      </div>
    </div>
  {/if}
{/if}
