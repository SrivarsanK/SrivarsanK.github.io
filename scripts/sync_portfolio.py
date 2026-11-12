"""Refresh portfolio content from public GitHub repositories before building."""
import json
import os
from pathlib import Path
import urllib.request

OWNER = "SrivarsanK"


def get_json(path):
    request = urllib.request.Request(
        "https://api.github.com" + path,
        headers={
            "Authorization": "Bearer " + os.environ["GITHUB_TOKEN"],
            "Accept": "application/vnd.github+json",
            "X-GitHub-Api-Version": "2022-11-28",
            "User-Agent": "SrivarsanK-portfolio",
        },
    )
    with urllib.request.urlopen(request, timeout=30) as response:
        return json.load(response)


def rust_text(value):
    # RSX interpolation must not interpret braces in repository descriptions.
    return json.dumps(value or "", ensure_ascii=False).replace("{", "{{").replace("}", "}}")


def main():
    repositories = []
    page = 1
    while True:
        batch = get_json(f"/users/{OWNER}/repos?per_page=100&page={page}&sort=updated")
        repositories.extend(batch)
        if len(batch) < 100:
            break
        page += 1

    languages = set()
    featured = []
    for repository in repositories:
        if repository["fork"]:
            continue
        repository_languages = get_json(f"/repos/{OWNER}/{repository['name']}/languages")
        languages.update(repository_languages)
        if "portfolio" in repository.get("topics", []) or repository["name"] == "Glide":
            featured.append((repository, repository_languages))

    language_cards = "\n".join(
        '                                div { class: "px-3 py-1 rounded-md border border-[#c084fc]/30 bg-[#c084fc]/10 text-[#d8b4fe] text-xs", '
        + rust_text(language) + " }"
        for language in sorted(languages, key=str.casefold)
    )
    project_cards = []
    for repository, repository_languages in featured:
        project_cards.append(
            '                                a {\n'
            '                                    href: ' + rust_text(repository["html_url"]) + ', target: "_blank", rel: "noopener noreferrer",\n'
            '                                    class: "border border-border/50 rounded-md p-3 hover:bg-background/60 transition-colors block cursor-pointer",\n'
            '                                    style: "text-decoration: none; color: inherit;",\n'
            '                                    div { class: "font-bold text-terminal-cyan mb-1", ' + rust_text(repository["name"]) + ' }\n'
            '                                    p { class: "text-xs mb-2", style: "color: white;", ' + rust_text(repository.get("description")) + ' }\n'
            '                                    div { class: "text-[10px] text-foreground/60", ' + rust_text(" · ".join(repository_languages)) + ' }\n'
            '                                }'
        )

    path = Path("packages/ui/src/terminal.rs")
    source = path.read_text(encoding="utf-8")
    language_start = source.index('                            // Languages (Purple)')
    language_end = source.index('                            // Frameworks / ML (Green)', language_start)
    source = source[:language_start] + (
        '                            // Languages (Purple)\n'
        '                            div { class: "text-sm text-foreground/60 mb-2 mt-4 font-bold tracking-wide", "LANGUAGES" }\n'
        '                            div { class: "flex flex-wrap gap-2",\n'
        + language_cards + '\n                            }\n\n'
    ) + source[language_end:]
    grid = '                                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px;",'
    project_start = source.index(grid, source.index('            "projects" => {')) + len(grid)
    project_end = source.index('\n                            }\n                            div {', project_start)
    source = source[:project_start] + "\n" + "\n".join(project_cards) + source[project_end:]
    path.write_text(source, encoding="utf-8")
    print(f"Updated {len(languages)} languages and {len(featured)} projects.")


if __name__ == "__main__":
    main()
