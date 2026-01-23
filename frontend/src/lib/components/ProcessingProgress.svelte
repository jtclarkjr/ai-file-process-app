<script lang="ts">
  import { fileProcessorStore } from '$lib/stores/fileProcessor.svelte'
</script>

{#if fileProcessorStore.processing}
  <div class="processing">
    <div class="spinner"></div>
    <div class="processing-text">
      <p class="processing-title">Processing your file...</p>
      <p class="processing-details">
        Using {fileProcessorStore.currentProvider?.name || 'AI'} to {fileProcessorStore.selectedOperation}
        your content
      </p>
    </div>
  </div>
{/if}

<style>
  .processing {
    display: flex;
    align-items: center;
    gap: 20px;
    padding: 20px;
    background: #f0f9ff;
    border: 1px solid #0ea5e9;
    border-radius: 8px;
  }

  .spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #e0f2fe;
    border-top-color: #0ea5e9;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .processing-text {
    flex: 1;
  }

  .processing-title {
    margin: 0 0 4px 0;
    font-weight: 500;
    color: #0369a1;
  }

  .processing-details {
    margin: 0;
    font-size: 0.875rem;
    color: #0284c7;
  }
</style>
