<script lang="ts">
  import { fileProcessorStore } from '$lib/stores/fileProcessor.svelte'
  import { TRANSLATIONS } from '$lib/constants/translations'
  import { onDestroy, onMount } from 'svelte'
  import { Copy, Maximize2, X } from '@lucide/svelte'

  let copyTooltip = 'Copy'
  let resetTooltipTimeout: ReturnType<typeof setTimeout> | undefined
  let expanded = $state(false)
  let hasCopied = $state(false)
  const text = $derived(
    TRANSLATIONS[fileProcessorStore.selectedLanguage] ?? TRANSLATIONS.en
  )

  $effect(() => {
    if (!hasCopied) {
      copyTooltip = text.copy
    }
  })

  const copyToClipboard = () => {
    if (fileProcessorStore.result?.result) {
      navigator.clipboard.writeText(fileProcessorStore.result.result)
      hasCopied = true
      copyTooltip = text.copied
      if (resetTooltipTimeout) {
        clearTimeout(resetTooltipTimeout)
      }
      resetTooltipTimeout = setTimeout(() => {
        hasCopied = false
        copyTooltip = text.copy
      }, 2000)
    }
  }

  onDestroy(() => {
    if (resetTooltipTimeout) {
      clearTimeout(resetTooltipTimeout)
    }
  })

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Escape' && expanded) {
      closeExpanded()
    }
  }

  onMount(() => {
    window.addEventListener('keydown', handleKeydown)
    return () => window.removeEventListener('keydown', handleKeydown)
  })

  const openExpanded = () => {
    expanded = true
  }

  const closeExpanded = () => {
    expanded = false
  }
</script>

{#if fileProcessorStore.error}
  <div class="error">
    <p>{fileProcessorStore.error}</p>
  </div>
{/if}

{#if fileProcessorStore.result}
  <div class="result">
    <div class="result-header">
      <h3>{text.result}</h3>
      <div class="result-actions">
        <button
          type="button"
          class="expand-btn"
          onclick={openExpanded}
          aria-label={text.expand}
          title={text.expand}
        >
          <span class="expand-icon" aria-hidden="true">
            <Maximize2 size={18} />
          </span>
        </button>
        <button
          type="button"
          class="copy-btn"
          onclick={copyToClipboard}
          aria-label={copyTooltip}
          title={copyTooltip}
          data-tooltip={copyTooltip}
        >
          <span class="copy-icon" aria-hidden="true">
            <Copy size={18} />
          </span>
        </button>
      </div>
    </div>

    <div class="result-content">
      <pre>{fileProcessorStore.result.result}</pre>
    </div>
  </div>
{/if}

{#if expanded && fileProcessorStore.result}
  <div class="result-overlay" role="dialog" aria-modal="true">
    <button
      class="overlay-backdrop"
      type="button"
      onclick={closeExpanded}
      aria-label={text.close}
    >
    </button>
    <div class="overlay-panel">
      <div class="overlay-header">
        <h3>{text.result}</h3>
        <div class="overlay-actions">
          <button
            type="button"
            class="copy-btn"
            onclick={copyToClipboard}
            aria-label={copyTooltip}
            title={copyTooltip}
            data-tooltip={copyTooltip}
          >
            <span class="copy-icon" aria-hidden="true">
              <Copy size={18} />
            </span>
          </button>
          <button
            type="button"
            class="close-btn"
            onclick={closeExpanded}
            aria-label={text.close}
            title={text.close}
          >
            <span class="close-icon" aria-hidden="true">
              <X size={20} />
            </span>
          </button>
        </div>
      </div>
      <div class="overlay-content">
        <pre>{fileProcessorStore.result.result}</pre>
      </div>
    </div>
  </div>
{/if}

<style>
  .error {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 15px 20px;
    background: #fef2f2;
    border: 1px solid #ef4444;
    border-radius: 8px;
    color: #b91c1c;
  }

  .error-icon {
    font-size: 1.25rem;
  }

  .error p {
    margin: 0;
  }

  .result {
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 8px;
    overflow: hidden;
  }

  .result-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 15px 20px;
    background: #f9fafb;
    border-bottom: 1px solid #e5e7eb;
  }

  .result-header h3 {
    margin: 0;
    font-size: 1rem;
    color: #374151;
  }

  .copy-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 34px;
    background: #6366f1;
    color: white;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    padding: 0;
  }

  .result-actions {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .expand-btn,
  .close-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 34px;
    height: 34px;
    border: none;
    border-radius: 6px;
    background: #e5e7eb;
    color: #374151;
    cursor: pointer;
    padding: 0;
  }

  .expand-btn:hover,
  .close-btn:hover {
    background: #d1d5db;
  }

  .copy-btn:hover {
    background: #4f46e5;
  }

  .copy-btn::after {
    content: attr(data-tooltip);
    position: absolute;
    right: 0;
    top: 100%;
    margin-top: 6px;
    padding: 4px 8px;
    background: #111827;
    color: #f9fafb;
    font-size: 0.75rem;
    border-radius: 4px;
    opacity: 0;
    pointer-events: none;
    transform: translateY(-4px);
    transition:
      opacity 120ms ease,
      transform 120ms ease;
    white-space: nowrap;
  }

  .copy-btn:hover::after,
  .copy-btn:focus-visible::after {
    opacity: 1;
    transform: translateY(0);
  }

  .copy-icon {
    display: block;
    color: currentColor;
  }

  .result-content {
    padding: 20px;
    max-height: 400px;
    overflow-y: auto;
  }

  .result-content pre {
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
    font-family:
      ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
    font-size: 0.875rem;
    line-height: 1.6;
    color: #374151;
  }

  .result-overlay {
    position: fixed;
    inset: 0;
    z-index: 50;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
  }

  .overlay-backdrop {
    position: absolute;
    inset: 0;
    background: rgba(15, 23, 42, 0.6);
    border: none;
    padding: 0;
  }

  .overlay-panel {
    position: relative;
    width: min(1100px, 100%);
    max-height: calc(100vh - 48px);
    background: #0f172a;
    color: #e2e8f0;
    border-radius: 12px;
    box-shadow: 0 20px 60px rgba(15, 23, 42, 0.5);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .overlay-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(148, 163, 184, 0.2);
  }

  .overlay-header h3 {
    margin: 0;
    font-size: 1rem;
    color: #e2e8f0;
  }

  .overlay-actions {
    display: inline-flex;
    align-items: center;
    gap: 8px;
  }

  .overlay-actions .copy-btn {
    background: #6366f1;
  }

  .overlay-actions .copy-btn:hover {
    background: #4f46e5;
  }

  .overlay-actions .close-btn {
    background: rgba(148, 163, 184, 0.2);
    color: #e2e8f0;
  }

  .overlay-actions .close-btn:hover {
    background: rgba(148, 163, 184, 0.35);
  }

  .overlay-content {
    padding: 24px;
    overflow: auto;
    flex: 1;
  }

  .overlay-content pre {
    margin: 0;
    white-space: pre-wrap;
    word-wrap: break-word;
    font-family:
      ui-monospace, SFMono-Regular, 'SF Mono', Menlo, Consolas, monospace;
    font-size: 0.95rem;
    line-height: 1.7;
    color: #e2e8f0;
  }

  @media (max-width: 640px) {
    .result-overlay {
      padding: 12px;
    }

    .overlay-panel {
      max-height: calc(100vh - 24px);
    }
  }
</style>
