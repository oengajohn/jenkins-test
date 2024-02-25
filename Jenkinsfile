pipeline {
    agent {
         docker {
            image 'rust:latest'
            args '-u root --privileged -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }
    environment {
        BRANCH_NAME = ''
        SHORT_COMMIT = ''
        GIT_REPO_NAME = ''
        BUILDVERSION = ''
        SERVER_IP = 'io.github.oengajohn.com'
    }
    stages {
        stage('Clone Repository') {
            steps {
                script {
                    // Clone the repository
                    git 'https://github.com/oengajohn/jenkins-test.git'
                    // Set environment variables
                    BRANCH_NAME = env.BRANCH_NAME
                    SHORT_COMMIT = sh(script: 'git rev-parse --short HEAD', returnStdout: true).trim()
                    GIT_REPO_NAME = sh(script: 'basename `git rev-parse --show-toplevel`', returnStdout: true).trim()
                    BUILDVERSION = sh(script: 'git describe --always', returnStdout: true).trim()
                }
                // Use the specified Git version

                echo 'Done installation'
            }
        }
    }
}
