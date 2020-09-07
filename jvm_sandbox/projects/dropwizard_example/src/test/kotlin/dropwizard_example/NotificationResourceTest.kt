package dropwizard_example

import org.junit.Test
import org.mockito.Mockito.mock
import org.mockito.Mockito.verify

class NotificationResourceTest {
    private val store = mock(NotificationStore::class.java)
    private val resource = NotificationResource("Hello, %s!", "Stranger", store)

    @Test
    fun testAdd() {
        val notification = Notification("jack", "john", "how are you?")
        resource.add("David", notification)

        verify(store).create(notification)
    }
}
