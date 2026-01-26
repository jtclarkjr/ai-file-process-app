<script lang="ts">
  import { onMount } from 'svelte'
  import { fileProcessorStore } from '$lib/stores/fileProcessor.svelte'
  import FileUpload from '$lib/components/FileUpload.svelte'
  import ProcessingProgress from '$lib/components/ProcessingProgress.svelte'
  import ResultDisplay from '$lib/components/ResultDisplay.svelte'
  import { ChevronDown } from '@lucide/svelte'
  import { OPERATIONS_BY_LANG } from '$lib/constants/operations'
  import { LANGUAGES } from '$lib/constants/languages'
  import { TRANSLATIONS } from '$lib/constants/translations'

  onMount(() => {
    fileProcessorStore.bootstrap()
    const dismissed = localStorage.getItem('privacyNoticeDismissed')
    if (dismissed === 'true') {
      showPrivacyNotice = false
    }
  })

  const dismissPrivacyNotice = () => {
    showPrivacyNotice = false
    localStorage.setItem('privacyNoticeDismissed', 'true')
  }

  let section1Open = $state(true)
  let section2Open = $state(true)
  let showPrivacyNotice = $state(true)

  const text = $derived(
    TRANSLATIONS[fileProcessorStore.selectedLanguage] ?? TRANSLATIONS.en
  )
  const operations = $derived(
    OPERATIONS_BY_LANG[fileProcessorStore.selectedLanguage] ??
      OPERATIONS_BY_LANG.en
  )
</script>

<svelte:head>
  <title>{text.appTitle}</title>
</svelte:head>

