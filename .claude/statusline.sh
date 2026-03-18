#!/bin/bash
# Claude Code statusline script (ToS-safe version)
# Line 1: Model | Context% | +added/-removed | git branch
# Line 2: Context window progress bar (detailed)
# Line 3: Session cost | Version
#
# This version uses ONLY stdin data provided by Claude Code.
# No external API calls, no OAuth token extraction, no keychain access.
# Fully compliant with Anthropic Consumer Terms of Service.

set -euo pipefail

# ---------- Read stdin (Claude Code provides JSON) ----------
input=$(cat)

# ---------- ANSI Colors ----------
readonly GREEN=$'\e[38;2;151;201;195m'
readonly YELLOW=$'\e[38;2;229;192;123m'
readonly RED=$'\e[38;2;224;108;117m'
readonly GRAY=$'\e[38;2;74;88;92m'
readonly RESET=$'\e[0m'
readonly DIM=$'\e[2m'
readonly SEP="${GRAY} │ ${RESET}"

# ---------- Badge (environment-driven) ----------
badge=""
if [[ -n "${CLAUDE_STATUSLINE_BADGE:-}" ]]; then
  badge="${YELLOW}${CLAUDE_STATUSLINE_BADGE} ${RESET}${SEP}"
fi

# ---------- Section visibility ----------
_hide="${CLAUDE_STATUSLINE_HIDE:-}"
_hidden() { [[ ",$_hide," == *",$1,"* ]]; }

# ---------- JSON field extraction (no eval, no jq) ----------
json_str() { printf '%s' "$1" | grep -oP "\"${2}\":\s*\"\K[^\"]+" | head -1; }
json_num() { printf '%s' "$1" | grep -oP "\"${2}\":\s*\K[0-9.]+"  | head -1; }

# ---------- Color by percentage ----------
color_for_pct() {
  local ipct
  ipct=$(printf "%.0f" "${1:-0}" 2>/dev/null) || ipct=0
  if (( ipct >= 80 )); then
    printf '%s' "$RED"
  elif (( ipct >= 50 )); then
    printf '%s' "$YELLOW"
  else
    printf '%s' "$GREEN"
  fi
}

# ---------- Progress bar (10 segments) ----------
progress_bar() {
  local filled
  filled=$(awk "BEGIN{v=int(${1:-0}/10+0.5); if(v>10)v=10; if(v<0)v=0; printf \"%d\",v}")
  local bar="" i
  for (( i=1; i<=10; i++ )); do
    if (( i <= filled )); then bar+="▰"; else bar+="▱"; fi
  done
  printf '%s' "$bar"
}

# ---------- Format token counts (e.g., 200000 -> 200K) ----------
format_tokens() {
  local t="${1:-0}"
  if (( t >= 1000000 )); then
    awk "BEGIN{printf \"%.0fM\", $t / 1000000}"
  elif (( t >= 1000 )); then
    awk "BEGIN{printf \"%.0fK\", $t / 1000}"
  else
    printf '%s' "$t"
  fi
}

# ---------- Parse stdin fields ----------
model_name=$(json_str "$input" "display_name")
used_pct=$(json_num "$input" "used_percentage")
ctx_size=$(json_num "$input" "context_window_size")
cwd=$(json_str "$input" "cwd")
lines_added=$(json_num "$input" "total_lines_added")
lines_removed=$(json_num "$input" "total_lines_removed")
total_cost=$(json_num "$input" "total_cost_usd")

# ---------- Git branch ----------
git_branch=""
if [[ -n "${cwd:-}" && -d "$cwd" ]]; then
  git_branch=$(git -C "$cwd" --no-optional-locks rev-parse --abbrev-ref HEAD 2>/dev/null) || true
fi

# ---------- Line stats ----------
git_stats=""
if [[ "${lines_added:-0}" != "0" || "${lines_removed:-0}" != "0" ]]; then
  git_stats="+${lines_added:-0}/-${lines_removed:-0}"
fi

# ---------- Context window formatting ----------
ctx_pct_int=$(printf "%.0f" "${used_pct:-0}" 2>/dev/null) || ctx_pct_int=0
ctx_color=$(color_for_pct "$ctx_pct_int")
used_tokens=$(awk "BEGIN{printf \"%.0f\", ${ctx_size:-0} * ${used_pct:-0} / 100}" 2>/dev/null) || used_tokens=0

# ---------- Cost formatting ----------
cost_display=""
if [[ -n "${total_cost:-}" && "$total_cost" != "0" ]]; then
  cost_display=$(awk "BEGIN{printf \"\$%.2f\", $total_cost}" 2>/dev/null) || cost_display=""
fi

# ========== Line 1: Model | Context% | Changes | Branch ==========
line1="${badge}🤖 ${model_name:-Unknown}${SEP}${ctx_color}📊 ${ctx_pct_int}%${RESET}"
if [[ -n "$git_stats" ]] && ! _hidden changes; then
  line1+="${SEP}✏️  ${GREEN}${git_stats}${RESET}"
fi
if [[ -n "$git_branch" ]] && ! _hidden branch; then
  line1+="${SEP}🔀 ${git_branch}"
fi

# ========== Line 2: Context window progress bar ==========
line2="${ctx_color}📐 CTX  $(progress_bar "$ctx_pct_int")  ${ctx_pct_int}%${RESET}"
if (( ${ctx_size:-0} > 0 )); then
  line2+="  ${DIM}$(format_tokens "$used_tokens") / $(format_tokens "${ctx_size:-0}") tokens${RESET}"
fi

# ========== Line 3: Session cost ==========
if [[ -n "$cost_display" ]]; then
  line3="${GREEN}💰 ${cost_display}${RESET}"
else
  line3="${DIM}💰 --${RESET}"
fi

# ---------- Output ----------
printf '%s\n' "$line1"
_hidden bar  || printf '%s\n' "$line2"
_hidden cost || printf '%s'   "$line3"
