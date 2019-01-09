pipeline {
    agent { docker { image 'rust:1.31-stretch' } }
    stages {
        stage('build') {
            steps {
                sh 'rustc --version'
            }
        }
    }
}
