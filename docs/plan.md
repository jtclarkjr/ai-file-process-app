Implement the following plan:                                                                                             
                                                                                                                            
  # AI File Processing Implementation Plan                                                                                  
                                                                                                                            
  ## Overview                                                                                                               
                                                                                                                            
  Implement AI-powered file processing for the existing SvelteKit + Rust/Axum application with strict                       
  **no-file-retention** policy. Files are processed in-memory only and discarded immediately after processing.              
                                                                                                                            
  **User Requirements:**                                                                                                    
  - AI Providers: OpenAI (GPT-4) + Anthropic (Claude)                                                                       
  - File Types: PDF, DOCX, TXT, images (JPG/PNG), code files                                                                
  - Processing: Inline/synchronous only                                                                                     
                                                                                                                            
  ---                                                                                                                       
                                                                                                                            
  ## Architecture                                                                                                           
                                                                                                                            
  ```                                                                                                                       
  User Upload → Streaming Read → MIME Validation → Content Extraction → AI Provider → JSON Response                         
  ↓                                    ↓                                                                                    
  (bounded buffer)                    (memory dropped)                                                                      
  ↓                                                                                                                         
  NO DISK WRITES                                                                                                            
  ```                                                                                                                       
                                                                                                                            
  ---                                                                                                                       
                                                                                                                            
  ## Implementation Phases                                                                                                  
                                                                                                                            
  ### Phase 1: Backend Foundation                                                                                           
                                                                                                                            
  **New Files:**                                                                                                            
  - `backend/src/config.rs` - Configuration management with env vars                                                        
  - `backend/src/errors.rs` - Custom error types                                                                            
  - `backend/src/services/mod.rs` - Service layer module                                                                    
  - `backend/src/services/ai_provider.rs` - AI provider trait abstraction                                                   
                                                                                                                            
  **Modify:**                                                                                                               
  - `backend/Cargo.toml` - Add dependencies:                                                                                
  - `reqwest` (HTTP client for AI APIs)                                                                                     
  - `async-trait` (async trait support)                                                                                     
  - `thiserror` (error handling)                                                                                            
  - `infer` (MIME detection)                                                                                                
  - `pdf-extract` (PDF text extraction)                                                                                     
  - `docx-rs` (DOCX parsing)                                                                                                
  - `bytes`, `base64`, `mime`                                                                                               
                                                                                                                            
  ### Phase 2: AI Provider Implementations                                                                                  
                                                                                                                            
  **New Files:**                                                                                                            
  - `backend/src/services/openai.rs` - OpenAI GPT-4/Vision implementation                                                   
  - `backend/src/services/anthropic.rs` - Claude implementation                                                             
                                                                                                                            
  **Key Features:**                                                                                                         
  - Unified trait interface for both providers                                                                              
  - Vision/image support for both                                                                                           
  - Timeout handling per request                                                                                            
  - Token usage tracking                                                                                                    
                                                                                                                            
  ### Phase 3: File Extraction                                                                                              
                                                                                                                            
  **New Files:**                                                                                                            
  - `backend/src/services/file_extractor.rs` - Content extraction service                                                   
                                                                                                                            
  **Supported Extractions:**                                                                                                
  | Type | Method |                                                                                                         
  |------|--------|                                                                                                         
  | PDF | `pdf-extract` crate |                                                                                             
  | DOCX | `docx-rs` XML parsing |                                                                                          
  | TXT/MD | UTF-8 decode |                                                                                                 
  | Code | UTF-8 + language detection |                                                                                     
  | Images | Pass-through for vision API |                                                                                  
                                                                                                                            
  **Security:**                                                                                                             
  - Magic byte MIME validation                                                                                              
  - Decompression bomb protection (100:1 max ratio)                                                                         
  - Size limits enforced during streaming read                                                                              
                                                                                                                            
  ### Phase 4: Route Handlers                                                                                               
                                                                                                                            
  **New Files:**                                                                                                            
  - `backend/src/routes/file_processing.rs` - Upload/process endpoints                                                      
                                                                                                                            
  **Modify:**                                                                                                               
  - `backend/src/routes/mod.rs` - Register new routes                                                                       
  - `backend/src/main.rs` - Initialize AppState with AI providers                                                           
                                                                                                                            
  **Endpoints:**                                                                                                            
  | Method | Path | Purpose |                                                                                               
  |--------|------|---------|                                                                                               
  | POST | `/api/files/process` | Process uploaded file |                                                                   
  | GET | `/api/files/supported-types` | List supported types |                                                             
  | GET | `/api/files/providers` | Available AI providers |                                                                 
                                                                                                                            
  **Request:** `multipart/form-data` with query params:                                                                     
  - `provider`: `openai` | `anthropic`                                                                                      
  - `operation`: `summarize` | `extract` | `analyze` | `classify` | `custom`                                                
  - `custom_prompt`: (optional) for custom operations                                                                       
                                                                                                                            
  ### Phase 5: Frontend Implementation                                                                                      
                                                                                                                            
  **New Files:**                                                                                                            
  - `frontend/src/lib/stores/fileProcessor.svelte.ts` - Svelte 5 runes store                                                
  - `frontend/src/lib/components/FileUpload.svelte` - Drag & drop upload                                                    
  - `frontend/src/lib/components/ProcessingProgress.svelte` - Progress UI                                                   
  - `frontend/src/lib/components/ResultDisplay.svelte` - Result display                                                     
  - `frontend/src/routes/process/+page.svelte` - File processing page                                                       
                                                                                                                            
  **Features:**                                                                                                             
  - Provider/operation selection                                                                                            
  - Custom prompt input                                                                                                     
  - Drag & drop file upload                                                                                                 
  - Progress indication                                                                                                     
  - Result display with metadata                                                                                            
  - Privacy notice                                                                                                          
                                                                                                                            
  ---                                                                                                                       
                                                                                                                            
  ## Configuration                                                                                                          
                                                                                                                            
  **Environment Variables (.env):**                                                                                         
  ```                                                                                                                       
  # AI Providers                                                                                                            
  OPENAI_API_KEY=sk-...                                                                                                     
  OPENAI_MODEL=gpt-4-turbo                                                                                                  
  ANTHROPIC_API_KEY=sk-ant-...                                                                                              
  ANTHROPIC_MODEL=claude-3-sonnet-20240229                                                                                  
                                                                                                                            
  # Limits                                                                                                                  
  MAX_FILE_SIZE_MB=10                                                                                                       
  REQUEST_TIMEOUT_SECS=120                                                                                                  
  AI_TIMEOUT_SECS=60                                                                                                        
  ```                                                                                                                       
                                                                                                                            
  ---                                                                                                                       
                                                                                                                            
  ## Security Measures                                                                                                      
                                                                                                                            
  1. **File Size**: 10MB max, enforced during streaming read                                                                
  2. **MIME Validation**: Magic bytes checked against declared type                                                         
  3. **Timeouts**: 120s request, 60s AI call                                                                                
  4. **Decompression Bomb**: 100:1 max ratio, 50MB max decompressed                                                         
  5. **No Disk Writes**: All processing in bounded memory buffers                                                           
  6. **No Content Logging**: Only metadata logged (size, type, duration)                                                    
                                                                                                                            
  ---                                                                                                                       
                                                                                                                            
  ## Critical Files to Modify                                                                                               
                                                                                                                            
  - `backend/Cargo.toml` - Dependencies                                                                                     
  - `backend/src/main.rs` - AppState initialization                                                                         
  - `backend/src/routes/mod.rs` - Route registration                                                                        
  - `.env` / `.env.example` - New environment variables                                                                     
                                                                                                                            
  ---                                                                                                                       
                                                                                                                            
  ## Verification                                                                                                           
                                                                                                                            
  1. **Unit Tests**: AI provider mocks, extraction functions                                                                
  2. **Integration Test**: Upload PDF → receive summary                                                                     
  3. **Memory Test**: Verify no disk writes with `strace`/`fs_usage`                                                        
  4. **Timeout Test**: Large file processing respects limits                                                                
  5. **Security Test**: Reject mismatched MIME types, oversized files                                                       
                                                                                                                            
  **Manual Testing:**                                                                                                       
  ```bash                                                                                                                   
  # Start backend                                                                                                           
  cargo run --manifest-path backend/Cargo.toml                                                                              
                                                                                                                            
  # Start frontend                                                                                                          
  cd frontend && bun dev                                                                                                    
                                                                                                                            
  # Test upload                                                                                                             
  curl -X POST http://localhost:8080/api/files/process \                                                                    
  -F "file=@test.pdf" \                                                                                                     
  -F "provider=openai" \                                                                                                    
  -F "operation=summarize"                                                                                                  
  ```                  
