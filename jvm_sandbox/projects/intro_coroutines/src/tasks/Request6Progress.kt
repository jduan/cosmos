package tasks

import contributors.GitHubService
import contributors.RequestData
import contributors.User
import contributors.logRepos
import contributors.logUsers
import java.util.concurrent.atomic.AtomicInteger

suspend fun loadContributorsProgress(
    service: GitHubService,
    req: RequestData,
    updateResults: suspend (List<User>, completed: Boolean) -> Unit
) {
    val repos = service
        .getOrgRepos(req.org)
        .also { logRepos(req, it) }
        .bodyList()

    val users = mutableListOf<User>()
    val count = AtomicInteger()
    // We iterate through all the repos sequentially here because a new request is sent only
    // when the previous result is received.
    return repos.forEach() { repo ->
        val batch = service.getRepoContributors(req.org, repo.name)
            .also { logUsers(repo, it) }
            .bodyList()
        users.addAll(batch)
        updateResults(users.aggregate(), count.incrementAndGet() == repos.size)
    }
}
