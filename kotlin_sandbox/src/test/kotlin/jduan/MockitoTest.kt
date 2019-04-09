package jduan

import com.nhaarman.mockitokotlin2.mock
import com.nhaarman.mockitokotlin2.verify
import com.nhaarman.mockitokotlin2.whenever
import junit.framework.TestCase
import junit.framework.TestCase.assertEquals
import org.junit.Test

interface BookService {
  fun inStock(bookId: Int): Boolean
  fun lend(bookId: Int, memberId: Int)
}

class LendBookManager(val bookService:BookService) {
  fun checkout(bookId: Int, memberId: Int) {
    if(bookService.inStock(bookId)) {
      bookService.lend(bookId, memberId)
    } else {
      throw IllegalStateException("Book is not available")
    }
  }
}

class MockitoTest {
  @Test
  fun `demo how to use mockito-kotlin`() {
    val mockBookService : BookService = mock()
    whenever(mockBookService.inStock(100)).thenReturn(true)
    val manager = LendBookManager(mockBookService)
    manager.checkout(100, 1)
    verify(mockBookService).lend(100, 1)
  }
}
