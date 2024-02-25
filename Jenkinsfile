pipeline {
    agent {
        docker {
            image 'mysql' // Specify your Docker image and tag
            args '-u root'
        }
    }
    environment {
        BRANCH_NAME = ''
        SHORT_COMMIT = ''
        GIT_REPO_NAME = ''
        BUILDVERSION = ''
        SERVER_IP = 'io.github.oengajohn.com'
        SERVER_IPCREDS = credentials('uat-test-creds')
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
            }
        }
    }
}
