<script lang="ts">
  import { page } from "$app/stores";
</script>

<div class="app">
  <!-- 사이드바 -->
  <aside class="sidebar">
    <div class="logo">
      <span class="logo-icon">📦</span>
      <span class="logo-text">Hub</span>
    </div>

    <nav class="nav">
      <a href="/" class="nav-item" class:active={$page.url.pathname === "/"}>
        <span class="nav-icon">📂</span>
        <span>Projects</span>
      </a>
    </nav>

    <div class="sidebar-footer">
      <a
        href="/settings"
        class="nav-item"
        class:active={$page.url.pathname === "/settings"}
      >
        <span class="nav-icon">⚙️</span>
        <span>Settings</span>
      </a>
    </div>
  </aside>

  <!-- 메인 컨텐츠 -->
  <main class="main-content">
    <slot />
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen,
      Ubuntu, Cantarell, sans-serif;
    background-color: #1a1a1a;
    color: #e0e0e0;
    font-size: 14px;
  }

  .app {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  /* 사이드바 */
  .sidebar {
    width: 220px;
    background-color: #252525;
    display: flex;
    flex-direction: column;
    border-right: 1px solid #333;
  }

  .logo {
    padding: 20px;
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 20px;
    font-weight: 600;
    border-bottom: 1px solid #333;
  }

  .logo-icon {
    font-size: 24px;
  }

  .nav {
    flex: 1;
    padding: 10px 0;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 20px;
    color: #b0b0b0;
    text-decoration: none;
    transition: all 0.2s;
    cursor: pointer;
  }

  .nav-item:hover {
    background-color: #2d2d2d;
    color: #e0e0e0;
  }

  .nav-item.active {
    background-color: #2d2d2d;
    color: #fff;
    border-left: 3px solid #4a9eff;
  }

  .nav-icon {
    font-size: 18px;
    width: 24px;
    text-align: center;
  }

  .sidebar-footer {
    border-top: 1px solid #333;
    padding: 10px 0;
  }

  /* 메인 컨텐츠 */
  .main-content {
    flex: 1;
    background-color: #1a1a1a;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
  }
</style>
