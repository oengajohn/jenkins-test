pipeline {
    agent {
        docker {
            image 'rust:latest'
            args '-u root --privileged -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }
    environment {
        SERVER_IP = 'io.github.oengajohn.com'
    }
    stages {
        stage('Clone Repository') {
            steps {
                script {
                    // Clone the repository
                    // Set environment variables
                    final scmVars = checkout(scm)
                    env.BRANCH_NAME = scmVars.GIT_BRANCH
                    env.SHORT_COMMIT = "${scmVars.GIT_COMMIT[0..7]}"
                    env.GIT_REPO_NAME = scmVars.GIT_URL.replaceFirst(/^.*\/([^\/]+?).git$/, '$1')
                }
                // Use the specified Git version

                echo 'Done installation'
            }
        }
        stage('Install Dependencies') {
            steps {
                echo "Branch -> ${env.BRANCH_NAME}"
                sh 'apk add --no-cache sqlite-dev musl-dev' // Fixed package name
                sh 'cargo install cross'
            }
        }
        stage('Create and Tar Working') {
            steps {
                script {
                    sh 'cross build --target x86_64-unknown-linux-gnu --release --workspace'
                    sh 'mkdir -p working'
                    sh 'cp ./target/x86_64-unknown-linux-gnu/release/agent ./working'
                    sh 'cp ./target/x86_64-unknown-linux-gnu/agent-cli ./working'
                    def version = sh(script: 'grep -oP \'version\\s*=\\s*"\\K[^\']\' cargo.toml', returnStdout: true).trim()
                    sh 'tar -cvf "${working}.tar" "${working}"' // Fixed variable interpolation
                }
            }
        }
    }
}
