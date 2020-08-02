package tasks

import contributors.*
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch

suspend fun loadContributorsChannels(
    service: GitHubService,
    req: RequestData,
    updateResults: suspend (List<User>, completed: Boolean) -> Unit
) {
    coroutineScope {
        val repos = service
            .getOrgRepos(req.org)
            .also { logRepos(req, it) }
            .bodyList()

        val channel = Channel<List<User>>(repos.size)

        launch {
            val allUsers = mutableListOf<User>()
            repeat(repos.size) {
                val users = channel.receive()
                allUsers.addAll(users)
                updateResults(allUsers.aggregate(), it == repos.size - 1)
            }
        }

        // We iterate through all the repos and launch a separate coroutine for each repo.
        // All the coroutines run in parallel.
        repos.forEach() { repo ->
            launch {
                log("start loading for ${repo.name}")
                // delay here is needed so we have time to cancel the coroutines
                delay(3000)
                val users = service.getRepoContributors(req.org, repo.name)
                    .also { logUsers(repo, it) }
                    .bodyList()
                channel.send(users)
            }
        }
    }
}
