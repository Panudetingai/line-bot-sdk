// Jenkins pipeline for line-bot-sdk-rs (NAPI-RS multi-platform build + npm publish)
//
// Required Jenkins agents (labels):
//   - linux-x64   : Ubuntu agent with Node 22+, Rust, Yarn
//   - windows-x64 : Windows agent with Node 22+, Rust, Yarn
//   - macos       : macOS agent with Node 22+, Rust, Yarn (Intel and/or Apple Silicon)
//
// Required Jenkins credentials:
//   - npm-token (Secret text) : NPM access token with publish permission
//
// Trigger: push git tag matching v* (e.g. v1.0.1)

pipeline {
  agent none

  environment {
    APP_NAME = 'line-bot-sdk-rs'
    DEBUG = 'napi:*'
    CARGO_INCREMENTAL = '1'
    MACOSX_DEPLOYMENT_TARGET = '10.13'
    HUSKY = '0'
    CI = 'true'
  }

  options {
    disableConcurrentBuilds()
    timeout(time: 60, unit: 'MINUTES')
    buildDiscarder(logRotator(numToKeepStr: '20'))
  }

  stages {
    stage('Lint') {
      agent { label 'linux-x64' }
      steps {
        checkout scm
        sh '''
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

    stage('Build native bindings') {
      parallel {
        stage('Linux x64') {
          agent { label 'linux-x64' }
          steps {
            checkout scm
            sh '''
              corepack enable
              yarn install --immutable
              rustup target add x86_64-unknown-linux-gnu
              yarn build -- --target x86_64-unknown-linux-gnu
              ls -la *.node
            '''
            stash includes: '*.node', name: 'bindings-x86_64-unknown-linux-gnu'
          }
        }

        stage('Windows x64') {
          agent { label 'windows-x64' }
          steps {
            checkout scm
            bat '''
              corepack enable
              yarn install --immutable
              rustup target add x86_64-pc-windows-msvc
              yarn build -- --target x86_64-pc-windows-msvc
              dir *.node
            '''
            stash includes: '*.node', name: 'bindings-x86_64-pc-windows-msvc'
          }
        }

        stage('macOS x64') {
          agent { label 'macos' }
          steps {
            checkout scm
            sh '''
              corepack enable
              yarn install --immutable
              rustup target add x86_64-apple-darwin
              yarn build -- --target x86_64-apple-darwin
              ls -la *.node
            '''
            stash includes: '*.node', name: 'bindings-x86_64-apple-darwin'
          }
        }

        stage('macOS arm64') {
          agent { label 'macos' }
          steps {
            checkout scm
            sh '''
              corepack enable
              yarn install --immutable
              rustup target add aarch64-apple-darwin
              yarn build -- --target aarch64-apple-darwin
              ls -la *.node
            '''
            stash includes: '*.node', name: 'bindings-aarch64-apple-darwin'
          }
        }
      }
    }

    stage('Test') {
      parallel {
        stage('Test Linux') {
          agent { label 'linux-x64' }
          steps {
            checkout scm
            sh 'corepack enable && yarn install --immutable'
            unstash 'bindings-x86_64-unknown-linux-gnu'
            sh 'yarn test'
          }
        }

        stage('Test Windows') {
          agent { label 'windows-x64' }
          steps {
            checkout scm
            bat 'corepack enable && yarn install --immutable'
            unstash 'bindings-x86_64-pc-windows-msvc'
            bat 'yarn test'
          }
        }

        stage('Test macOS arm64') {
          agent { label 'macos' }
          steps {
            checkout scm
            sh 'corepack enable && yarn install --immutable'
            unstash 'bindings-aarch64-apple-darwin'
            sh 'yarn test'
          }
        }
      }
    }

    stage('Publish to npm') {
      when {
        expression { env.TAG_NAME ==~ /^v.*/ || env.GIT_BRANCH ==~ /^v.*/ || env.BRANCH_NAME ==~ /^v.*/ }
      }
      agent { label 'linux-x64' }
      steps {
        checkout scm

        sh '''
          corepack enable
          yarn install --immutable
          yarn napi create-npm-dirs
          rm -rf artifacts && mkdir -p artifacts
        '''

        script {
          def targets = [
            'x86_64-unknown-linux-gnu',
            'x86_64-pc-windows-msvc',
            'x86_64-apple-darwin',
            'aarch64-apple-darwin',
          ]
          for (target in targets) {
            unstash "bindings-${target}"
            sh """
              mkdir -p artifacts/bindings-${target}
              mv *.node artifacts/bindings-${target}/
            """
          }
        }

        sh '''
          ls -R artifacts/
          yarn artifacts
          ls -R npm/
          yarn build:js
        '''

        withCredentials([string(credentialsId: 'npm-token', variable: 'NPM_TOKEN')]) {
          sh '''
            npm config set provenance true
            echo "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" >> ~/.npmrc
            npm publish --access public
          '''
        }
      }
    }
  }

  post {
    success {
      echo 'Pipeline completed successfully.'
    }
    failure {
      echo 'Pipeline failed. Check logs above.'
    }
  }
}
