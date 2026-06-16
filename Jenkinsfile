// Single-node Windows Jenkins pipeline for line-bot-sdk-rs
//
// Jenkins Credential: Secret text, ID: NPM_TOKEN
//
// Publish: Build with Parameters -> PUBLISH_TO_NPM = true

pipeline {
    agent any

    parameters {
        booleanParam(
            name: 'PUBLISH_TO_NPM',
            defaultValue: true,
            description: 'Publish line-bot-sdk-rs to npm after tests pass'
        )
    }

    environment {
        NODE_FILE = 'line-bot-sdk-rs.win32-x64-msvc.node'
        DEBUG = 'napi:*'
        CARGO_INCREMENTAL = '1'
        HUSKY = '0'
        CI = 'true'
    }

    options {
        disableConcurrentBuilds()
        timeout(time: 60, unit: 'MINUTES')
        buildDiscarder(logRotator(numToKeepStr: '20'))
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Install dependencies') {
            steps {
                bat '''
                    @echo off
                    cd /d "%WORKSPACE%"
                    corepack enable
                    node --version
                    yarn --version
                    yarn install --immutable
                '''
            }
        }

        stage('Lint') {
            steps {
                bat '''
                    @echo off
                    cd /d "%WORKSPACE%"
                    corepack enable
                    yarn install --immutable
                    yarn lint
                    cargo fmt -- --check
                '''
            }
        }

        stage('Build native bindings (Windows)') {
            steps {
                bat '''
                    @echo off
                    cd /d "%WORKSPACE%"
                    corepack enable
                    yarn install --immutable
                    rustup target add x86_64-pc-windows-msvc
                    yarn build -- --target x86_64-pc-windows-msvc
                    if not exist "%NODE_FILE%" (
                        echo ERROR: missing %NODE_FILE%
                        exit /b 1
                    )
                '''
            }
        }

        stage('Test On Windows') {
            steps {
                bat '''
                    @echo off
                    cd /d "%WORKSPACE%"
                    corepack enable
                    yarn install --immutable
                    yarn test
                '''
            }
        }

        stage('Publish to npm') {
            when {
                expression {
                    params.PUBLISH_TO_NPM ||
                    (env.TAG_NAME ?: '') ==~ /^v.*/ ||
                    (env.BRANCH_NAME ?: '') ==~ /^v.*/
                }
            }
            steps {
                withCredentials([string(credentialsId: 'NPM_TOKEN', variable: 'NPM_TOKEN')]) {
                    powershell '''
                        $ErrorActionPreference = "Stop"
                        Set-Location $env:WORKSPACE

                        corepack enable
                        yarn install --immutable

                        $version = node -p "require('./package.json').version"
                        Write-Host "Publishing version: $version"

                        if (-not (Test-Path $env:NODE_FILE)) {
                            throw "Missing native binding: $($env:NODE_FILE)"
                        }

                        yarn napi create-npm-dirs
                        $artifactDir = "artifacts/bindings-x86_64-pc-windows-msvc"
                        New-Item -ItemType Directory -Force -Path $artifactDir | Out-Null
                        Copy-Item -Force $env:NODE_FILE "$artifactDir/$env:NODE_FILE"

                        yarn artifacts
                        yarn build:js

                        if (-not (Test-Path "npm/win32-x64-msvc/$env:NODE_FILE")) {
                            throw "Platform .node file missing in npm/win32-x64-msvc"
                        }

                        npm config set "//registry.npmjs.org/:_authToken" $env:NPM_TOKEN
                        try {
                            $user = npm whoami 2>&1
                            if ($LASTEXITCODE -ne 0) { throw "npm whoami failed: $user" }
                            Write-Host "npm user: $user"

                            npm publish --access public --ignore-scripts
                            if ($LASTEXITCODE -ne 0) { throw "npm publish main package failed" }

                            npm publish --access public npm/win32-x64-msvc
                            if ($LASTEXITCODE -ne 0) { throw "npm publish win32 package failed" }

                            $published = npm view line-bot-sdk-rs version
                            Write-Host "npm registry version: $published"
                            if ($published -ne $version) {
                                throw "Expected version $version but npm has $published"
                            }
                            Write-Host "SUCCESS: published line-bot-sdk-rs@$version"
                        } finally {
                            npm config delete "//registry.npmjs.org/:_authToken" 2>$null
                        }
                    '''
                }
            }
        }
    }

    post {
        failure {
            echo 'Pipeline FAILED. Check console log for details.'
        }
    }
}
