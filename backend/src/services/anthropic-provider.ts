import Anthropic from "@anthropic-ai/sdk";
import { toPrompt, parseOperation } from "../types";
import type { AiProvider, AiRequest, AiResponse, FileContent } from "./ai-provider";

type ImageMediaType = "image/jpeg" | "image/png" | "image/gif" | "image/webp";

/**
 * Anthropic provider implementation
 */
export class AnthropicProvider implements AiProvider {
  readonly name = "anthropic";
  private client: Anthropic;
  private model: string;

  constructor(apiKey: string, model: string, timeoutMs: number) {
    this.model = model;
    this.client = new Anthropic({
      apiKey,
      timeout: timeoutMs,
    });
  }

  get supportsVision(): boolean {
    return this.model.includes("claude-3");
  }

  async process(request: AiRequest): Promise<AiResponse> {
    const operation = parseOperation(request.operation);
    const prompt = toPrompt(operation, request.customPrompt);

    const content = this.buildContent(request.content, prompt, request.fileName);

    const response = await this.client.messages.create({
      model: this.model,
      max_tokens: 4096,
      messages: [{ role: "user", content }],
    });

    // Extract text from response
    const result = response.content
      .filter((block): block is Anthropic.TextBlock => block.type === "text")
      .map((block) => block.text)
      .join("\n");

    if (!result) {
      throw new Error("No text response from Anthropic");
    }

    return {
      result,
      model: response.model,
      usage: {
        inputTokens: response.usage.input_tokens,
        outputTokens: response.usage.output_tokens,
      },
    };
  }

  private buildContent(
    content: FileContent,
    prompt: string,
    fileName?: string | null
  ): Anthropic.MessageParam["content"] {
    if (content.type === "text") {
      let fullText = `${prompt}\n\n`;
      if (fileName) {
        fullText += `File: ${fileName}\n\n`;
      }
      fullText += content.text;

      return [{ type: "text", text: fullText }];
    }

    // Image content
    if (!this.supportsVision) {
      throw new Error(
        `Model ${this.model} does not support vision/image processing`
      );
    }

    const base64Data = Buffer.from(content.data).toString("base64");

    let textPart = prompt;
    if (fileName) {
      textPart += `\n\nFile: ${fileName}`;
    }

    return [
      { type: "text", text: textPart },
      {
        type: "image",
        source: {
          type: "base64",
          media_type: content.mediaType as ImageMediaType,
          data: base64Data,
        },
      },
    ];
  }
}
