package tasks

import contributors.MockGithubService
import contributors.progressResults
import contributors.testRequestData
import kotlinx.coroutines.runBlocking
import org.junit.Assert
import org.junit.Test

class Request6ProgressKtTest {
    @Test
    fun testProgress() = runBlocking {
        val startTime = System.currentTimeMillis()
        var index = 0
        loadContributorsProgress(MockGithubService, testRequestData) {
            users, _ ->
            val expected = progressResults[index++]
            val time = System.currentTimeMillis() - startTime
            Assert.assertTrue("Expected intermediate result after virtual ${expected.timeFromStart} ms:",
                time in expected.timeFromStart..(expected.timeFromStart + 500)
            )
            Assert.assertEquals("Wrong intermediate result after $time:", expected.users, users)
        }
    }
}
