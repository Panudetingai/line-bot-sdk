// Single-node Windows Jenkins pipeline for line-bot-sdk-rs
//
// Jenkins Credential required:
//   - Secret text, ID: npm-token (npm access token with publish permission)
//
// Publish triggers (any one):
//   1. Push git tag v* (e.g. v1.0.1) and build that tag in Jenkins
//   2. Run "Build with Parameters" and check PUBLISH_TO_NPM
//
// Note: this pipeline only builds the Windows x64 binary.

pipeline {
    agent any

    parameters {
        booleanParam(
            name: 'PUBLISH_TO_NPM',
            defaultValue: true,
            description: 'Publish line-bot-sdk-rs to npm after tests pass (uncheck to skip)'
        )
    }

    environment {
        APP_NAME = 'line-bot-sdk-rs'
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
                script {
                    echo "BRANCH_NAME=${env.BRANCH_NAME ?: 'n/a'}"
                    echo "GIT_BRANCH=${env.GIT_BRANCH ?: 'n/a'}"
                    echo "TAG_NAME=${env.TAG_NAME ?: 'n/a'}"
                    echo "PUBLISH_TO_NPM=${params.PUBLISH_TO_NPM}"
                }
            }
        }

        stage('Lint') {
            steps {
                bat '''
                    corepack enable
                    node --version
                    yarn --version
                    rustc --version
                    yarn install --immutable
                    yarn lint
                    cargo fmt -- --check
                '''
            }
        }

        stage('Build native bindings (Windows)') {
            steps {
                bat '''
                    corepack enable
                    rustup target add x86_64-pc-windows-msvc
                    yarn build -- --target x86_64-pc-windows-msvc
                    if not exist "%NODE_FILE%" (
                        echo ERROR: Native binding not found: %NODE_FILE%
                        dir *.node 2>nul
                        exit /b 1
                    )
                    echo Built native binding:
                    dir "%NODE_FILE%"
                '''
            }
        }

        stage('Test On Windows') {
            steps {
                bat '''
                    corepack enable
                    yarn test
                '''
            }
        }

        stage('Publish to npm') {
            when {
                expression {
                    params.PUBLISH_TO_NPM ||
                    (env.TAG_NAME ?: '') ==~ /^v.*/ ||
                    (env.GIT_BRANCH ?: '') ==~ /^v.*/ ||
                    (env.BRANCH_NAME ?: '') ==~ /^v.*/
                }
            }
            steps {
                bat '''
                    corepack enable
                    yarn napi create-npm-dirs
                    if not exist "artifacts\\bindings-x86_64-pc-windows-msvc" mkdir "artifacts\\bindings-x86_64-pc-windows-msvc"
                    copy /Y "%NODE_FILE%" "artifacts\\bindings-x86_64-pc-windows-msvc\\%NODE_FILE%"
                    yarn artifacts
                    yarn build:js
                    dir npm\\win32-x64-msvc
                '''

                withCredentials([string(credentialsId: 'npm-token', variable: 'NPM_TOKEN')]) {
                    bat '''
                        call yarn napi prepublish -t npm --skip-optional-publish
                        npm config set provenance true
                        echo //registry.npmjs.org/:_authToken=%NPM_TOKEN%> "%USERPROFILE%\\.npmrc"
                        npm publish --access public --ignore-scripts
                        npm publish --access public "npm\\win32-x64-msvc"
                    '''
                }
            }
        }
    }

    post {
        always {
            bat 'if exist "%USERPROFILE%\\.npmrc" del "%USERPROFILE%\\.npmrc"'
        }
        success {
            script {
                def willPublish = params.PUBLISH_TO_NPM ||
                    (env.TAG_NAME ?: '') ==~ /^v.*/ ||
                    (env.GIT_BRANCH ?: '') ==~ /^v.*/ ||
                    (env.BRANCH_NAME ?: '') ==~ /^v.*/
                if (willPublish) {
                    echo 'Pipeline completed successfully. Package published to npm.'
                } else {
                    echo 'Pipeline completed successfully. Publish was skipped (no tag v* and PUBLISH_TO_NPM=false).'
                    echo 'To publish: run "Build with Parameters" and check PUBLISH_TO_NPM, or push tag v1.0.1'
                }
            }
        }
        failure {
            echo 'Pipeline failed. Check the stage logs above for details.'
        }
    }
}
