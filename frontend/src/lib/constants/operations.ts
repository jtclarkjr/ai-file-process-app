import type { Operation, ResponseLanguage } from '$lib/types/fileProcessor';

export const OPERATIONS_BY_LANG: Record<
  ResponseLanguage,
  { id: Operation; label: string; description: string }[]
> = {
  en: [
    { id: 'summarize', label: 'Summarize', description: 'Get a concise summary' },
    { id: 'extract', label: 'Extract', description: 'Extract key information' },
    { id: 'analyze', label: 'Analyze', description: 'In-depth analysis' },
    { id: 'classify', label: 'Classify', description: 'Categorize the content' },
    { id: 'custom', label: 'Custom', description: 'Use your own prompt' },
  ],
  ja: [
    { id: 'summarize', label: '要約', description: '簡潔な要約を生成' },
    { id: 'extract', label: '抽出', description: '重要情報を抽出' },
    { id: 'analyze', label: '分析', description: '詳細な分析' },
    { id: 'classify', label: '分類', description: '内容を分類' },
    { id: 'custom', label: 'カスタム', description: '独自のプロンプトを使用' },
  ],
};
