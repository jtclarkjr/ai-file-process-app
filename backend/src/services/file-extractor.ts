import { fileTypeFromBuffer } from "file-type";
import pdf from "pdf-parse";
import mammoth from "mammoth";
import { config } from "../config";
import {
  SupportedFileType,
  fromMime,
  fromExtension,
  toMime,
  isImage,
} from "../types";
import type { FileContent } from "./ai-provider";

/**
 * Custom error classes
 */
export class FileTooLargeError extends Error {
  constructor(size: number, maxSize: number) {
    super(`File too large: ${size} bytes exceeds maximum of ${maxSize} bytes`);
    this.name = "FileTooLargeError";
  }
}

export class UnsupportedFileTypeError extends Error {
  constructor(fileType: string) {
    super(`Unsupported file type: ${fileType}`);
    this.name = "UnsupportedFileTypeError";
  }
}

export class MimeTypeMismatchError extends Error {
  constructor(declared: string, detected: string) {
    super(`MIME type mismatch: declared ${declared}, detected ${detected}`);
    this.name = "MimeTypeMismatchError";
  }
}

export class DecompressionBombError extends Error {
  constructor(ratio: number, maxRatio: number) {
    super(
      `Decompression bomb detected: ratio ${ratio}:1 exceeds maximum of ${maxRatio}:1`
    );
    this.name = "DecompressionBombError";
  }
}

export class FileExtractionError extends Error {
  constructor(message: string) {
    super(`File extraction failed: ${message}`);
    this.name = "FileExtractionError";
  }
}

/**
 * File extractor service
 */
export class FileExtractor {
  /**
   * Validate and detect file type from data
   */
  async validateAndDetectType(
    data: Uint8Array,
    declaredMime?: string | null,
    fileName?: string | null
  ): Promise<SupportedFileType> {
    // Check file size
    if (data.length > config.maxFileSizeBytes) {
      throw new FileTooLargeError(data.length, config.maxFileSizeBytes);
    }

    // Detect MIME type from magic bytes
    const detected = await fileTypeFromBuffer(data);
    const detectedMime = detected?.mime;

    // Try to determine file type from detected MIME
    let fileType = detectedMime ? fromMime(detectedMime) : undefined;

    // If not detected from magic bytes, try extension
    if (!fileType && fileName) {
      const ext = fileName.split(".").pop();
      if (ext) {
        fileType = fromExtension(ext);
      }
    }

    // If still not detected, try declared MIME
    if (!fileType && declaredMime) {
      fileType = fromMime(declaredMime);
    }

    if (!fileType) {
      throw new UnsupportedFileTypeError(
        detectedMime ?? declaredMime ?? "unknown"
      );
    }

    // For binary files, validate MIME type matches if declared
    if (declaredMime && detectedMime) {
      if (isImage(fileType) || fileType === SupportedFileType.PDF) {
        const declaredType = fromMime(declaredMime);
        const detectedType = fromMime(detectedMime);

        if (declaredType !== detectedType) {
          throw new MimeTypeMismatchError(declaredMime, detectedMime);
        }
      }
    }

    return fileType;
  }

  /**
   * Extract content from file
   */
  async extract(
    data: Uint8Array,
    fileType: SupportedFileType
  ): Promise<{ content: FileContent; originalSize: number }> {
    const originalSize = data.length;

    let content: FileContent;

    switch (fileType) {
      case SupportedFileType.PDF:
        content = await this.extractPdf(data);
        break;
      case SupportedFileType.DOCX:
        content = await this.extractDocx(data);
        break;
      case SupportedFileType.TEXT:
      case SupportedFileType.MARKDOWN:
      case SupportedFileType.CODE:
        content = this.extractText(data);
        break;
      default:
        if (isImage(fileType)) {
          content = {
            type: "image",
            data,
            mediaType: toMime(fileType),
          };
        } else {
          throw new FileExtractionError(`Unsupported file type: ${fileType}`);
        }
    }

    // Check for decompression bomb (text content much larger than original)
    if (content.type === "text") {
      const textBytes = new TextEncoder().encode(content.text).length;
      const ratio = Math.floor(textBytes / Math.max(originalSize, 1));

      if (ratio > config.maxDecompressionRatio) {
        throw new DecompressionBombError(ratio, config.maxDecompressionRatio);
      }

      if (textBytes > config.maxDecompressedSizeBytes) {
        throw new FileTooLargeError(textBytes, config.maxDecompressedSizeBytes);
      }
    }

    return { content, originalSize };
  }

  private async extractPdf(data: Uint8Array): Promise<FileContent> {
    try {
      const buffer = Buffer.from(data);
      const result = await pdf(buffer);

      if (!result.text.trim()) {
        throw new FileExtractionError(
          "PDF contains no extractable text (may be image-based)"
        );
      }

      return { type: "text", text: result.text };
    } catch (error) {
      if (error instanceof FileExtractionError) throw error;
      throw new FileExtractionError(
        `PDF extraction failed: ${error instanceof Error ? error.message : String(error)}`
      );
    }
  }

  private async extractDocx(data: Uint8Array): Promise<FileContent> {
    try {
      const buffer = Buffer.from(data);
      const result = await mammoth.extractRawText({ buffer });

      if (!result.value.trim()) {
        throw new FileExtractionError("DOCX contains no extractable text");
      }

      return { type: "text", text: result.value };
    } catch (error) {
      if (error instanceof FileExtractionError) throw error;
      throw new FileExtractionError(
        `DOCX parsing failed: ${error instanceof Error ? error.message : String(error)}`
      );
    }
  }

  private extractText(data: Uint8Array): FileContent {
    try {
      const text = new TextDecoder("utf-8", { fatal: true }).decode(data);
      return { type: "text", text };
    } catch {
      throw new FileExtractionError("Invalid UTF-8 in text file");
    }
  }
}
