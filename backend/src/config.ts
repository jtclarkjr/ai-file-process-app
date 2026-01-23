/**
 * Environment configuration
 */
export const config = {
  // AI Provider settings
  openaiApiKey: process.env.OPENAI_API_KEY,
  openaiModel: process.env.OPENAI_MODEL ?? "gpt-4-turbo",
  anthropicApiKey: process.env.ANTHROPIC_API_KEY,
  anthropicModel: process.env.ANTHROPIC_MODEL ?? "claude-3-sonnet-20240229",

  // Server settings
  host: process.env.HOST ?? "0.0.0.0",
  port: Number(process.env.PORT ?? 8080),
  logLevel: process.env.LOG_LEVEL ?? "info",

  // File processing limits
  maxFileSizeMb: Number(process.env.MAX_FILE_SIZE_MB ?? 10),
  requestTimeoutSecs: Number(process.env.REQUEST_TIMEOUT_SECS ?? 120),
  aiTimeoutSecs: Number(process.env.AI_TIMEOUT_SECS ?? 60),

  // Decompression bomb protection
  maxDecompressionRatio: 100,
  maxDecompressedSizeMb: 50,

  // Computed values
  get maxFileSizeBytes() {
    return this.maxFileSizeMb * 1024 * 1024;
  },
  get maxDecompressedSizeBytes() {
    return this.maxDecompressedSizeMb * 1024 * 1024;
  },
  get aiTimeoutMs() {
    return this.aiTimeoutSecs * 1000;
  },
} as const;

export type Config = typeof config;
