#!/usr/bin/env sh
set -eu

repository="${APOSLOP_REPOSITORY:-EzyGang/aposlop}"
branch="${APOSLOP_DOCS_BRANCH:-main}"
destination="${1:-aposlop-docs}"
workflow="docs-artifact.yml"

if ! command -v gh >/dev/null 2>&1; then
    printf 'docs downloader: GitHub CLI is required\n' >&2
    exit 1
fi

if ! gh auth status >/dev/null 2>&1; then
    printf 'docs downloader: authenticate with gh auth login or GH_TOKEN\n' >&2
    exit 1
fi

if [ -e "$destination" ]; then
    if [ ! -d "$destination" ] || [ -n "$(ls -A "$destination")" ]; then
        printf 'docs downloader: destination must be absent or empty: %s\n' "$destination" >&2
        exit 1
    fi
else
    mkdir -p "$destination"
fi

run_id="$(gh run list \
    --repo "$repository" \
    --workflow "$workflow" \
    --branch "$branch" \
    --status success \
    --limit 1 \
    --json databaseId \
    --jq '.[0].databaseId // empty')"

if [ -z "$run_id" ]; then
    printf 'docs downloader: no successful documentation workflow run exists on %s\n' "$branch" >&2
    exit 1
fi

artifact="$(gh api "repos/$repository/actions/runs/$run_id/artifacts" \
    --jq '[.artifacts[] | select(.expired == false and (.name | startswith("aposlop-docs-")))] | sort_by(.created_at) | .[-1].name // empty')"

if [ -z "$artifact" ]; then
    printf 'docs downloader: the latest successful run has no active documentation artifact\n' >&2
    exit 1
fi

gh run download "$run_id" \
    --repo "$repository" \
    --name "$artifact" \
    --dir "$destination"

printf 'Downloaded %s to %s\n' "$artifact" "$destination"
