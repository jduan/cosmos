package tasks

import contributors.GitHubService
import contributors.RequestData
import contributors.User
import contributors.log
import contributors.logRepos
import contributors.logUsers
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.delay

suspend fun loadContributorsNotCancellable(service: GitHubService, req: RequestData): List<User> {
    val repos = service
        .getOrgRepos(req.org)
        .also { logRepos(req, it) }
        .bodyList()

    // We iterate through all the repos and launch a separate coroutine for each repo.
    // All the coroutines run in parallel.
    val deferred = repos.map { repo ->
        // coroutines launched in the global scope are harder to cancel.
        // You will need to have directly access to the job handle to cancel it.
        // Unlike "structured concurrency", cancelling a parent coroutine would cancel all of its children.
        GlobalScope.async {
            log("start loading for ${repo.name}")
            delay(3000)
            service.getRepoContributors(req.org, repo.name)
                .also { logUsers(repo, it) }
                .bodyList()
        }
    }

    return deferred.awaitAll().flatten().aggregate()
}