<div class="container">
  <div class="top-bar">
    <div></div>
    <div class="language-toggle">
      <label class="sr-only" for="language">{text.language}</label>
      <select
        id="language"
        bind:value={fileProcessorStore.selectedLanguage}
        disabled={fileProcessorStore.processing}
      >
        {#each LANGUAGES as lang}
          <option value={lang.id}>{lang.label}</option>
        {/each}
      </select>
    </div>
  </div>
  <header>
    <h1>{text.appTitle}</h1>
    <p class="subtitle">{text.appSubtitle}</p>
  </header>

  {#if showPrivacyNotice}
    <div class="privacy-notice">
      <p>
        <strong>{text.privacyLabel}</strong>
        {text.privacyBody}
      </p>
      <button
        type="button"
        class="privacy-close"
        onclick={dismissPrivacyNotice}
      >
        ×
      </button>
    </div>
  {/if}

  <section class="upload-section" class:collapsed={!section1Open}>
    <div class="section-header">
      <h2>{text.selectFile}</h2>
      <button
        type="button"
        class="section-toggle"
        aria-expanded={section1Open}
        aria-label={`${text.toggleSection} 1`}
        onclick={() => (section1Open = !section1Open)}
      >
        <span class="chevron" class:open={section1Open} aria-hidden="true">
          <ChevronDown />
        </span>
      </button>
    </div>
    {#if section1Open}
      <FileUpload />
    {/if}
  </section>

  <section class="options-section" class:collapsed={!section2Open}>
    <div class="section-header">
      <h2>{text.configureProcessing}</h2>
      <button
        type="button"
        class="section-toggle"
        aria-expanded={section2Open}
        aria-label={`${text.toggleSection} 2`}
        onclick={() => (section2Open = !section2Open)}
      >
        <span class="chevron" class:open={section2Open} aria-hidden="true">
          <ChevronDown />
        </span>
      </button>
    </div>

    {#if section2Open}
      <div class="options-grid">
        <div class="option-group">
          <label for="provider">{text.aiProvider}</label>
          <select
            id="provider"
            bind:value={fileProcessorStore.selectedProvider}
            disabled={fileProcessorStore.processing}
          >
            {#each fileProcessorStore.providers as provider}
              <option value={provider.id} disabled={!provider.available}>
                {provider.name}
                {#if !provider.available}({text.notConfigured}){/if}
              </option>
            {/each}
          </select>
          {#if fileProcessorStore.currentProvider?.supports_vision}
            <span class="vision-badge">{text.supportsImages}</span>
          {/if}
        </div>

        <div class="option-group">
          <label for="operation">{text.operation}</label>
          <select
            id="operation"
            bind:value={fileProcessorStore.selectedOperation}
            disabled={fileProcessorStore.processing}
          >
            {#each operations as op}
              <option value={op.id}>{op.label} - {op.description}</option>
            {/each}
          </select>
        </div>
      </div>

      {#if fileProcessorStore.selectedOperation === 'custom'}
        <div class="custom-prompt">
          <label for="customPrompt">{text.customPromptLabel}</label>
          <textarea
            id="customPrompt"
            bind:value={fileProcessorStore.customPrompt}
            placeholder={text.customPromptPlaceholder}
            rows="3"
            disabled={fileProcessorStore.processing}
          ></textarea>
        </div>
      {/if}
    {/if}
  </section>

  <section class="result-section">
    <ProcessingProgress />
    <ResultDisplay />
  </section>
</div>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
  }

  .top-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 10px;
  }

  .language-toggle select {
    padding: 6px 10px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 0.875rem;
    background: white;
    color: #374151;
  }

  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 12px;
  }

  .section-header h2 {
    margin: 0;
  }

  .section-toggle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    background: transparent;
    border: none;
    font-size: 0.875rem;
    color: #374151;
    cursor: pointer;
  }

  .section-toggle:hover {
    color: #111827;
  }

  section.collapsed {
    padding: 12px 20px;
  }

  section.collapsed .section-header {
    margin-bottom: 0;
  }

  .chevron {
    display: inline-block;
    transition: transform 160ms ease;
    color: currentColor;
  }

  .chevron.open {
    transform: rotate(180deg);
  }

  header {
    text-align: center;
    margin-bottom: 30px;
  }

  header h1 {
    color: #333;
    margin-bottom: 8px;
    border-bottom: 2px solid #ff3e00;
    padding-bottom: 10px;
    display: inline-block;
  }

  .subtitle {
    color: #666;
    margin: 0;
  }

  .privacy-notice {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    padding: 15px 20px;
    background: #ecfdf5;
    border: 1px solid #10b981;
    border-radius: 8px;
    margin-bottom: 30px;
    position: relative;
  }

  .privacy-notice p {
    margin: 0;
    color: #065f46;
    font-size: 0.875rem;
  }

  .privacy-close {
    position: absolute;
    top: 8px;
    right: 8px;
    width: 28px;
    height: 28px;
    border-radius: 50%;
    border: none;
    background: transparent;
    color: #065f46;
    font-size: 1.25rem;
    line-height: 1;
    cursor: pointer;
  }

  .privacy-close:hover {
    background: rgba(6, 95, 70, 0.12);
  }

  section {
    background: white;
    padding: 20px;
    border-radius: 8px;
    margin-bottom: 20px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  }

  section h2 {
    color: #555;
    font-size: 1.1rem;
    margin: 0 0 15px 0;
  }

  .options-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 20px;
  }

  .option-group {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .option-group label {
    font-weight: 500;
    color: #374151;
    font-size: 0.875rem;
  }

  .option-group select {
    padding: 10px 12px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 1rem;
    background: white;
  }

  .option-group select:disabled {
    background: #f3f4f6;
    cursor: not-allowed;
  }

  .vision-badge {
    display: inline-block;
    padding: 4px 8px;
    background: #dbeafe;
    color: #1e40af;
    border-radius: 4px;
    font-size: 0.75rem;
    font-weight: 500;
  }

  .custom-prompt {
    margin-top: 20px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .custom-prompt label {
    font-weight: 500;
    color: #374151;
    font-size: 0.875rem;
  }

  .custom-prompt textarea {
    padding: 12px;
    border: 1px solid #d1d5db;
    border-radius: 6px;
    font-size: 1rem;
    font-family: inherit;
    resize: vertical;
  }

  .custom-prompt textarea:disabled {
    background: #f3f4f6;
    cursor: not-allowed;
  }

  .result-section {
    background: transparent;
    box-shadow: none;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 20px;
  }
</style>
