import type {
  Operation,
  ProcessResponse,
  ProcessResult,
  ProviderInfo,
  ResponseLanguage,
  SupportedTypeInfo,
} from "$lib/types/fileProcessor";

// Svelte 5 runes-based store
class FileProcessorStore {
  // State
  providers = $state<ProviderInfo[]>([]);
  supportedTypes = $state<SupportedTypeInfo[]>([]);
  selectedProvider = $state<string>("anthropic");
  selectedOperation = $state<Operation>("summarize");
  selectedLanguage = $state<ResponseLanguage>("en");
  customPrompt = $state<string>("");
  selectedFile = $state<File | null>(null);
  processing = $state(false);
  result = $state<ProcessResult | null>(null);
  error = $state<string | null>(null);

  private apiBase = this.resolveApiBase();
  private logPrefix = "[fileProcessor]";
  private bootstrapped = false;

  private resolveApiBase() {
    const envBase = (import.meta.env?.VITE_API_BASE as string | undefined)?.replace(/\/$/, "");
    if (envBase) return envBase;
    return "";
  }

  private log(...args: unknown[]) {
    console.log(this.logPrefix, ...args);
  }

  bootstrap() {
    if (this.bootstrapped) return;
    this.bootstrapped = true;
    this.log("bootstrap");
    this.fetchProviders();
    this.fetchSupportedTypes();
  }

  async fetchProviders() {
    this.log("fetch providers: start");
    try {
      const res = await fetch(`${this.apiBase}/api/files/providers`);
      const json = await res.json();
      if (json.success && json.data) {
        this.providers = json.data;
        this.selectedProvider = "anthropic";
        this.log("fetch providers: success", { count: json.data.length });
      }
    } catch (e) {
      this.log("fetch providers: error", e);
    }
  }

  async fetchSupportedTypes() {
    this.log("fetch supported types: start");
    try {
      const res = await fetch(`${this.apiBase}/api/files/supported-types`);
      const json = await res.json();
      if (json.success && json.data) {
        this.supportedTypes = json.data;
        this.log("fetch supported types: success", { count: json.data.length });
      }
    } catch (e) {
      this.log("fetch supported types: error", e);
    }
  }

  setFile(file: File | null) {
    this.selectedFile = file;
    this.result = null;
    this.error = null;
  }

  setOperation(operation: Operation) {
    this.selectedOperation = operation;
  }

  setCustomPrompt(prompt: string) {
    this.customPrompt = prompt;
  }

  setLanguage(language: ResponseLanguage) {
    this.selectedLanguage = language;
  }

  get acceptedExtensions(): string {
    return this.supportedTypes
      .flatMap((t) => t.extensions)
      .map((ext) => `.${ext}`)
      .join(",");
  }

  get currentProvider(): ProviderInfo | undefined {
    return this.providers.find((p) => p.id === this.selectedProvider);
  }

  get canProcess(): boolean {
    return (
      this.selectedFile !== null &&
      !this.processing &&
      this.currentProvider?.available === true &&
      (this.selectedOperation !== "custom" || this.customPrompt.trim().length > 0)
    );
  }

  async processFile() {
    if (!this.selectedFile || !this.canProcess) {
      this.error = "Please select a file and configure options";
      return;
    }

    this.log("process file: start", {
      file: this.selectedFile.name,
      size: this.selectedFile.size,
      provider: this.selectedProvider,
      operation: this.selectedOperation,
    });

    this.processing = true;
    this.error = null;
    this.result = null;

    try {
      const formData = new FormData();
      formData.append("file", this.selectedFile);

      const params = new URLSearchParams({
        provider: this.selectedProvider,
        operation: this.selectedOperation,
        language: this.selectedLanguage,
      });

      if (this.selectedOperation === "custom" && this.customPrompt) {
        params.append("custom_prompt", this.customPrompt);
      }

      const res = await fetch(`${this.apiBase}/api/files/process?${params}`, {
        method: "POST",
        body: formData,
      });

      const json: ProcessResponse = await res.json();

      if (json.success && json.data) {
        this.result = json.data;
        this.log("process file: success", {
          model: json.data.model,
          processing_time_ms: json.data.processing_time_ms,
        });
      } else {
        this.error = json.error || "Failed to process file";
        this.log("process file: api error", this.error);
      }
    } catch (e) {
      this.error = "Network error - please try again";
      this.log("process file: network error", e);
    } finally {
      this.processing = false;
    }
  }

  reset() {
    this.selectedFile = null;
    this.result = null;
    this.error = null;
    this.customPrompt = "";
  }
}

// Export singleton instance
export const fileProcessorStore = new FileProcessorStore();
