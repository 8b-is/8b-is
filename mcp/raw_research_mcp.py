"""
8b-is — Model Context Protocol (MCP) Server for Raw Research Contributions
Provides MCP tools for AI agents and researchers to query, search, and submit raw research papers.
"""

import os
import json
import glob
from pathlib import Path

RESEARCH_DIR = Path(__file__).parent.parent / "raw_research"

def submit_raw_research(title: str, author: str, content: str, tags: list[str]) -> dict[str, str]:
    """
    MCP Tool: Submits a new raw research paper into the raw_research/ directory.
    """
    RESEARCH_DIR.mkdir(parents=True, exist_ok=True)
    
    # Generate clean filename from title
    clean_title = "".join(c if c.isalnum() else "_" for c in title.lower()).strip("_")
    if not clean_title:
        return {"status": "INVALID_TITLE", "message": "Title must contain at least one letter or number."}
    filename = f"{clean_title}.md"
    file_path = RESEARCH_DIR / filename

    tag_str = " ".join(f"#{t.strip('#')}" for t in tags)
    header = f"# {title}\n\n**Author:** {author}  \n**Tags:** {tag_str}  \n\n"
    
    full_text = header + content
    # Exclusive creation prevents both concurrent overwrites and following an
    # existing symlink. A submission never edits an existing paper.
    try:
        with file_path.open("x", encoding="utf-8") as output:
            output.write(full_text)
    except FileExistsError:
        return {
            "status": "CONFLICT",
            "file_path": str(file_path),
            "filename": filename,
            "message": "A paper with this normalized title already exists; choose a different title."
        }

    return {
        "status": "SUCCESS",
        "file_path": str(file_path),
        "filename": filename,
        "message": f"Raw research paper '{title}' successfully saved to raw_research/{filename}."
    }

def list_raw_research() -> dict[str, list[dict[str, str]]]:
    """
    MCP Tool: Lists all raw research papers currently in the raw_research/ directory.
    """
    if not RESEARCH_DIR.exists():
        return {"papers": []}

    papers = []
    for p in RESEARCH_DIR.glob("*.md"):
        if p.name in ["README.md", "paper_template.md"]:
            continue
        
        content = p.read_text(encoding="utf-8", errors="ignore")
        first_line = content.splitlines()[0] if content.splitlines() else p.name
        papers.append({
            "filename": p.name,
            "title": first_line.lstrip("#").strip(),
            "path": str(p)
        })

    return {"papers": papers}

def search_raw_research(query: str) -> dict[str, list[dict[str, str]]]:
    """
    MCP Tool: Searches across raw research papers for a matching query string or tag.
    """
    query_lower = query.lower()
    matches = []

    if not RESEARCH_DIR.exists():
        return {"results": []}

    for p in RESEARCH_DIR.glob("*.md"):
        content = p.read_text(encoding="utf-8", errors="ignore")
        if query_lower in content.lower():
            matches.append({
                "filename": p.name,
                "path": str(p),
                "snippet": content[:300] + "..."
            })

    return {"results": matches}

if __name__ == "__main__":
    print("8b-is Raw Research MCP Tools Ready.")
    print("Functions:", [submit_raw_research, list_raw_research, search_raw_research])
