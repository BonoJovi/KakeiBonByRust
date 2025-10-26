#!/bin/zsh
# AI Context Initializer for GitHub Copilot CLI (zsh version)
# Usage: Add to ~/.zshrc: source /path/to/this/file
#        Or run: source .ai-context/init.zsh

# Display project context from .ai-context/ directory
ghcp-init() {
    local ai_context_dir=".ai-context"
    
    # Check if .ai-context directory exists
    if [[ ! -d "$ai_context_dir" ]]; then
        echo "⚠️  .ai-context/ directory not found in current directory"
        echo "   Please run this command from the project root"
        return 1
    fi
    
    echo "📚 Loading project context..."
    echo "================================"
    echo ""
    
    # Read all markdown files in .ai-context/ (zsh glob)
    local files=($ai_context_dir/*.md(N))
    
    if [[ ${#files[@]} -eq 0 ]]; then
        echo "⚠️  No .md files found in .ai-context/"
        return 1
    fi
    
    for file in $files; do
        echo "--- ${file:t} ---"
        cat "$file"
        echo ""
        echo ""
    done
    
    echo "================================"
    echo "✓ Project context loaded"
    echo ""
    echo "You can now ask GitHub Copilot CLI questions with full project context."
}

echo "✓ ghcp-init function loaded. Run 'ghcp-init' to load project context."
