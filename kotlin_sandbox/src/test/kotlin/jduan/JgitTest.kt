package jduan

import org.junit.Test
import com.jcraft.jsch.SftpException
import com.jcraft.jsch.JSchException
import com.jcraft.jsch.JSch

class JgitTest {
    @Test fun f() {
        val jsch = JSch()

        try {
            val keypath = "/Users/jingjing_duan/.ssh/id_rsa2"
            println("Loading private key: ${keypath}")
            // If jsch fails to load the private key, it would throw an exception!
            jsch.addIdentity(keypath)
        } catch (e: JSchException) {
            e.printStackTrace()
        } catch (e: SftpException) {
            e.printStackTrace()
        }
    }
}
