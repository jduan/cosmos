package jduan.github

import org.kohsuke.github.GitHub

// To add a comment, you need to use the "issue" api instead of "pull request" api
fun addComment() {
    // You need to set an oauth2 token via env var
    val token = System.getenv("GITHUB_TOKEN")
    val github = GitHub.connectToEnterpriseWithOAuth(
        "https://git.musta.ch/api/v3", "jingjing_duan", token
    )

    val repo = github.getRepository("airbnb/dummy")
//    val pr = repo.getPullRequest(890)
    val issue = repo.getIssue(890)
    val response = issue.comment("hello")
    println(response)
}

fun createPullRequest() {
    // You need to set an oauth2 token via env var
    val token = System.getenv("GITHUB_TOKEN")
    val github = GitHub.connectToEnterpriseWithOAuth(
        "https://git.musta.ch/api/v3", "jingjing_duan", token
    )

    val repo = github.getRepository("airbnb/dummy")
    val title = "Test only 1"
    val head = "jduan--test11"
    val base = "master"
    val body = "Please ignore"
    val response = repo.createPullRequest(title, head, base, body)
    println(response)
}
