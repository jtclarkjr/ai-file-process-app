export interface ProcessResult {
	result: string;
	model: string;
	file_type: string;
	original_size: number;
	processing_time_ms: number;
	usage: TokenUsage | null;
}

export interface TokenUsage {
	input_tokens: number;
	output_tokens: number;
}

export interface ProcessResponse {
	success: boolean;
	data: ProcessResult | null;
	error: string | null;
}

export interface ProviderInfo {
	id: string;
	name: string;
	available: boolean;
	supports_vision: boolean;
}

export interface SupportedTypeInfo {
	file_type: string;
	extensions: string[];
	description: string;
}

export type Operation = 'summarize' | 'extract' | 'analyze' | 'classify' | 'custom';
