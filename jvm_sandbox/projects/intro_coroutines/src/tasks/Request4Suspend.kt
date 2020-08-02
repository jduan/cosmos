package tasks

import contributors.*

// This looks surprisingly similar to the "blocking" version. It's readable and expresses
// exactly what we're trying to achieve.
suspend fun loadContributorsSuspend(service: GitHubService, req: RequestData): List<User> {
    val repos = service
        .getOrgRepos(req.org)
        .also { logRepos(req, it) }
        .bodyList()

    // We iterate through all the repos sequentially here because a new request is sent only
    // when the previous result is received.
    return repos.flatMap { repo ->
        service.getRepoContributors(req.org, repo.name)
            .also { logUsers(repo, it) }
            .bodyList()
    }.aggregate()
}
