pipeline {
    agent {
        docker {
            image 'rust:alpine3.19' // Specify your Docker image and tag
            args '-u root --privileged -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }
    environment {
        SERVER_IP = 'io.github.oengajohn.com'
        SERVER_IPCREDS = credentials('uat-test-creds')
        GIT_VERSION = '2.44' // Specify your desired Git version
    }
    stages {
        stage('Clone Repository') {
            steps {
                // Use the specified Git version
                sh "apt-get update && apt-get install -y git=${GIT_VERSION}*"
                git 'https://github.com/oengajohn/jenkins-test.git'
                echo "Done installation"
            }
        }
    }
}
