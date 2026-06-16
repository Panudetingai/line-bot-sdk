# Publish line-bot-sdk-rs + win32-x64-msvc to npm (Windows)
# Usage: $env:NPM_TOKEN = "your-token"; .\scripts\publish-npm.ps1

$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

if (-not $env:NPM_TOKEN) {
    Write-Error 'Set NPM_TOKEN first: $env:NPM_TOKEN = "npm_..."'
}

$version = node -p "require('./package.json').version"
Write-Host "Publishing version: $version"

corepack enable
yarn install --immutable
yarn build -- --target x86_64-pc-windows-msvc

$nodeFile = 'line-bot-sdk-rs.win32-x64-msvc.node'
if (-not (Test-Path $nodeFile)) {
    throw "Native binding not found: $nodeFile"
}

yarn napi create-npm-dirs
$artifactDir = 'artifacts/bindings-x86_64-pc-windows-msvc'
New-Item -ItemType Directory -Force -Path $artifactDir | Out-Null
Copy-Item -Force $nodeFile "$artifactDir/$nodeFile"

yarn artifacts
yarn build:js

if (-not (Test-Path "npm/win32-x64-msvc/$nodeFile")) {
    throw "Platform package missing .node file after artifacts step"
}

npm config set "//registry.npmjs.org/:_authToken" $env:NPM_TOKEN
try {
    Write-Host "Logged in as: $(npm whoami)"
    npm publish --access public --ignore-scripts
    npm publish --access public npm/win32-x64-msvc
    Write-Host "Published: line-bot-sdk-rs@$version"
    Write-Host "Published: line-bot-sdk-rs-win32-x64-msvc@$version"
    Write-Host "Verify: npm view line-bot-sdk-rs version"
} finally {
    npm config delete "//registry.npmjs.org/:_authToken" 2>$null
}
