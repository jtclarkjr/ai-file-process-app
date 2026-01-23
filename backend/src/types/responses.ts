import { z } from 'zod';
import { SupportedFileType } from './file-types';

/**
 * Response schemas and types
 */

export const TokenUsageSchema = z.object({
  inputTokens: z.number(),
  outputTokens: z.number(),
});

export type TokenUsage = z.infer<typeof TokenUsageSchema>;

export const ProcessResultSchema = z.object({
  result: z.string(),
  model: z.string(),
  fileType: z.nativeEnum(SupportedFileType),
  originalSize: z.number(),
  processingTimeMs: z.number(),
  usage: TokenUsageSchema.nullable(),
});

export type ProcessResult = z.infer<typeof ProcessResultSchema>;

export const ProcessResponseSchema = z.object({
  success: z.boolean(),
  data: ProcessResultSchema.nullable(),
  error: z.string().nullable(),
});

export type ProcessResponse = z.infer<typeof ProcessResponseSchema>;

export const SupportedTypeInfoSchema = z.object({
  fileType: z.nativeEnum(SupportedFileType),
  extensions: z.array(z.string()),
  description: z.string(),
});

export type SupportedTypeInfoResponse = z.infer<typeof SupportedTypeInfoSchema>;

export const SupportedTypesResponseSchema = z.object({
  success: z.boolean(),
  data: z.array(SupportedTypeInfoSchema),
});

export type SupportedTypesResponse = z.infer<
  typeof SupportedTypesResponseSchema
>;

export const ProviderInfoSchema = z.object({
  id: z.string(),
  name: z.string(),
  available: z.boolean(),
  supportsVision: z.boolean(),
});

export type ProviderInfo = z.infer<typeof ProviderInfoSchema>;

export const ProvidersResponseSchema = z.object({
  success: z.boolean(),
  data: z.array(ProviderInfoSchema),
});

export type ProvidersResponse = z.infer<typeof ProvidersResponseSchema>;

export const HealthResponseSchema = z.object({
  status: z.string(),
});

export type HealthResponse = z.infer<typeof HealthResponseSchema>;

export const ErrorResponseSchema = z.object({
  success: z.literal(false),
  error: z.string(),
  data: z.null(),
});

export type ErrorResponse = z.infer<typeof ErrorResponseSchema>;

export function errorResponse(message: string): ErrorResponse {
  return { success: false, error: message, data: null };
}
