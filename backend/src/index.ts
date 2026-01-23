import { Hono } from "hono";
import { serveStatic } from "hono/bun";
import { cors } from "hono/cors";
import { logger } from "hono/logger";
import { config } from "./config";
import { files, health } from "./routes";
import {
  type AiProvider,
  AnthropicProvider,
  FileExtractor,
  OpenAiProvider,
} from "./services";

// Initialize providers
let openaiProvider: AiProvider | null = null;
let anthropicProvider: AiProvider | null = null;

if (config.openaiApiKey) {
  console.log(`OpenAI provider configured with model: ${config.openaiModel}`);
  openaiProvider = new OpenAiProvider(
    config.openaiApiKey,
    config.openaiModel,
    config.aiTimeoutMs,
  );
} else {
  console.warn("No OpenAI API key configured");
}

if (config.anthropicApiKey) {
  console.log(
    `Anthropic provider configured with model: ${config.anthropicModel}`,
  );
  anthropicProvider = new AnthropicProvider(
    config.anthropicApiKey,
    config.anthropicModel,
    config.aiTimeoutMs,
  );
} else {
  console.warn("No Anthropic API key configured");
}

if (!openaiProvider && !anthropicProvider) {
  console.warn(
    "No AI providers configured. Set OPENAI_API_KEY or ANTHROPIC_API_KEY.",
  );
}

const fileExtractor = new FileExtractor();

// Create Hono app
const app = new Hono();

// Middleware
app.use("*", logger());
app.use("*", cors());

// Inject providers into context
app.use("/api/*", async (c, next) => {
  c.set("openaiProvider", openaiProvider);
  c.set("anthropicProvider", anthropicProvider);
  c.set("fileExtractor", fileExtractor);
  await next();
});

// API routes
app.route("/api", health);
app.route("/api/files", files);

// Static files (SvelteKit SPA) - check if directory exists
const staticDir = "./static";
try {
  const stat = Bun.file(`${staticDir}/index.html`);
  if (await stat.exists()) {
    console.log("Serving static files from ./static");

    // Serve static files
    app.use("/*", serveStatic({ root: staticDir }));

    // SPA fallback - serve index.html for all unmatched routes
    app.get("*", async (c) => {
      const file = Bun.file(`${staticDir}/index.html`);
      return c.html(await file.text());
    });
  }
} catch {
  console.log("No static directory found, running API only");
}

// Root endpoint
app.get("/", (c) => {
  return c.json({ message: "AI File Processor API" });
});

// Start server
console.log(`
╔══════════════════════════════════════════════════════════════╗
║                   AI File Processor                          ║
╠══════════════════════════════════════════════════════════════╣
║  Server:     http://${config.host}:${config.port}                          ║
║  Max File:   ${config.maxFileSizeMb}MB                                        ║
║  AI Timeout: ${config.aiTimeoutSecs}s                                         ║
╚══════════════════════════════════════════════════════════════╝
`);

export default {
  port: config.port,
  hostname: config.host,
  fetch: app.fetch,
};
