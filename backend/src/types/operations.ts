/**
 * Operation definitions and prompts
 */
export const Operation = {
  SUMMARIZE: "summarize",
  EXTRACT: "extract",
  ANALYZE: "analyze",
  CLASSIFY: "classify",
  CUSTOM: "custom",
} as const;

export type Operation = (typeof Operation)[keyof typeof Operation];

const operationPrompts: Record<Exclude<Operation, "custom">, string> = {
  [Operation.SUMMARIZE]:
    "Provide a concise summary of the following content. Focus on the main points and key takeaways.",
  [Operation.EXTRACT]:
    "Extract all important information from the following content. Include key facts, figures, names, dates, and any structured data. Format the output as a structured list.",
  [Operation.ANALYZE]:
    "Analyze the following content in depth. Identify themes, patterns, sentiment, and provide insights. Include both objective observations and interpretive analysis.",
  [Operation.CLASSIFY]: `Classify the following content. Determine:
1. Document type (e.g., report, letter, article, code, etc.)
2. Primary topic/subject
3. Target audience
4. Tone (formal, informal, technical, etc.)
5. Key categories or tags that apply`,
};

export function toPrompt(
  operation: Operation,
  customPrompt?: string | null,
): string {
  if (operation === Operation.CUSTOM) {
    return customPrompt ?? "Process this content.";
  }
  return operationPrompts[operation];
}

export function parseOperation(value: string): Operation {
  const normalized = value.toLowerCase();
  const validOps = Object.values(Operation);

  if (validOps.includes(normalized as Operation)) {
    return normalized as Operation;
  }

  throw new Error(
    `Invalid operation: ${value}. Valid options: ${validOps.join(", ")}`,
  );
}
