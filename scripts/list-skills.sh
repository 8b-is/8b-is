#!/usr/bin/env bash
# list-skills — list the available skills in a proper UX way.
# Groups by source, shows each skill's name and one-line description, and
# reports a total. Sources: ~/.agents, ~/.claude, and the workspace.
set -u

root="$(cd "$(dirname "$0")/.." && pwd)"
roots=(
  "$HOME/.agents/skills|user agents skills"
  "$HOME/.claude/skills|user claude skills"
  "$root/.agents/skills|workspace skills"
)

total=0
for entry in "${roots[@]}"; do
  dir="${entry%%|*}"
  label="${entry#*|}"
  [ -d "$dir" ] || continue
  count=0
  printf '\n== %s (%s) ==\n' "$label" "${dir/#$HOME/\~}"
  for skill in "$dir"/*/; do
    [ -d "$skill" ] && [ -f "$skill/SKILL.md" ] || continue
    name="$(basename "$skill")"
    desc="$(awk '
      /^description:/ {
        sub(/^description:[[:space:]]*/, "");
        if ($0 ~ /^>/) { getline; while ($0 ~ /^[[:space:]]*$/) getline; sub(/^[[:space:]]*/, ""); print; exit }
        else { sub(/^["'"'"']/, ""); sub(/["'"'"']$/, ""); print; exit }
      }' "$skill/SKILL.md" 2>/dev/null | cut -c1-96)"
    printf '  %-26s %s\n' "$name" "${desc:-—}"
    count=$((count + 1))
  done
  printf '  (%d skills)\n' "$count"
  total=$((total + count))
done
printf '\n%d skills total.\n' "$total"