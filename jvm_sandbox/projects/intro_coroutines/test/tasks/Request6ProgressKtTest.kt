package tasks

import contributors.MockGithubService
import contributors.progressResults
import contributors.testRequestData
import kotlinx.coroutines.test.runBlockingTest
import org.junit.Assert
import org.junit.Test

class Request6ProgressKtTest {
    @Test
    fun testProgress() = runBlockingTest {
        val startTime = currentTime
        var index = 0
        loadContributorsProgress(MockGithubService, testRequestData) {
            users, _ ->
            val expected = progressResults[index++]
            val time = currentTime - startTime
            Assert.assertTrue("Expected intermediate result after virtual ${expected.timeFromStart} ms:",
                time in expected.timeFromStart..(expected.timeFromStart + 500)
            )
            Assert.assertEquals("Wrong intermediate result after $time:", expected.users, users)
        }
    }
}
