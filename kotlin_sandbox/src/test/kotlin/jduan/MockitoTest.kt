package jduan

import com.nhaarman.mockitokotlin2.mock
import com.nhaarman.mockitokotlin2.verify
import com.nhaarman.mockitokotlin2.whenever
import org.junit.Assert.assertEquals
import org.junit.Test
import org.mockito.ArgumentMatchers.anyInt
import org.mockito.Mockito.`when`
import org.mockito.Mockito.atLeast
import org.mockito.Mockito.atLeastOnce
import org.mockito.Mockito.atMost
import org.mockito.Mockito.never
import org.mockito.Mockito.times

interface BookService {
    fun inStock(bookId: Int): Boolean
    fun lend(bookId: Int, memberId: Int)
}

class LendBookManager(private val bookService: BookService) {
    fun checkout(bookId: Int, memberId: Int) {
        if (bookService.inStock(bookId)) {
            bookService.lend(bookId, memberId)
        } else {
            throw IllegalStateException("Book is not available")
        }
    }
}

class MockitoTest {
    @Test
    fun `demo how to use mockito-kotlin`() {
        val mockBookService: BookService = mock()
        whenever(mockBookService.inStock(100)).thenReturn(true)

        val manager = LendBookManager(mockBookService)
        manager.checkout(100, 1)

        verify(mockBookService).lend(100, 1)
    }

    @Test
    fun `show how to stub method calls`() {
        val mockedList: List<String> = mock()
        whenever(mockedList.get(0)).thenReturn("first")
        whenever(mockedList.get(1)).thenThrow(RuntimeException())

        assertEquals("first", mockedList.get(0))
        // return "null" because "get(999)" wasn't stubbed
        assertEquals(null, mockedList.get(999))
        try {
            mockedList.get(1)
        } catch (ex: RuntimeException) {
            // expected
        }
    }

    @Test
    fun `argument matchers`() {
        val mockedList: List<String> = mock()
        whenever(mockedList.get(anyInt())).thenReturn("element")

        assertEquals("element", mockedList.get(999))
    }

    @Test
    fun `verify exact number of calls or at least X or never`() {
        val mockedList: MutableList<String> = mock()

        // using mock
        mockedList.add("once")
        mockedList.add("twice")
        mockedList.add("twice")
        mockedList.add("three times")
        mockedList.add("three times")
        mockedList.add("three times")

        // verify
        verify(mockedList).add("once")
        verify(mockedList, times(1)).add("once")
        verify(mockedList, times(2)).add("twice")
        verify(mockedList, times(3)).add("three times")
        verify(mockedList, never()).add("never happened")
        verify(mockedList, atMost(1)).add("once")
        verify(mockedList, atLeastOnce()).add("once")
        verify(mockedList, atLeast(2)).add("twice")
        verify(mockedList, atMost(3)).add("three times")
    }

    @Test
    fun `stub void methods with exceptions`() {
        val mockedList: MutableList<String> = mock()

        `when`(mockedList.clear()).thenThrow(RuntimeException())
        try {
            mockedList.clear()
        } catch (ex: RuntimeException) {
            // expected
        }
    }

    @Test
    fun `verification in order`() {
        val mockedList: MutableList<String> = mock()

        `when`(mockedList.clear()).thenThrow(RuntimeException())
        try {
            mockedList.clear()
        } catch (ex: RuntimeException) {
            // expected
        }
    }
}
