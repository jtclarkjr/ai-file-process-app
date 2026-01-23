import type { TokenUsage } from "../types";

/**
 * File content union type
 */
export type FileContent =
  | { type: "text"; text: string }
  | { type: "image"; data: Uint8Array; mediaType: string };

/**
 * AI request structure
 */
export interface AiRequest {
  content: FileContent;
  operation: string;
  customPrompt?: string | null;
  fileName?: string | null;
}

/**
 * AI response structure
 */
export interface AiResponse {
  result: string;
  model: string;
  usage: TokenUsage | null;
}

/**
 * Abstract AI provider interface
 */
export interface AiProvider {
  readonly name: string;
  readonly supportsVision: boolean;
  process(request: AiRequest): Promise<AiResponse>;
}
