package jduan

import com.jcraft.jsch.JSch
import com.jcraft.jsch.JSchException
import com.jcraft.jsch.SftpException
import org.junit.Test

class JSchTest {
    @Test
    fun f() {
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
