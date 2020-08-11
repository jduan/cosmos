package tasks

import contributors.GitHubService
import contributors.RequestData
import contributors.User
import contributors.log
import contributors.logRepos
import contributors.logUsers
import kotlinx.coroutines.async
import kotlinx.coroutines.awaitAll
import kotlinx.coroutines.coroutineScope

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
            service.getRepoContributors(req.org, repo.name)
                .also { logUsers(repo, it) }
                .bodyList()
        }
    }

    deferred.awaitAll().flatten().aggregate()
}
