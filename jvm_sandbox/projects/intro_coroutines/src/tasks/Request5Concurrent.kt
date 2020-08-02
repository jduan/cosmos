package tasks

import contributors.*
import kotlinx.coroutines.*

suspend fun loadContributorsConcurrent(service: GitHubService, req: RequestData): List<User> = coroutineScope {
     val repos = service
        .getOrgRepos(req.org)
        .also { logRepos(req, it) }
        .bodyList()

    // We iterate through all the repos and launch a separate coroutine for each repo.
    // All the coroutines run in parallel.
    val deferred = repos.map { repo ->
        async {
            log("start loading for ${repo.name}")
            // delay here is needed so we have time to cancel the coroutines
            delay(3000)
            service.getRepoContributors(req.org, repo.name)
                .also { logUsers(repo, it) }
                .bodyList()
        }
    }

    deferred.awaitAll().flatten().aggregate()
}
