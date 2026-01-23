<script lang="ts">
	import { fileProcessorStore } from '$lib/stores/fileProcessor.svelte';
import { Play, Trash2 } from '@lucide/svelte';

	let dragOver = $state(false);
	let fileInput: HTMLInputElement;

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		dragOver = true;
	}

	function handleDragLeave(e: DragEvent) {
		e.preventDefault();
		dragOver = false;
	}

	function handleDrop(e: DragEvent) {
		e.preventDefault();
		dragOver = false;

		const files = e.dataTransfer?.files;
		if (files && files.length > 0) {
			fileProcessorStore.setFile(files[0]);
			if (fileInput) {
				fileInput.value = '';
			}
		}
	}

	function handleFileSelect(e: Event) {
		const input = e.target as HTMLInputElement;
		if (input.files && input.files.length > 0) {
			fileProcessorStore.setFile(input.files[0]);
		}
	}

	function triggerFileSelect() {
		fileInput.click();
	}

	function clearFile() {
		fileProcessorStore.setFile(null);
		if (fileInput) {
			fileInput.value = '';
		}
	}

	function handleProcess() {
		if (fileProcessorStore.canProcess) {
			fileProcessorStore.processFile();
		}
	}

	function formatFileSize(bytes: number): string {
		if (bytes < 1024) return `${bytes} B`;
		if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
		return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
	}
</script>

<div class="file-upload">
	{#if fileProcessorStore.selectedFile}
		<div
			class="selected-file"
			class:drag-over={dragOver}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			ondrop={handleDrop}
			role="region"
			aria-label="Selected file"
		>
			<div class="file-info">
				<span class="file-icon">📄</span>
				<div class="file-details">
					<span class="file-name">{fileProcessorStore.selectedFile.name}</span>
					<span class="file-size"
						>{formatFileSize(fileProcessorStore.selectedFile.size)}</span
					>
				</div>
			</div>
			<div class="file-actions">
				<span
					class="action-icon process-icon"
					class:disabled={!fileProcessorStore.canProcess}
					role="button"
					tabindex="0"
					aria-label="Process file"
					aria-disabled={!fileProcessorStore.canProcess}
					onclick={handleProcess}
					onkeydown={(e) => e.key === 'Enter' && handleProcess()}
				>
					<span class="icon-mark" aria-hidden="true">
						<Play size={18} />
					</span>
				</span>
				<span
					class="action-icon clear-icon"
					role="button"
					tabindex="0"
					aria-label="Remove file"
					onclick={clearFile}
					onkeydown={(e) => e.key === 'Enter' && clearFile()}
				>
					<span class="icon-mark" aria-hidden="true">
						<Trash2 size={18} />
					</span>
				</span>
			</div>
		</div>
	{:else}
		<div
			class="drop-zone"
			class:drag-over={dragOver}
			ondragover={handleDragOver}
			ondragleave={handleDragLeave}
			ondrop={handleDrop}
			onclick={triggerFileSelect}
			onkeydown={(e) => e.key === 'Enter' && triggerFileSelect()}
			role="button"
			tabindex="0"
		>
			<div class="drop-content">
				<span class="upload-icon">📁</span>
				<p class="drop-text">Drag & drop a file here, or click to browse</p>
				<p class="supported-text">
					Supported: PDF, DOCX, TXT, MD, images (JPG, PNG, GIF, WebP), code files
				</p>
			</div>
		</div>
	{/if}

	<input
		type="file"
		bind:this={fileInput}
		onchange={handleFileSelect}
		accept={fileProcessorStore.acceptedExtensions}
		class="hidden-input"
	/>
</div>

<style>
	.file-upload {
		width: 100%;
	}

	.drop-zone {
		border: 2px dashed #ddd;
		border-radius: 8px;
		padding: 40px 20px;
		text-align: center;
		cursor: pointer;
		transition: all 0.2s ease;
		background: #fafafa;
	}

	.drop-zone:hover,
	.drop-zone.drag-over {
		border-color: #2563eb;
		background: #eff6ff;
	}

	.drop-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
	}

	.upload-icon {
		font-size: 3rem;
	}

	.drop-text {
		margin: 0;
		font-size: 1rem;
		color: #333;
	}

	.supported-text {
		margin: 0;
		font-size: 0.875rem;
		color: #888;
	}

	.selected-file {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 15px 20px;
		background: #f0fdf4;
		border: 1px solid #22c55e;
		border-radius: 8px;
	}

	.selected-file.drag-over {
		border-color: #2563eb;
		background: #eff6ff;
	}

	.file-info {
		display: flex;
		align-items: center;
		gap: 12px;
	}

	.file-icon {
		font-size: 2rem;
	}

	.file-details {
		display: flex;
		flex-direction: column;
	}

	.file-name {
		font-weight: 500;
		color: #333;
		word-break: break-all;
	}

	.file-size {
		font-size: 0.875rem;
		color: #666;
	}

	.file-actions {
		display: inline-flex;
		align-items: center;
		gap: 8px;
	}

	.action-icon {
		padding: 8px;
		color: white;
		border-radius: 6px;
		cursor: pointer;
		font-size: 0.875rem;
		display: inline-flex;
		align-items: center;
		justify-content: center;
	}

	.action-icon.disabled {
		opacity: 0.5;
		cursor: not-allowed;
		pointer-events: none;
	}

	.process-icon {
		background: #6366f1;
	}

	.process-icon:hover {
		background: #4f46e5;
	}

	.clear-icon {
		background: #ef4444;
	}

	.clear-icon:hover {
		background: #dc2626;
	}

	.icon-mark {
		display: block;
		color: currentColor;
	}

	.hidden-input {
		display: none;
	}
</style>
