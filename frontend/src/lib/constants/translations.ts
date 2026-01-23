import type { ResponseLanguage } from '$lib/types/fileProcessor';

export const TRANSLATIONS: Record<
	ResponseLanguage,
	{
		appTitle: string;
		appSubtitle: string;
		privacyLabel: string;
		privacyBody: string;
		selectFile: string;
		configureProcessing: string;
		aiProvider: string;
		operation: string;
		language: string;
		supportsImages: string;
		customPromptLabel: string;
		customPromptPlaceholder: string;
		notConfigured: string;
		toggleSection: string;
		result: string;
		expand: string;
		copy: string;
		copied: string;
		close: string;
		dropText: string;
		supportedText: string;
		removeFile: string;
		processFile: string;
		selectedFileRegion: string;
		processingTitle: string;
	}
> = {
	en: {
		appTitle: 'AI File Processing',
		appSubtitle: 'Upload a file and let AI analyze it for you',
		privacyLabel: 'Privacy First:',
		privacyBody:
			'Your files are processed in-memory only and immediately discarded. No files are stored on our servers.',
		selectFile: 'Select a File',
		configureProcessing: 'Configure Processing',
		aiProvider: 'AI Provider',
		operation: 'Operation',
		language: 'Language',
		supportsImages: 'Supports Images',
		customPromptLabel: 'Custom Prompt',
		customPromptPlaceholder: 'Enter your custom instructions for processing this file...',
		notConfigured: 'Not configured',
		toggleSection: 'Toggle section',
		result: 'Result',
		expand: 'Expand',
		copy: 'Copy',
		copied: 'Copied',
		close: 'Close',
		dropText: 'Drag & drop a file here, or click to browse',
		supportedText: 'Supported: PDF, DOCX, TXT, MD, images (JPG, PNG, GIF, WebP), code files',
		removeFile: 'Remove file',
		processFile: 'Process file',
		selectedFileRegion: 'Selected file',
		processingTitle: 'Processing your file...',
	},
	ja: {
		appTitle: 'AIファイル処理',
		appSubtitle: 'ファイルをアップロードしてAIに解析させます',
		privacyLabel: 'プライバシー重視:',
		privacyBody:
			'ファイルはメモリ内でのみ処理され、直ちに破棄されます。サーバーには保存されません。',
		selectFile: 'ファイルを選択',
		configureProcessing: '処理を設定',
		aiProvider: 'AIプロバイダー',
		operation: '操作',
		language: '言語',
		supportsImages: '画像対応',
		customPromptLabel: 'カスタムプロンプト',
		customPromptPlaceholder: 'このファイルを処理するための指示を入力してください...',
		notConfigured: '未設定',
		toggleSection: 'セクションの開閉',
		result: '結果',
		expand: '拡大',
		copy: 'コピー',
		copied: 'コピーしました',
		close: '閉じる',
		dropText: 'ここにファイルをドラッグ＆ドロップ、またはクリックして選択',
		supportedText: '対応: PDF, DOCX, TXT, MD, 画像 (JPG, PNG, GIF, WebP), コードファイル',
		removeFile: 'ファイルを削除',
		processFile: 'ファイルを処理',
		selectedFileRegion: '選択済みファイル',
		processingTitle: 'ファイルを処理中...',
	},
};
