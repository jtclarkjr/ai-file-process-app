Docker Deployment Plan                                                                                                     
                                                                                                                            
 Overview                                                                                                                   
                                                                                                                            
 Update the existing Docker configuration to deploy the simplified AI file processing app. The old Dockerfile references    
 removed components (WASM, shared crate, PostgreSQL) that need to be cleaned up.                                            
                                                                                                                            
 Current State                                                                                                              
                                                                                                                            
 - Dockerfile has 4 stages including WASM build (no longer needed)                                                          
 - docker-compose.yml includes PostgreSQL and pgAdmin (no longer needed)                                                    
 - Backend is now a simple Rust/Axum server with AI providers                                                               
 - Frontend is SvelteKit with adapter-auto                                                                                  
                                                                                                                            
 Architecture                                                                                                               
                                                                                                                            
 ┌─────────────────────────────────────────┐                                                                                
 │           Docker Container              │                                                                                
 │  ┌─────────────────────────────────┐    │                                                                                
 │  │     Rust Backend (Axum)         │    │                                                                                
 │  │  - Serves API at /api/*         │    │                                                                                
 │  │  - Serves static files          │    │                                                                                
 │  │  - OpenAI/Anthropic integration │    │                                                                                
 │  └─────────────────────────────────┘    │                                                                                
 │              ↓ serves                   │                                                                                
 │  ┌─────────────────────────────────┐    │                                                                                
 │  │   Static Frontend (SvelteKit)   │    │                                                                                
 │  │   - Built at /app/static        │    │                                                                                
 │  └─────────────────────────────────┘    │                                                                                
 └─────────────────────────────────────────┘                                                                                
          ↕ API calls                                                                                                       
 ┌─────────────────────────────────────────┐                                                                                
 │   External AI APIs (OpenAI/Anthropic)   │                                                                                
 └─────────────────────────────────────────┘                                                                                
                                                                                                                            
 ---                                                                                                                        
 Files to Modify                                                                                                            
                                                                                                                            
 1. Dockerfile - Simplify to 3 stages                                                                                       
                                                                                                                            
 Remove:                                                                                                                    
 - Stage 1 (WASM builder) - no longer needed                                                                                
 - References to shared and wasm directories                                                                                
                                                                                                                            
 New structure:                                                                                                             
 1. Frontend Builder - Build SvelteKit static files                                                                         
 2. Backend Builder - Compile Rust binary                                                                                   
 3. Runtime - Minimal Debian image with binary + static files                                                               
                                                                                                                            
 2. docker-compose.yml - Remove database services                                                                           
                                                                                                                            
 Remove:                                                                                                                    
 - postgres service                                                                                                         
 - pgadmin service                                                                                                          
 - postgres_data volume                                                                                                     
 - DATABASE_URL environment variable                                                                                        
 - Health check dependencies                                                                                                
                                                                                                                            
 Add:                                                                                                                       
 - AI provider environment variables (via .env file)                                                                        
 - Volume mount for .env (optional)                                                                                         
                                                                                                                            
 3. frontend/svelte.config.js - Use static adapter                                                                          
                                                                                                                            
 Change:                                                                                                                    
 - Replace adapter-auto with adapter-static                                                                                 
 - This produces static files for the backend to serve                                                                      
                                                                                                                            
 4. frontend/package.json - Add static adapter                                                                              
                                                                                                                            
 Add:                                                                                                                       
 - @sveltejs/adapter-static dependency                                                                                      
                                                                                                                            
 5. .dockerignore - Update for new structure                                                                                
                                                                                                                            
 Keep current patterns (already appropriate)                                                                                
                                                                                                                            
 ---                                                                                                                        
 Implementation Details                                                                                                     
                                                                                                                            
 Dockerfile (3-stage build)                                                                                                 
                                                                                                                            
 # Stage 1: Frontend Builder                                                                                                
 FROM oven/bun:1 AS frontend-builder                                                                                        
 # Install deps, build SvelteKit static output                                                                              
                                                                                                                            
 # Stage 2: Backend Builder                                                                                                 
 FROM rust:1.83-slim-bookworm AS backend-builder                                                                            
 # Build release binary (only backend crate)                                                                                
                                                                                                                            
 # Stage 3: Runtime                                                                                                         
 FROM debian:bookworm-slim                                                                                                  
 # Copy binary + static files, run as non-root user                                                                         
                                                                                                                            
 docker-compose.yml                                                                                                         
                                                                                                                            
 services:                                                                                                                  
   app:                                                                                                                     
     build: .                                                                                                               
     ports:                                                                                                                 
       - "8080:8080"                                                                                                        
     env_file:                                                                                                              
       - .env  # Contains OPENAI_API_KEY, ANTHROPIC_API_KEY, etc.                                                           
     environment:                                                                                                           
       - RUST_LOG=backend=info,tower_http=info                                                                              
       - HOST=0.0.0.0                                                                                                       
       - PORT=8080                                                                                                          
                                                                                                                            
 Environment Variables                                                                                                      
                                                                                                                            
 Required in .env for production:                                                                                           
 # At least one AI provider required                                                                                        
 OPENAI_API_KEY=sk-...                                                                                                      
 ANTHROPIC_API_KEY=sk-ant-...                                                                                               
                                                                                                                            
 # Optional overrides                                                                                                       
 OPENAI_MODEL=gpt-4-turbo                                                                                                   
 ANTHROPIC_MODEL=claude-3-sonnet-20240229                                                                                   
 MAX_FILE_SIZE_MB=10                                                                                                        
 AI_TIMEOUT_SECS=60                                                                                                         
                                                                                                                            
 ---                                                                                                                        
 Verification                                                                                                               
                                                                                                                            
 Build Test                                                                                                                 
                                                                                                                            
 docker build -t ai-file-processor .                                                                                        
                                                                                                                            
 Run Test                                                                                                                   
                                                                                                                            
 # Create .env with API keys first                                                                                          
 docker run -p 8080:8080 --env-file .env ai-file-processor                                                                  
                                                                                                                            
 With docker-compose                                                                                                        
                                                                                                                            
 docker-compose up --build                                                                                                  
                                                                                                                            
 Functional Test                                                                                                            
                                                                                                                            
 1. Open http://localhost:8080                                                                                              
 2. Upload a test file (PDF, TXT, image)                                                                                    
 3. Select provider and operation                                                                                           
 4. Verify processing works                                                                                                 
                                                                                                                            
 API Health Check                                                                                                           
                                                                                                                            
 curl http://localhost:8080/api/health                                                                                      
 # Should return: OK                                                                                                        
                                                                                                                            
 ---                                                                                                                        
 Security Considerations                                                                                                    
                                                                                                                            
 1. Non-root user - Container runs as appuser                                                                               
 2. No secrets in image - API keys passed via environment                                                                   
 3. Minimal base image - debian:bookworm-slim                                                                               
 4. Read-only filesystem - Can add --read-only flag                                                                         
 5. No persistent storage - Stateless container    
