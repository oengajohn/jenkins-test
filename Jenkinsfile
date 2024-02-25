pipeline {
    agent {
        docker {
            image 'mysql' // Specify your Docker image and tag
            args '-u root --privileged -v /var/run/docker.sock:/var/run/docker.sock'
        }
    }
    environment {
        SERVER_IP = 'io.github.oengajohn.com'
        SERVER_IPCREDS = credentials('uat-test-creds')
    }
    stages {
        stage('Clone Repository') {
            steps {
                // Use the specified Git version
                git 'https://github.com/oengajohn/jenkins-test.git'
                echo "Done installation"
            }
        }
    }
}
