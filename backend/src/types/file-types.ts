/**
 * Supported file type definitions
 */
export const SupportedFileType = {
  PDF: "pdf",
  DOCX: "docx",
  TEXT: "text",
  MARKDOWN: "markdown",
  CODE: "code",
  IMAGE_JPEG: "image_jpeg",
  IMAGE_PNG: "image_png",
  IMAGE_GIF: "image_gif",
  IMAGE_WEBP: "image_webp",
} as const;

export type SupportedFileType =
  (typeof SupportedFileType)[keyof typeof SupportedFileType];

const mimeToFileType: Record<string, SupportedFileType> = {
  "application/pdf": SupportedFileType.PDF,
  "application/vnd.openxmlformats-officedocument.wordprocessingml.document":
    SupportedFileType.DOCX,
  "text/plain": SupportedFileType.TEXT,
  "text/markdown": SupportedFileType.MARKDOWN,
  "image/jpeg": SupportedFileType.IMAGE_JPEG,
  "image/png": SupportedFileType.IMAGE_PNG,
  "image/gif": SupportedFileType.IMAGE_GIF,
  "image/webp": SupportedFileType.IMAGE_WEBP,
  // Code files
  "text/x-python": SupportedFileType.CODE,
  "application/x-python": SupportedFileType.CODE,
  "text/javascript": SupportedFileType.CODE,
  "application/javascript": SupportedFileType.CODE,
  "text/x-typescript": SupportedFileType.CODE,
  "application/typescript": SupportedFileType.CODE,
  "application/json": SupportedFileType.CODE,
  "text/yaml": SupportedFileType.CODE,
  "application/x-yaml": SupportedFileType.CODE,
  "text/html": SupportedFileType.CODE,
  "text/css": SupportedFileType.CODE,
  "text/xml": SupportedFileType.CODE,
  "application/xml": SupportedFileType.CODE,
};

const extensionToFileType: Record<string, SupportedFileType> = {
  // Documents
  pdf: SupportedFileType.PDF,
  docx: SupportedFileType.DOCX,
  txt: SupportedFileType.TEXT,
  md: SupportedFileType.MARKDOWN,
  markdown: SupportedFileType.MARKDOWN,
  // Code files
  py: SupportedFileType.CODE,
  pyw: SupportedFileType.CODE,
  js: SupportedFileType.CODE,
  mjs: SupportedFileType.CODE,
  cjs: SupportedFileType.CODE,
  ts: SupportedFileType.CODE,
  tsx: SupportedFileType.CODE,
  mts: SupportedFileType.CODE,
  rs: SupportedFileType.CODE,
  c: SupportedFileType.CODE,
  h: SupportedFileType.CODE,
  cpp: SupportedFileType.CODE,
  hpp: SupportedFileType.CODE,
  cc: SupportedFileType.CODE,
  java: SupportedFileType.CODE,
  go: SupportedFileType.CODE,
  json: SupportedFileType.CODE,
  yaml: SupportedFileType.CODE,
  yml: SupportedFileType.CODE,
  html: SupportedFileType.CODE,
  htm: SupportedFileType.CODE,
  css: SupportedFileType.CODE,
  scss: SupportedFileType.CODE,
  sass: SupportedFileType.CODE,
  xml: SupportedFileType.CODE,
  svg: SupportedFileType.CODE,
  sh: SupportedFileType.CODE,
  bash: SupportedFileType.CODE,
  zsh: SupportedFileType.CODE,
  sql: SupportedFileType.CODE,
  rb: SupportedFileType.CODE,
  php: SupportedFileType.CODE,
  swift: SupportedFileType.CODE,
  kt: SupportedFileType.CODE,
  kts: SupportedFileType.CODE,
  scala: SupportedFileType.CODE,
  toml: SupportedFileType.CODE,
  // Images
  jpg: SupportedFileType.IMAGE_JPEG,
  jpeg: SupportedFileType.IMAGE_JPEG,
  png: SupportedFileType.IMAGE_PNG,
  gif: SupportedFileType.IMAGE_GIF,
  webp: SupportedFileType.IMAGE_WEBP,
};

const fileTypeToMime: Record<SupportedFileType, string> = {
  [SupportedFileType.PDF]: "application/pdf",
  [SupportedFileType.DOCX]:
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
  [SupportedFileType.TEXT]: "text/plain",
  [SupportedFileType.MARKDOWN]: "text/markdown",
  [SupportedFileType.CODE]: "text/plain",
  [SupportedFileType.IMAGE_JPEG]: "image/jpeg",
  [SupportedFileType.IMAGE_PNG]: "image/png",
  [SupportedFileType.IMAGE_GIF]: "image/gif",
  [SupportedFileType.IMAGE_WEBP]: "image/webp",
};

export function fromMime(mime: string): SupportedFileType | undefined {
  return mimeToFileType[mime];
}

export function fromExtension(ext: string): SupportedFileType | undefined {
  return extensionToFileType[ext.toLowerCase().replace(/^\./, "")];
}

export function toMime(fileType: SupportedFileType): string {
  return fileTypeToMime[fileType];
}

export function isImage(fileType: SupportedFileType): boolean {
  return [
    SupportedFileType.IMAGE_JPEG,
    SupportedFileType.IMAGE_PNG,
    SupportedFileType.IMAGE_GIF,
    SupportedFileType.IMAGE_WEBP,
  ].includes(fileType);
}

export interface SupportedTypeInfo {
  fileType: SupportedFileType;
  extensions: string[];
  description: string;
}

export function getAllSupported(): SupportedTypeInfo[] {
  return [
    {
      fileType: SupportedFileType.PDF,
      extensions: ["pdf"],
      description: "PDF documents",
    },
    {
      fileType: SupportedFileType.DOCX,
      extensions: ["docx"],
      description: "Microsoft Word documents",
    },
    {
      fileType: SupportedFileType.TEXT,
      extensions: ["txt"],
      description: "Plain text files",
    },
    {
      fileType: SupportedFileType.MARKDOWN,
      extensions: ["md", "markdown"],
      description: "Markdown files",
    },
    {
      fileType: SupportedFileType.CODE,
      extensions: [
        "py",
        "js",
        "ts",
        "tsx",
        "rs",
        "c",
        "cpp",
        "java",
        "go",
        "json",
        "yaml",
        "yml",
        "html",
        "css",
        "xml",
        "sh",
        "sql",
        "rb",
        "php",
        "swift",
        "kt",
        "scala",
        "toml",
      ],
      description: "Source code files",
    },
    {
      fileType: SupportedFileType.IMAGE_JPEG,
      extensions: ["jpg", "jpeg"],
      description: "JPEG images",
    },
    {
      fileType: SupportedFileType.IMAGE_PNG,
      extensions: ["png"],
      description: "PNG images",
    },
    {
      fileType: SupportedFileType.IMAGE_GIF,
      extensions: ["gif"],
      description: "GIF images",
    },
    {
      fileType: SupportedFileType.IMAGE_WEBP,
      extensions: ["webp"],
      description: "WebP images",
    },
  ];
}
