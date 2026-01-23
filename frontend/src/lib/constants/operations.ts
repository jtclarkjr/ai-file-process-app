import type { Operation } from '$lib/types/fileProcessor';

export const OPERATIONS: { id: Operation; label: string; description: string; }[] = [
	{ id: 'summarize', label: 'Summarize', description: 'Get a concise summary' },
	{ id: 'extract', label: 'Extract', description: 'Extract key information' },
	{ id: 'analyze', label: 'Analyze', description: 'In-depth analysis' },
	{ id: 'classify', label: 'Classify', description: 'Categorize the content' },
	{ id: 'custom', label: 'Custom', description: 'Use your own prompt' },
];
