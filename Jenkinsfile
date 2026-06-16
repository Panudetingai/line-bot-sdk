// ปรับปรุงสำหรับรันภายในเครื่อง Windows ตัวหลักตัวเดียว (Single Node Setup)
// มัดรวมงานทั้งหมดมาทำบน Agent ตัวเดียว ไม่ต้องกระจายเครื่องรัน
//
// ข้อกำหนดใน Jenkins Credentials:
//   - ต้องสร้าง 'Secret text' ตั้ง ID ว่า: npm-token

pipeline {
    agent any // สั่งรันบนเครื่องคอมพิวเตอร์ Windows หลักของคุณทันที ไม่ต้องตามหาเครื่องอื่น

    environment {
        APP_NAME = 'line-bot-sdk-rs'
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
        // === ด่านที่ 1: ตรวจสอบความถูกต้องของโค้ด ===
        stage('Lint') {
            steps {
                checkout scm
                // ใช้ bat สำหรับทำงานบน Windows Terminal
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

        // === ด่านที่ 2: คอมไพล์โค้ดเป็นไฟล์ระบบ .node ของ Windows ===
        stage('Build native bindings (Windows)') {
            steps {
                checkout scm
                bat '''
                    corepack enable
                    yarn install --immutable
                    rustup target add x86_64-pc-windows-msvc
                    yarn build -- --target x86_64-pc-windows-msvc
                    dir *.node
                '''
                // ฝากไฟล์ที่คอมไพล์เสร็จไว้ในระบบคลังชั่วคราว
                stash includes: '*.node', name: 'bindings-x86_64-pc-windows-msvc'
            }
        }

        // === ด่านที่ 3: ทดสอบการทำงานบนเครื่อง Windows ===
        stage('Test On Windows') {
            steps {
                checkout scm
                bat 'corepack enable && yarn install --immutable'
                // ดึงไฟล์ที่คอมไพล์ไว้จากสเตจที่แล้วออกมาทดสอบ
                unstash 'bindings-x86_64-pc-windows-msvc'
                bat 'yarn test'
            }
        }

        // === ด่านที่ 4: มัดรวมชิ้นงานและส่งขึ้น NPM Registry ===
        stage('Publish to npm') {
            when {
                // ระบบจะยอมทำด่านนี้ก็ต่อเมื่อคุณ Push Git Tag เช่น v1.0.1 ขึ้นไปบน GitHub เท่านั้น
                expression { env.TAG_NAME ==~ /^v.*/ || env.GIT_BRANCH ==~ /^v.*/ || env.BRANCH_NAME ==~ /^v.*/ }
            }
            steps {
                checkout scm

                // เตรียมโฟลเดอร์ปลายทาง
                bat '''
                    corepack enable
                    yarn install --immutable
                    yarn napi create-npm-dirs
                '''

                // ดึงไฟล์ Windows .node ออกมาจัดเรียงเข้าโฟลเดอร์เตรียมแพ็ก
                script {
                    unstash "bindings-x86_64-pc-windows-msvc"
                    // ใช้คำสั่งสคริปต์ Windows ในการสร้างโฟลเดอร์และย้ายไฟล์
                    bat """
                        if not exist "artifacts\\bindings-x86_64-pc-windows-msvc" mkdir "artifacts\\bindings-x86_64-pc-windows-msvc"
                        move *.node artifacts\\bindings-x86_64-pc-windows-msvc\\
                    """
                }

                // สั่งมัดรวมโครงสร้างชิ้นงาน JS
                bat '''
                    yarn artifacts
                    yarn build:js
                '''

                // เรียกใช้ Token ที่เราฝากไว้ในระบบ Jenkins Credentials (ID: npm-token)
                withCredentials([string(credentialsId: 'npm-token', variable: 'NPM_TOKEN')]) {
                    // สร้างคอนฟิกสำหรับล็อกอิน NPM บนระบบ Windows ชั่วคราวแล้วกดปล่อยของ
                    bat """
                        npm config set provenance true
                        echo //registry.npmjs.org/:_authToken=%NPM_TOKEN% > %USERPROFILE%\\.npmrc
                        npm publish --access public
                    """
                }
            }
        }
    }

    post {
        always {
            // สั่งทำความสะอาดลบรหัสผ่านออกจากเครื่องคอมพิวเตอร์หลังทำงานเสร็จเพื่อความปลอดภัย
            bat 'if exist "%USERPROFILE%\\.npmrc" del "%USERPROFILE%\\.npmrc"'
        }
        success {
            echo 'Pipeline completed successfully! แพ็กเกจถูกส่งขึ้น NPM เรียบร้อยแล้ว'
        }
        failure {
            echo 'Pipeline failed. ลองตรวจสอบข้อผิดพลาดด้านบนดูอีกครั้งครับ'
        }
    }
}