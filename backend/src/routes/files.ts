import { Hono } from 'hono';
import type { AiProvider, FileExtractor } from '../services';
import {
  DecompressionBombError,
  FileExtractionError,
  FileTooLargeError,
  MimeTypeMismatchError,
  UnsupportedFileTypeError,
} from '../services';
import {
  errorResponse,
  getAllSupported,
  isImage,
  type ProcessResponse,
  type ProvidersResponse,
  type SupportedTypesResponse,
} from '../types';

interface FilesContext {
  Variables: {
    openaiProvider: AiProvider | null;
    anthropicProvider: AiProvider | null;
    fileExtractor: FileExtractor;
  };
}

const files = new Hono<FilesContext>();

/**
 * POST /files/process - Process a file with AI
 */
files.post('/process', async (c) => {
  const startTime = performance.now();

  try {
    const openai = c.get('openaiProvider');
    const anthropic = c.get('anthropicProvider');
    const extractor = c.get('fileExtractor');

    // Parse query params
    const url = new URL(c.req.url);
    const provider = url.searchParams.get('provider');
    const operation = url.searchParams.get('operation');
    const customPrompt = url.searchParams.get('custom_prompt');
    const language = url.searchParams.get('language');

    if (!provider || !operation) {
      return c.json(
        errorResponse('Missing required query params: provider, operation'),
        400,
      );
    }

    // Get the file from form data
    const formData = await c.req.formData();
    const file = formData.get('file');

    if (!file || !(file instanceof File)) {
      return c.json(errorResponse('No file provided'), 400);
    }

    const fileData = new Uint8Array(await file.arrayBuffer());
    const fileName = file.name;
    const contentType = file.type || null;

    console.log(
      `Processing file: ${fileName} (${fileData.length} bytes) with provider=${provider}, operation=${operation}`,
    );

    // Validate and detect file type
    const fileType = await extractor.validateAndDetectType(
      fileData,
      contentType,
      fileName,
    );

    // Get the appropriate provider
    let aiProvider: AiProvider;
    if (provider.toLowerCase() === 'openai') {
      if (!openai) {
        return c.json(errorResponse('OpenAI provider not configured'), 503);
      }
      aiProvider = openai;
    } else if (provider.toLowerCase() === 'anthropic') {
      if (!anthropic) {
        return c.json(errorResponse('Anthropic provider not configured'), 503);
      }
      aiProvider = anthropic;
    } else {
      return c.json(
        errorResponse(
          `Unknown provider: ${provider}. Valid options: openai, anthropic`,
        ),
        400,
      );
    }

    // Check if provider supports images when needed
    if (isImage(fileType) && !aiProvider.supportsVision) {
      return c.json(
        errorResponse(
          `Provider ${aiProvider.name} does not support vision/image processing`,
        ),
        400,
      );
    }

    // Extract content from file
    const { content, originalSize } = await extractor.extract(
      fileData,
      fileType,
    );

    // Process with AI provider
    const aiResponse = await aiProvider.process({
      content,
      operation,
      customPrompt,
      fileName,
      language,
    });

    const processingTimeMs = Math.round(performance.now() - startTime);

    const response: ProcessResponse = {
      success: true,
      data: {
        result: aiResponse.result,
        model: aiResponse.model,
        fileType,
        originalSize,
        processingTimeMs,
        usage: aiResponse.usage,
      },
      error: null,
    };

    return c.json(response);
  } catch (error) {
    console.error('Error processing file:', error);

    if (error instanceof FileTooLargeError) {
      return c.json(errorResponse(error.message), 413);
    }
    if (error instanceof UnsupportedFileTypeError) {
      return c.json(errorResponse(error.message), 415);
    }
    if (error instanceof MimeTypeMismatchError) {
      return c.json(errorResponse(error.message), 415);
    }
    if (error instanceof DecompressionBombError) {
      return c.json(errorResponse(error.message), 400);
    }
    if (error instanceof FileExtractionError) {
      return c.json(errorResponse(error.message), 422);
    }

    const message = error instanceof Error ? error.message : 'Internal server error';
    return c.json(errorResponse(message), 500);
  }
});

/**
 * GET /files/supported-types - List supported file types
 */
files.get('/supported-types', (c) => {
  const supported = getAllSupported();

  const response: SupportedTypesResponse = {
    success: true,
    data: supported,
  };

  return c.json(response);
});

/**
 * GET /files/providers - List available AI providers
 */
files.get('/providers', (c) => {
  const openai = c.get('openaiProvider');
  const anthropic = c.get('anthropicProvider');

  const response: ProvidersResponse = {
    success: true,
    data: [
      {
        id: 'openai',
        name: 'OpenAI',
        available: openai !== null,
        supportsVision: openai?.supportsVision ?? false,
      },
      {
        id: 'anthropic',
        name: 'Anthropic',
        available: anthropic !== null,
        supportsVision: anthropic?.supportsVision ?? false,
      },
    ],
  };

  return c.json(response);
});

export { files };
