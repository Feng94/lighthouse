pipeline {
    agent { docker { image 'rust:latest' } }
    stages {
        stage('build') {
            steps {
                sh 'cargo build'
            }
        }
    }
}
