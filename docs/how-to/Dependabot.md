# Combine Actions-Digest with Dependabot

How-to digest a workflow _once_ to then hand updates over to Dependabot.

## Step 1 - Digest The Workflow

> Please set GITHUB_TOKEN to a personal access token to avoid running into API rate-limting.

```shell
actions-digest workflow.yaml | sponge workflow.yaml
```

This will update the workflow file in-place. You'll see what happened in the logs.

## Step 2 - Add Dependabot Config

Create `.github/dependabot.yml` like so:

```yaml
version: 2
updates:
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
```

This will instruct Dependabot to check for new versions weekly and issue pull-request with new commit sha's on each action that changed.
