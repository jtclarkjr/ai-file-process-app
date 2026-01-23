import OpenAI from "openai";
import { toPrompt, parseOperation } from "../types";
import type { AiProvider, AiRequest, AiResponse, FileContent } from "./ai-provider";

/**
 * OpenAI provider implementation
 */
export class OpenAiProvider implements AiProvider {
  readonly name = "openai";
  private client: OpenAI;
  private model: string;

  constructor(apiKey: string, model: string, timeoutMs: number) {
    this.model = model;
    this.client = new OpenAI({
      apiKey,
      timeout: timeoutMs,
    });
  }

  get supportsVision(): boolean {
    return (
      this.model.includes("vision") ||
      this.model.includes("gpt-4-turbo") ||
      this.model.includes("gpt-4o") ||
      this.model.includes("gpt-5")
    );
  }

  private get usesMaxCompletionTokens(): boolean {
    // GPT-5+ models use max_completion_tokens instead of max_tokens
    return this.model.includes("gpt-5") || this.model.includes("o1") || this.model.includes("o3");
  }

  async process(request: AiRequest): Promise<AiResponse> {
    const operation = parseOperation(request.operation);
    const prompt = toPrompt(operation, request.customPrompt);

    const messages = this.buildMessages(request.content, prompt, request.fileName);

    const params: OpenAI.ChatCompletionCreateParamsNonStreaming = {
      model: this.model,
      messages,
    };

    // Use appropriate token limit parameter based on model
    if (this.usesMaxCompletionTokens) {
      params.max_completion_tokens = 4096;
    } else {
      params.max_tokens = 4096;
    }

    const response = await this.client.chat.completions.create(params);

    const result = response.choices[0]?.message?.content ?? "";

    return {
      result,
      model: response.model,
      usage: response.usage
        ? {
            inputTokens: response.usage.prompt_tokens,
            outputTokens: response.usage.completion_tokens,
          }
        : null,
    };
  }

  private buildMessages(
    content: FileContent,
    prompt: string,
    fileName?: string | null
  ): OpenAI.ChatCompletionMessageParam[] {
    if (content.type === "text") {
      let fullPrompt = `${prompt}\n\n`;
      if (fileName) {
        fullPrompt += `File: ${fileName}\n\n`;
      }
      fullPrompt += content.text;

      return [{ role: "user", content: fullPrompt }];
    }

    // Image content
    if (!this.supportsVision) {
      throw new Error(
        `Model ${this.model} does not support vision/image processing`
      );
    }

    const base64Data = Buffer.from(content.data).toString("base64");
    const dataUrl = `data:${content.mediaType};base64,${base64Data}`;

    let textPart = prompt;
    if (fileName) {
      textPart += `\n\nFile: ${fileName}`;
    }

    return [
      {
        role: "user",
        content: [
          { type: "text", text: textPart },
          { type: "image_url", image_url: { url: dataUrl } },
        ],
      },
    ];
  }
}
