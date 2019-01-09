pipeline {
    agent { docker { dockerfile: true } }
    stages {
        stage('build') {
            steps {
                sh 'cargo build'
            }
        }
    }
}
